use std::collections::HashSet;

use bevy_ecs::prelude::*;

use lsp_core::{components::*, lang::SimpleCompletion, token::Token, utils::offset_to_position};
use lsp_types::CompletionItemKind;

use crate::TurtleLang;

pub fn turtle_lov_prefix_completion(
    mut query: Query<(
        &TokenComponent,
        Option<&Element<TurtleLang>>,
        &mut CompletionRequest,
    )>,
) {
    for (word, turtle, mut req) in &mut query {
        match word.token.value() {
            Token::Invalid(_) => {}
            _ => continue,
        }

        let mut start = Position::new(0, 0);
        let mut defined = HashSet::new();
        if let Some(t) = turtle {
            if t.base.is_some() {
                start = Position::new(1, 0);
            }

            for pref in &t.prefixes {
                defined.insert(pref.prefix.value().as_str());
            }
        }

        use lsp_types::{Position, Range};
        req.extend(
            lov::LOCAL_PREFIXES
                .iter()
                .filter(|x| x.name.starts_with(&word.text))
                .filter(|x| !defined.contains(&x.name))
                .flat_map(|lov| {
                    let new_text = format!("{}:", lov.name);
                    if new_text != word.text {
                        let edits = vec![
                            lsp_types::TextEdit {
                                new_text,
                                range: word.range.clone(),
                            },
                            lsp_types::TextEdit {
                                range: Range::new(start.clone(), start),
                                new_text: format!("@prefix {}: <{}>.\n", lov.name, lov.location),
                            },
                        ];
                        Some(SimpleCompletion {
                            kind: CompletionItemKind::MODULE,
                            label: format!("{}", lov.name),
                            documentation: lov.title.to_string().into(),
                            sort_text: None,
                            filter_text: None,
                            edits,
                        })
                    } else {
                        None
                    }
                }),
        );
    }
}

pub fn turtle_prefix_completion(
    mut query: Query<(
        &TokenComponent,
        &Element<TurtleLang>,
        &mut CompletionRequest,
    )>,
) {
    for (word, turtle, mut req) in &mut query {
        println!("Current token {:?}", word);
        let st = &word.text;
        let pref = if let Some(idx) = st.find(':') {
            &st[..idx]
        } else {
            &st
        };

        let completions = turtle
            .0
            .prefixes
            .iter()
            .filter(|p| p.prefix.as_str().starts_with(pref))
            .flat_map(|x| {
                let url = x.value.expand(&turtle.0);
                let new_text = format!("{}:", x.prefix.as_str());
                if new_text != word.text {
                    let edits = vec![lsp_types::TextEdit {
                        new_text,
                        range: word.range.clone(),
                    }];
                    Some(SimpleCompletion {
                        kind: CompletionItemKind::MODULE,
                        label: format!("{}", x.prefix.as_str()),
                        documentation: url,
                        sort_text: None,
                        filter_text: None,
                        edits,
                    })
                } else {
                    None
                }
            });

        req.0.extend(completions);
    }
}
