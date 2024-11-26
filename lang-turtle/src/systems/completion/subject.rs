use bevy_ecs::prelude::*;
use lsp_core::{components::*, lang::SimpleCompletion, token::Token};
use lsp_types::CompletionItemKind;
use tracing::debug;

use crate::{NamedNode, TurtleLang};

pub fn subject_completion(
    mut query: Query<(
        &TokenComponent,
        &Element<TurtleLang>,
        &mut CompletionRequest,
    )>,
    triples: Query<(&Triples, &Label), With<Open>>,
) {
    for (word, turtle, mut req) in &mut query {
        let m_expaned = match word.token.value() {
            Token::PNameLN(pref, value) => NamedNode::Prefixed {
                prefix: pref.clone().unwrap_or_default(),
                value: value.clone(),
            }
            .expand(turtle.0.value()),
            _ => continue,
        };
        let Some(expanded) = m_expaned else { continue };

        for (triples, label) in &triples {
            for triple in &triples.0 {
                debug!("Triple {} start with {}", triple.subject.as_str(), expanded);
                let subj = triple.subject.as_str();
                if subj.starts_with(&expanded) {
                    let new_text = turtle.0.shorten(subj).unwrap_or_else(|| String::from(subj));

                    if new_text != word.text {
                        req.push(
                            SimpleCompletion::new(
                                CompletionItemKind::MODULE,
                                subj.to_string(),
                                lsp_types::TextEdit {
                                    new_text,
                                    range: word.range.clone(),
                                },
                            )
                            .documentation(format!("Subject from {}", label.0)),
                        );
                    }
                }
            }
        }
    }
}
