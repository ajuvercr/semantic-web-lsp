use crate::components::CommandReceiver;
use bevy_ecs::prelude::*;

mod diagnostics;
pub use diagnostics::publish_diagnostics;
mod semantics;
pub use semantics::{semantic_tokens_system, SemanticTokensSchedule};


/// This system queries for entities that have our Task<Transform> component. It polls the
/// tasks to see if they're complete. If the task is complete it takes the result, adds a
/// new [`Mesh3d`] and [`MeshMaterial3d`] to the entity using the result from the task's work, and
/// removes the task component from the entity.
pub fn handle_tasks(mut commands: Commands, mut receiver: ResMut<CommandReceiver>) {
    while let Ok(Some(mut com)) = receiver.0.try_next() {
        commands.append(&mut com);
    }
}
