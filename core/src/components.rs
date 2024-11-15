use std::collections::HashMap;

use crate::{
    lang::{Lang, SimpleCompletion},
    model::Spanned,
    triples::{MyQuad, MyTerm},
};
use bevy_ecs::{prelude::*, world::CommandQueue};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use lsp_types::{Position, SemanticToken};
use sophia_api::{prelude::Dataset, quad::Quad as _, term::matcher::TermMatcher};

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Tokens(pub Vec<Spanned<crate::token::Token>>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Element<L: Lang>(pub Spanned<L::Element>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Wrapped<E>(pub E);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Errors<E>(pub Vec<E>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Source(pub String);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct RopeC(pub ropey::Rope);

#[derive(Component, Debug)]
pub struct Open;

#[derive(Component, Debug)]
pub struct Dirty;

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Label(pub lsp_types::Url);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct HighlightRequest(pub Vec<SemanticToken>);

#[derive(Resource, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct CommandReceiver(pub UnboundedReceiver<CommandQueue>);

#[derive(Resource, AsRef, Deref, AsMut, DerefMut, Debug, Clone)]
pub struct CommandSender(pub UnboundedSender<CommandQueue>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug, Clone)]
pub struct DocumentLinks(pub Vec<(lsp_types::Url, &'static str)>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct PositionComponent(pub Position);

#[derive(Component, Debug)]
pub struct TokenComponent {
    pub token: Spanned<crate::token::Token>,
    pub range: lsp_types::Range,
    pub text: String,
}

#[derive(Debug, PartialEq)]
pub enum TripleTarget {
    Subject,
    Predicate,
    Object,
    Graph,
}
#[derive(Component, Debug)]
pub struct TripleComponent {
    pub triple: MyQuad<'static>,
    pub target: TripleTarget,
}

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct CompletionRequest(pub Vec<SimpleCompletion>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct FormatRequest(pub Option<Vec<lsp_types::TextEdit>>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct DefinedProperties(pub Vec<DefinedProperties>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Triples(pub Vec<MyQuad<'static>>);

impl Triples {
    pub fn object<'s, S, P>(&'s self, subj: S, pred: P) -> Option<&MyTerm<'_>>
    where
        S: TermMatcher + 's,
        P: TermMatcher + 's,
    {
        self.0
            .quads_matching(
                subj,
                pred,
                sophia_api::prelude::Any,
                sophia_api::prelude::Any,
            )
            .flatten()
            .next()
            .map(|x| x.o())
    }
}
