use bevy_ecs::prelude::*;

use lsp_core::{components::*, systems::prefix::prefix_completion_helper};

use crate::TurtleLang;

pub fn turtle_lov_undefined_prefix_completion(
    mut query: Query<(
        &TokenComponent,
        &Element<TurtleLang>,
        &Prefixes,
        &mut CompletionRequest,
    )>,
) {
    for (word, turtle, prefixes, mut req) in &mut query {
        let mut start = Position::new(0, 0);

        if turtle.base.is_some() {
            start = Position::new(1, 0);
        }

        use lsp_types::{Position, Range};
        prefix_completion_helper(word, prefixes, &mut req.0, |lov| {
            Some(vec![lsp_types::TextEdit {
                range: Range::new(start.clone(), start),
                new_text: format!("@prefix {}: <{}>.\n", lov.name, lov.location),
            }])
        });
    }
}
