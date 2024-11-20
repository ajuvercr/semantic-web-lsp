use std::collections::HashSet;

use bevy_ecs::prelude::*;

use lsp_core::{components::*, lang::SimpleCompletion, token::Token};
use lsp_types::CompletionItemKind;

use crate::TurtleLang;

pub fn turtle_lov_undefined_prefix_completion(
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
                    let sort_text = format!("2 {}", new_text);
                    let filter_text = new_text.clone();
                    if new_text != word.text {
                        Some(
                            SimpleCompletion::new(
                                CompletionItemKind::MODULE,
                                format!("{}", lov.name),
                                lsp_types::TextEdit {
                                    new_text,
                                    range: word.range.clone(),
                                },
                            )
                            .text_edit(lsp_types::TextEdit {
                                range: Range::new(start.clone(), start),
                                new_text: format!("@prefix {}: <{}>.\n", lov.name, lov.location),
                            })
                            .sort_text(sort_text)
                            .filter_text(filter_text),
                        )
                    } else {
                        None
                    }
                }),
        );
    }
}

