use std::{collections::HashSet, ops::Deref};

use bevy_ecs::component::Component;
use lsp_types::{CompletionItemKind, TextEdit};

use crate::prelude::*;

const JSONLD: &'static str = include_str!("./jsonld.json");

lazy_static::lazy_static! {
    static ref HASHMAP: Vec<(&'static str, &'static str)> = {
        let m: Vec<_> = JSONLD.split('\n').flat_map(|x| { let mut s = x.split(' ');
            let first = s.next()?;
            let second = s.next()?;
            Some((first, second))
        }).collect();
        m
    };
}

/// One defined prefix, maps prefix to url
#[derive(Debug, Clone)]
pub struct Prefix {
    pub prefix: String,
    pub url: lsp_types::Url,
}

/// [`Component`] that containing defined prefixes and base URL.
///
/// [`lsp_core`](crate) uses [`Prefixes`] in different systems, for example
/// - to check for undefined prefixes diagnostics with
/// [`undefined_prefix`](crate::prelude::systems::undefined_prefix)
/// - derive linked documents [`DocumentLinks`] with
/// [`derive_prefix_links`](crate::prelude::systems::derive_prefix_links)
#[derive(Component, Debug)]
pub struct Prefixes(pub Vec<Prefix>, pub lsp_types::Url);
impl Deref for Prefixes {
    type Target = Vec<Prefix>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Prefixes {
    pub fn shorten(&self, value: &str) -> Option<String> {
        let try_shorten = |prefix: &Prefix| {
            let short = value.strip_prefix(prefix.url.as_str())?;
            Some(format!("{}:{}", prefix.prefix, short))
        };

        self.0.iter().flat_map(try_shorten).next()
    }

    pub fn expand(&self, token: &Token) -> Option<String> {
        match token {
            Token::PNameLN(pref, x) => {
                let pref = pref.as_ref().map(|x| x.as_str()).unwrap_or("");
                let prefix = self.0.iter().find(|x| &x.prefix == pref)?;
                Some(format!("{}{}", prefix.url, x))
            }
            Token::IRIRef(x) => {
                return self.1.join(&x).ok().map(|x| x.to_string());
            }
            _ => None,
        }
    }

    pub fn expand_json(&self, token: &Token) -> Option<String> {
        match token {
            Token::Str(pref, _) => {
                if let Some(x) = pref.find(':') {
                    let prefix = &pref[..x];
                    if let Some(exp) = self.0.iter().find(|x| &x.prefix == prefix) {
                        return Some(format!("{}{}", exp.url.as_str(), &pref[x + 1..]));
                    }
                } else {
                    if let Some(exp) = self.0.iter().find(|x| &x.prefix == pref) {
                        return Some(exp.url.as_str().to_string());
                    }
                }

                return Some(
                    self.1
                        .join(&pref)
                        .ok()
                        .map(|x| x.to_string())
                        .unwrap_or(pref.to_string()),
                );
            }
            _ => None,
        }
    }
}

pub fn prefix_completion_helper(
    word: &TokenComponent,
    prefixes: &Prefixes,
    completions: &mut Vec<SimpleCompletion>,
    mut extra_edits: impl FnMut(&str, &str) -> Option<Vec<TextEdit>>,
) {
    match word.token.value() {
        Token::Invalid(_) => {}
        _ => return,
    }

    let mut defined = HashSet::new();
    for p in prefixes.0.iter() {
        defined.insert(p.url.as_str());
    }

    completions.extend(
        HASHMAP
            .iter()
            .filter(|(name, _)| name.starts_with(&word.text))
            .filter(|(_, location)| !defined.contains(location))
            .flat_map(|(name, location)| {
                let new_text = format!("{}:", name);
                let sort_text = format!("2 {}", new_text);
                let filter_text = new_text.clone();
                if new_text != word.text {
                    let extra_edit = extra_edits(name, location)?;
                    let completion = SimpleCompletion::new(
                        CompletionItemKind::MODULE,
                        format!("{}", name),
                        lsp_types::TextEdit {
                            new_text,
                            range: word.range.clone(),
                        },
                    )
                    .sort_text(sort_text)
                    .filter_text(filter_text);

                    let completion = extra_edit
                        .into_iter()
                        .fold(completion, |completion: SimpleCompletion, edit| {
                            completion.text_edit(edit)
                        });
                    Some(completion)
                } else {
                    None
                }
            }),
    );
}
