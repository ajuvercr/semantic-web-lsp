use std::ops::Range;

pub mod backend;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Error {
    pub msg: String,
    pub span: Range<usize>,
}

#[cfg(feature = "bin")]
pub use tower_lsp::lsp_types;
