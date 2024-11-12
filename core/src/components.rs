use crate::{
    lang::{Lang, SimpleCompletion},
    model::Spanned,
    triples::MyQuad,
};
use bevy_ecs::{prelude::*, world::CommandQueue};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use lsp_types::SemanticToken;

#[derive(Component)]
pub struct Tokens<L: Lang>(pub Vec<Spanned<L::Token>>);

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
pub struct Label(pub lsp_types::Url);

#[derive(Component)]
pub struct HighlightRequest(pub Vec<SemanticToken>);

#[derive(Resource)]
pub struct CommandReceiver(pub UnboundedReceiver<CommandQueue>);

#[derive(Resource, Debug, Clone)]
pub struct CommandSender(pub UnboundedSender<CommandQueue>);

#[derive(Component)]
pub struct CurrentWord(pub lsp_types::Range);

#[derive(Component)]
pub struct CompletionRequest(pub Vec<SimpleCompletion>);

#[derive(Component)]
pub struct FormatRequest(pub Option<Vec<lsp_types::TextEdit>>);

#[derive(Component)]
pub struct Triples(pub Vec<MyQuad<'static>>);
