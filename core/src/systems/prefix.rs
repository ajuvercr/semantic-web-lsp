use std::collections::HashSet;

use lov::LocalPrefix;
use lsp_types::{CompletionItemKind, TextEdit};

use crate::{
    components::{Prefixes, TokenComponent},
    lang::SimpleCompletion,
    token::Token,
};

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
        lov::LOCAL_PREFIXES
            .iter()
            .map(|x| (x.name, x.location))
            .chain(HASHMAP.iter().cloned())
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
