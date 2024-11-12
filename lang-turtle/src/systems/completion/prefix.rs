use bevy_ecs::prelude::*;

use lsp_core::{components::*, lang::SimpleCompletion, utils::lsp_range_to_range};
use lsp_types::CompletionItemKind;

use crate::TurtleLang;

pub fn turtle_prefix_completion(
    mut query: Query<(
        &CurrentWord,
        &Element<TurtleLang>,
        &RopeC,
        &mut CompletionRequest,
    )>,
) {
    for (word, turtle, rope, mut req) in &mut query {
        if let Some(r) = lsp_range_to_range(&word.0, &rope.0) {
            let st = rope.0.slice(r).to_string();
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
                .map(|x| {
                    let url = x.value.expand(&turtle.0);

                    let edits = vec![lsp_types::TextEdit {
                        new_text: format!("{}:", x.prefix.as_str()),
                        range: word.0,
                    }];
                    SimpleCompletion {
                        kind: CompletionItemKind::MODULE,
                        label: format!("{}", x.prefix.as_str()),
                        documentation: url,
                        sort_text: None,
                        filter_text: None,
                        edits,
                    }
                });

            req.0.extend(completions);
        }
    }
}
