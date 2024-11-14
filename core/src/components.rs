use crate::{
    lang::{Lang, SimpleCompletion},
    model::Spanned,
    triples::{MyQuad, MyTerm},
};
use bevy_ecs::{prelude::*, world::CommandQueue};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use lsp_types::{Position, SemanticToken};
use sophia_api::{prelude::Dataset, quad::Quad as _, term::matcher::TermMatcher};

#[derive(Component)]
pub struct Tokens(pub Vec<Spanned<crate::token::Token>>);

#[derive(Component)]
pub struct Element<L: Lang>(pub Spanned<L::Element>);

#[derive(Component)]
pub struct Wrapped<E>(pub E);
#[derive(Component)]
pub struct Errors<E>(pub Vec<E>);

#[derive(Component)]
pub struct Source(pub String);

#[derive(Component)]
pub struct RopeC(pub ropey::Rope);

#[derive(Component)]
pub struct Open;

#[derive(Component)]
pub struct Dirty;

#[derive(Component)]
pub struct Label(pub lsp_types::Url);

#[derive(Component)]
pub struct HighlightRequest(pub Vec<SemanticToken>);

#[derive(Resource)]
pub struct CommandReceiver(pub UnboundedReceiver<CommandQueue>);

#[derive(Resource, Debug, Clone)]
pub struct CommandSender(pub UnboundedSender<CommandQueue>);

#[derive(Component)]
pub struct PositionComponent(pub Position);

#[derive(Component, Debug)]
pub struct TokenComponent {
    pub token: Spanned<crate::token::Token>,
    pub range:  lsp_types::Range,
    pub text: String,
}

#[derive(Component)]
pub struct CompletionRequest(pub Vec<SimpleCompletion>);

#[derive(Component)]
pub struct FormatRequest(pub Option<Vec<lsp_types::TextEdit>>);

#[derive(Component)]
pub struct Triples(pub Vec<MyQuad<'static>>);

#[derive(Component)]
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
