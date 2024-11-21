use std::fmt::Debug;

use crate::{
    lang::{Lang, LangHelper, SimpleCompletion},
    model::Spanned,
    token::Token,
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

#[derive(Debug)]
pub struct Prefix {
    pub prefix: String,
    pub url: lsp_types::Url,
}

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Prefixes(pub Vec<Prefix>);
impl Prefixes {
    pub fn shorten(&self, value: &str) -> Option<String> {
        let try_shorten = |prefix: &Prefix| {
            let short = value.strip_prefix(prefix.url.as_str())?;
            Some(format!("{}:{}", prefix.prefix, short))
        };

        self.0.iter().flat_map(try_shorten).next()
    }

    pub fn expand(&self, token: &Token) -> Option<String> {
        match token {
            Token::PNameLN(pref, x) => {
                let pref = pref.as_ref().map(|x| x.as_str()).unwrap_or("");
                let prefix = self.0.iter().find(|x| &x.prefix == pref)?;
                Some(format!("{}{}", prefix.url, x))
            }
            _ => None,
        }
    }

    pub fn expand_json(&self, token: &Token) -> Option<String> {
        match token {
            Token::Str(pref, _) => {
                if let Some(x) = pref.find(':') {
                    let prefix = &pref[..x];
                    if let Some(exp) = self.iter().find(|x| &x.prefix == prefix) {
                        return Some(format!("{}{}", exp.url.as_str(), &pref[x + 1..]));
                    }
                } else {
                    if let Some(exp) = self.iter().find(|x| &x.prefix == pref) {
                        return Some(exp.url.as_str().to_string());
                    }
                }
                Some(pref.to_string())
            }
            _ => None,
        }
    }
}

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Wrapped<E>(pub E);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Errors<E>(pub Vec<E>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Source(pub String);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct RopeC(pub ropey::Rope);


#[derive(Component, Debug, AsRef, Deref)]
pub struct DynLang(pub Box<dyn LangHelper + 'static + Send + Sync>);

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

    pub fn objects<'s, S, P>(&'s self, subj: S, pred: P) -> impl Iterator<Item = &MyTerm<'_>>
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
            .map(|x| x.o())
    }
}
