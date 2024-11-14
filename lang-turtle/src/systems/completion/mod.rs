mod prefix;
mod subject;

use lsp_core::{
    components::{CompletionRequest, RopeC, Tokens},
    utils::position_to_offset,
};
pub use prefix::turtle_prefix_completion;
pub use subject::subject_completion;

#[cfg(test)]
mod tests;

use bevy_ecs::prelude::*;
