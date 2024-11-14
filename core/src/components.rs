use crate::{
    lang::{Lang, SimpleCompletion},
    model::Spanned,
    triples::{MyQuad, MyTerm},
};
use bevy_ecs::{prelude::*, world::CommandQueue};
use derive_more::{AsMut, AsRef};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use lsp_types::{Position, SemanticToken};
use sophia_api::{prelude::Dataset, quad::Quad as _, term::matcher::TermMatcher};

#[derive(Component, AsRef, AsMut, Debug)]
pub struct Tokens(pub Vec<Spanned<crate::token::Token>>);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct Element<L: Lang>(pub Spanned<L::Element>);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct Wrapped<E>(pub E);
#[derive(Component, AsRef, AsMut, Debug)]
pub struct Errors<E>(pub Vec<E>);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct Source(pub String);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct RopeC(pub ropey::Rope);

#[derive(Component, Debug)]
pub struct Open;

#[derive(Component, Debug)]
pub struct Dirty;

#[derive(Component, AsRef, AsMut, Debug)]
pub struct Label(pub lsp_types::Url);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct HighlightRequest(pub Vec<SemanticToken>);

#[derive(Resource, AsRef, AsMut, Debug)]
pub struct CommandReceiver(pub UnboundedReceiver<CommandQueue>);

#[derive(Resource, AsRef, AsMut, Debug, Clone)]
pub struct CommandSender(pub UnboundedSender<CommandQueue>);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct PositionComponent(pub Position);

#[derive(Component, Debug)]
pub struct TokenComponent {
    pub token: Spanned<crate::token::Token>,
    pub range: lsp_types::Range,
    pub text: String,
}

#[derive(Component, AsRef, AsMut, Debug)]
pub struct CompletionRequest(pub Vec<SimpleCompletion>);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct FormatRequest(pub Option<Vec<lsp_types::TextEdit>>);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct Triples(pub Vec<MyQuad<'static>>);

#[derive(Component, AsRef, AsMut, Debug)]
pub struct DefinedProperties(pub Vec<DefinedProperties>);

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
