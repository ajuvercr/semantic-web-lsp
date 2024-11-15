use std::{fmt::Display, hash::Hash, ops::Range};

use crate::model::Spanned;
use bevy_ecs::system::Resource;
use chumsky::prelude::Simple;
use futures::{channel::mpsc, StreamExt};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionTextEdit, Diagnostic, DiagnosticSeverity,
    Documentation, InsertTextFormat, SemanticTokenType, TextDocumentItem, TextEdit, Url,
};
use ropey::Rope;

use crate::{client::Client, utils::offset_to_position};

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

pub fn head() -> lsp_types::Range {
    let start = lsp_types::Position {
        line: 0,
        character: 0,
    };
    lsp_types::Range {
        end: start.clone(),
        start,
    }
}

#[derive(Debug)]
pub struct SimpleCompletion {
    pub kind: CompletionItemKind,
    pub label: String,
    pub _documentation: Option<String>,
    pub _sort_text: Option<String>,
    pub _filter_text: Option<String>,
    pub edits: Vec<TextEdit>,
    pub _commit_char: Option<String>,
}
impl SimpleCompletion {
    pub fn new(kind: CompletionItemKind, label: String, edit: TextEdit) -> Self {
        Self {
            kind,
            label,
            edits: vec![edit],
            _documentation: None,
            _sort_text: None,
            _filter_text: None,
            _commit_char: None,
        }
    }

    pub fn text_edit(mut self, edit: TextEdit) -> Self {
        self.edits.push(edit);
        self
    }

    pub fn documentation(mut self, documentation: impl Into<String>) -> Self {
        self._documentation = Some(documentation.into());
        self
    }

    pub fn m_documentation<S: Into<String>>(mut self, documentation: Option<S>) -> Self {
        self._documentation = documentation.map(|x| x.into());
        self
    }

    pub fn sort_text(mut self, sort_text: impl Into<String>) -> Self {
        self._sort_text = Some(sort_text.into());
        self
    }

    pub fn filter_text(mut self, filter_text: impl Into<String>) -> Self {
        self._filter_text = Some(filter_text.into());
        self
    }

    pub fn commit_char(mut self, commit_char: impl Into<String>) -> Self {
        self._commit_char = Some(commit_char.into());
        self
    }
}

impl Into<CompletionItem> for SimpleCompletion {
    fn into(self) -> CompletionItem {
        let SimpleCompletion {
            _filter_text: filter_text,
            _sort_text: sort_text,
            label,
            _documentation: documentation,
            kind,
            edits,
            _commit_char: commit_char,
        } = self;

        let text_edit = edits
            .iter()
            .next()
            .map(|x| CompletionTextEdit::Edit(x.clone()));

        let additional_text_edits = edits.into_iter().skip(1).collect();

        CompletionItem {
            label,
            kind: Some(kind),
            sort_text,
            insert_text_format: (kind == CompletionItemKind::SNIPPET)
                .then_some(InsertTextFormat::SNIPPET),
            filter_text,
            documentation: documentation.map(|st| Documentation::String(st)),
            text_edit,
            additional_text_edits: Some(additional_text_edits),
            commit_characters: commit_char.map(|x| vec![String::from(x)]),
            ..Default::default()
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

pub trait Token: Sized {
    fn token(&self) -> Option<SemanticTokenType>;

    fn span_tokens(Spanned(this, span): &Spanned<Self>) -> Vec<(SemanticTokenType, Range<usize>)> {
        if let Some(x) = this.token() {
            vec![(x, span.clone())]
        } else {
            Vec::new()
        }
    }
}

pub trait Lang: Sized + 'static {
    /// Type of tokens after tokenization
    type Token: PartialEq + Hash + Clone + Send + Sync + Token;
    type TokenError: Into<SimpleDiagnostic> + Send + Sync + std::fmt::Debug;

    /// Type of Element inside a ParentingSystem
    type Element: Send + Sync;
    type ElementError: Into<SimpleDiagnostic> + Send + Sync + std::fmt::Debug;

    const CODE_ACTION: bool;
    const HOVER: bool;
    const LANG: &'static str;
    const TRIGGERS: &'static [&'static str];
    const LEGEND_TYPES: &'static [SemanticTokenType];
    const PATTERN: Option<&'static str>;
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
pub struct OtherPublisher {
    tx: mpsc::UnboundedSender<DiagnosticItem>,
}

impl OtherPublisher {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<DiagnosticItem>) {
        let (tx, rx) = mpsc::unbounded();
        (Self { tx }, rx)
    }

    pub fn publish(
        &mut self,
        params: &TextDocumentItem,
        diagnostics: Vec<Diagnostic>,
    ) -> Option<()> {
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

pub struct Publisher<C: Client + Send + Sync + 'static> {
    version: i32,
    uri: Url,
    client: C,
    diagnostics: Vec<Diagnostic>,
    rope: Rope,
    rx: mpsc::UnboundedReceiver<Vec<SimpleDiagnostic>>,
}

impl<C: Client + Send + Sync + 'static> Publisher<C> {
    pub fn new(uri: Url, version: i32, client: C, rope: Rope) -> (Self, DiagnosticSender) {
        let (tx, rx) = mpsc::unbounded();
        (
            Self {
                version,
                uri,
                client,
                diagnostics: Vec::new(),
                rx,
                rope,
            },
            DiagnosticSender { tx },
        )
    }

    pub async fn spawn(mut self) {
        loop {
            if let Some(x) = self.rx.next().await {
                self.diagnostics.extend(x.into_iter().flat_map(|item| {
                    let (span, message) = (item.range, item.msg);
                    let start_position = offset_to_position(span.start, &self.rope)?;
                    let end_position = offset_to_position(span.end, &self.rope)?;
                    Some(Diagnostic {
                        range: lsp_types::Range::new(start_position, end_position),
                        message,
                        severity: item.severity,
                        ..Default::default()
                    })
                }));

                self.client
                    .publish_diagnostics(
                        self.uri.clone(),
                        self.diagnostics.clone(),
                        Some(self.version),
                    )
                    .await;
            } else {
                return;
            }
        }
    }
}
