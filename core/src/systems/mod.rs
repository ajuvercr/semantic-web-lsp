use crate::{
    components::{CommandReceiver, PositionComponent, RopeC, TokenComponent, Tokens, Triples},
    utils::{position_to_offset, range_to_range},
};
use bevy_ecs::prelude::*;

mod diagnostics;
pub use diagnostics::publish_diagnostics;
mod semantics;
pub use semantics::{semantic_tokens_system, SemanticTokensSchedule};
mod properties;
pub use properties::derive_classes;
use tracing::{debug, info};

pub fn handle_tasks(mut commands: Commands, mut receiver: ResMut<CommandReceiver>) {
    while let Ok(Some(mut com)) = receiver.0.try_next() {
        commands.append(&mut com);
    }
}

pub fn get_current_token(
    mut query: Query<(Entity, &Tokens, &PositionComponent, &RopeC)>,
    mut commands: Commands,
) {
    println!("Get current token!");
    for (entity, tokens, position, rope) in &mut query {
        let Some(offset) = position_to_offset(position.0, &rope.0) else {
            debug!("Couldn't transform to an offset");
            continue;
        };

        let Some(token) = tokens
            .0
            .iter()
            .filter(|x| x.span().contains(&offset))
            .min_by_key(|x| x.span().end - x.span().start)
        else {
            let closest = tokens.0.iter().min_by_key(|x| {
                let start = if offset > x.span().start {
                    offset - x.span().start
                } else {
                    x.span().start - offset
                };

                let end = if offset > x.span().end {
                    offset - x.span().end
                } else {
                    x.span().end - offset
                };

                if start > end {
                    end
                } else {
                    start
                }
            });
            debug!(
                "Failed to find a token, offset {} closest {:?}",
                offset, closest
            );
            continue;
        };

        let Some(range) = range_to_range(token.span(), &rope.0) else {
            debug!("Failed to transform span to range");
            continue;
        };

        info!("Get current found {:?} {:?}", token, range);
        let text = rope.0.slice(token.span().clone()).to_string();

        commands.entity(entity).insert(TokenComponent {
            token: token.clone(),
            range,
            text,
        });
    }
}

pub fn get_current_triple(query: Query<(&PositionComponent, &Triples, &RopeC)>) {
    for (position, triples, rope) in &query {
        let Some(offset) = position_to_offset(position.0, &rope.0) else {
            debug!("Couldn't transform to an offset");
            continue;
        };
        let current_triples: Vec<_> = triples
            .0
            .iter()
            .filter(|triple| triple.span.contains(&offset))
            .collect();
        info!("Current triples {:?} {:?}", current_triples, triples.0);
    }
}
