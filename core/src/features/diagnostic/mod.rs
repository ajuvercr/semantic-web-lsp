use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Range};

use bevy_ecs::system::Resource;
use chumsky::prelude::Simple;
use futures::{channel::mpsc, StreamExt};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionTextEdit, Diagnostic,
    DiagnosticSeverity, Documentation, InsertTextFormat, SemanticTokenType, TextDocumentItem,
    TextEdit, Url,
};
use ropey::Rope;

pub mod systems;

#[derive(Debug)]
pub struct SimpleDiagnostic {
    pub range: Range<usize>,
    pub msg: String,
    pub severity: Option<DiagnosticSeverity>,
}

impl SimpleDiagnostic {
    pub fn new(range: Range<usize>, msg: String) -> Self {
        Self {
            range,
            msg,
            severity: None,
        }
    }

    pub fn new_severity(range: Range<usize>, msg: String, severity: DiagnosticSeverity) -> Self {
        Self {
            range,
            msg,
            severity: Some(severity),
        }
    }
}

impl<T: Display + Eq + Hash> From<Simple<T>> for SimpleDiagnostic {
    fn from(e: Simple<T>) -> Self {
        let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason() {
            msg.clone()
        } else {
            format!(
                "{}{}, expected {}",
                if e.found().is_some() {
                    "Unexpected token"
                } else {
                    "Unexpected end of input"
                },
                if let Some(label) = e.label() {
                    format!(" while parsing {}", label)
                } else {
                    String::new()
                },
                if e.expected().len() == 0 {
                    "something else".to_string()
                } else {
                    e.expected()
                        .map(|expected| match expected {
                            Some(expected) => format!("'{}'", expected),
                            None => "end of input".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(" or ")
                },
            )
        };

        SimpleDiagnostic::new(e.span(), msg)
    }
}

impl<T: Display + Eq + Hash> From<(usize, Simple<T>)> for SimpleDiagnostic {
    fn from(this: (usize, Simple<T>)) -> Self {
        let (len, e) = this;
        let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason() {
            msg.clone()
        } else {
            format!(
                "{}{}, expected {}",
                if e.found().is_some() {
                    "Unexpected token"
                } else {
                    "Unexpected end of input"
                },
                if let Some(label) = e.label() {
                    format!(" while parsing {}", label)
                } else {
                    String::new()
                },
                if e.expected().len() == 0 {
                    "something else".to_string()
                } else {
                    e.expected()
                        .map(|expected| match expected {
                            Some(expected) => format!("'{}'", expected),
                            None => "end of input".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(" or ")
                },
            )
        };

        let range = (len - e.span().end)..(len - e.span().start);
        SimpleDiagnostic::new(range, msg)
    }
}

#[derive(Clone)]
pub struct DiagnosticSender {
    tx: mpsc::UnboundedSender<Vec<SimpleDiagnostic>>,
}

#[derive(Debug)]
pub struct DiagnosticItem {
    pub diagnostics: Vec<Diagnostic>,
    pub uri: Url,
    pub version: Option<i32>,
}
impl DiagnosticSender {
    pub fn push(&self, diagnostic: SimpleDiagnostic) -> Option<()> {
        let out = self.tx.unbounded_send(vec![diagnostic]).ok();
        out
    }

    pub fn push_all(&self, diagnostics: Vec<SimpleDiagnostic>) -> Option<()> {
        self.tx.unbounded_send(diagnostics).ok()
    }
}

#[derive(Resource)]
pub struct DiagnosticPublisher {
    tx: mpsc::UnboundedSender<DiagnosticItem>,
    diagnostics: HashMap<lsp_types::Url, Vec<(Diagnostic, &'static str)>>,
}

impl DiagnosticPublisher {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<DiagnosticItem>) {
        let (tx, rx) = mpsc::unbounded();
        (
            Self {
                tx,
                diagnostics: HashMap::new(),
            },
            rx,
        )
    }

    pub fn publish(
        &mut self,
        params: &TextDocumentItem,
        diagnostics: Vec<Diagnostic>,
        reason: &'static str,
    ) -> Option<()> {
        let items = self.diagnostics.entry(params.uri.clone()).or_default();
        items.retain(|(_, r)| *r != reason);
        items.extend(diagnostics.into_iter().map(|x| (x, reason)));
        let diagnostics: Vec<_> = items.iter().map(|(x, _)| x).cloned().collect();
        let uri = params.uri.clone();
        let version = Some(params.version);
        let item = DiagnosticItem {
            diagnostics,
            uri,
            version,
        };
        self.tx.unbounded_send(item).ok()
    }
}
