use std::ops::Range;

pub mod backend;
pub mod client;
pub use client::TowerClient;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Error {
    pub msg: String,
    pub span: Range<usize>,
}
