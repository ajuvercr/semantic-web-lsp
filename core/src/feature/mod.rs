//! All features supported by the language server.
//! Features like completion ([`CompletionRequest`](`completion::CompletionRequest`)) and hover expose a component that collects items returned to the
//! client.
pub mod completion;
pub use completion::Label as CompletionLabel;
pub mod hover;
pub use hover::Label as HoverLabel;
pub mod parse;
pub use parse::Label as ParseLabel;
pub mod rename;
pub use rename::{PrepareRename as PrepareRenameLabel, Rename as RenameLabel};
pub mod diagnostics;
pub use diagnostics::Label as DiagnosticsLabel;
pub mod save;
pub use save::Label as SaveLabel;
pub mod inlay;
pub use inlay::Label as InlayLabel;
pub mod format;
pub use format::Label as FormatLabel;
pub mod semantic;
pub use semantic::Label as SemanticLabel;
pub mod references;
pub use references::Label as ReferencesLabel;
pub mod goto_implementation;
pub use goto_implementation::Label as GotoImplementationLabel;
pub mod goto_type;
pub use goto_type::Label as GotoTypeLabel;
