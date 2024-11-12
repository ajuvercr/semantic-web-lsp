use bevy_ecs::prelude::*;
use lsp_core::{components::*, lang::SimpleCompletion, utils::lsp_range_to_range};
use lsp_types::CompletionItemKind;

use crate::TurtleLang;

pub fn subject_completion(
    mut query: Query<(
        &CurrentWord,
        &Element<TurtleLang>,
        &RopeC,
        &mut CompletionRequest,
    )>,
    triples: Query<(&Triples, &Label)>,
) {
    for (word, turtle, rope, mut req) in &mut query {
        if let Some(r) = lsp_range_to_range(&word.0, &rope.0) {
            let st = rope.0.slice(r).to_string();

            for (triples, label) in &triples {
                for triple in &triples.0 {
                    let subj = triple.subject.as_str();
                    if subj.starts_with(&st) {
                        let new_text = turtle.0.shorten(subj).unwrap_or_else(|| String::from(subj));
                        let edits = vec![lsp_types::TextEdit {
                            new_text,
                            range: word.0,
                        }];
                        req.0.push(SimpleCompletion {
                            kind: CompletionItemKind::MODULE,
                            label: format!("{}", subj),
                            documentation: format!("Subject from {}", label.0).into(),
                            sort_text: None,
                            filter_text: None,
                            edits,
                        });
                    }
                }
            }
        }
    }
}
