use bevy_ecs::prelude::*;

use lsp_core::{components::*, lang::SimpleCompletion};
use lsp_types::CompletionItemKind;

use crate::TurtleLang;

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
