use lsp_core::lang::SimpleCompletion;
use lsp_types::Range;

use super::Turtle;

#[async_trait::async_trait]
pub trait CompletionProvider<Ctx> {
    async fn find_completions(&self, ctx: &Ctx, range: Range) -> Vec<SimpleCompletion>;
}

mod namespace;
pub use namespace::*;

mod shape;
pub use shape::*;
