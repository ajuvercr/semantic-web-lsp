use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Range};

use crate::{features::diagnostic::SimpleDiagnostic, model::Spanned};
use bevy_ecs::system::Resource;
use chumsky::prelude::Simple;
use futures::{channel::mpsc, StreamExt};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionTextEdit, Diagnostic,
    Documentation, InsertTextFormat, SemanticTokenType, TextDocumentItem, TextEdit, Url,
};
use ropey::Rope;

use crate::{client::Client, utils::offset_to_position};

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
    pub _label_details: Option<CompletionItemLabelDetails>,
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
            _label_details: None,
            _documentation: None,
            _sort_text: None,
            _filter_text: None,
            _commit_char: None,
        }
    }

    pub fn label_detail(mut self, detail: impl Into<String>) -> Self {
        if let Some(ref mut t) = self._label_details {
            t.detail = Some(detail.into());
        } else {
            self._label_details = Some(CompletionItemLabelDetails {
                detail: Some(detail.into()),
                description: None,
            });
        }

        self
    }

    pub fn label_description(mut self, description: impl Into<String>) -> Self {
        if let Some(ref mut t) = self._label_details {
            t.description = Some(description.into());
        } else {
            self._label_details = Some(CompletionItemLabelDetails {
                description: Some(description.into()),
                detail: None,
            });
        }

        self
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
            _label_details,
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
            label_details: _label_details,
            documentation: documentation.map(|st| Documentation::String(st)),
            text_edit,
            additional_text_edits: Some(additional_text_edits),
            commit_characters: commit_char.map(|x| vec![String::from(x)]),
            ..Default::default()
        }
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

pub trait Lang: 'static {
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

pub trait LangHelper: std::fmt::Debug {
    fn _get_relevant_text(&self, token: &Spanned<crate::token::Token>, rope: &Rope) -> String {
        rope.slice(token.span().clone()).to_string()
    }
    fn get_relevant_text(
        &self,
        token: &Spanned<crate::token::Token>,
        rope: &Rope,
    ) -> (String, std::ops::Range<usize>) {
        (self._get_relevant_text(token, rope), token.span().clone())
    }
    fn keyword(&self) -> &[&'static str];
}

