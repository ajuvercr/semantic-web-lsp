use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    lang::{Lang, LangHelper},
    prelude::*,
    systems::TypeId,
    token::Token,
    triples::{MyQuad, MyTerm},
};
use bevy_ecs::{prelude::*, world::CommandQueue};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use lsp_types::{Position, Range, SemanticToken, SemanticTokenType};
use sophia_api::{prelude::Dataset, quad::Quad as _, term::matcher::TermMatcher};

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Tokens(pub Vec<Spanned<crate::token::Token>>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Element<L: Lang>(pub Spanned<L::Element>);

#[derive(Debug, Clone)]
pub struct Prefix {
    pub prefix: String,
    pub url: lsp_types::Url,
}

#[derive(Component, Debug)]
pub struct Prefixes(pub Vec<Prefix>, pub lsp_types::Url);
impl Deref for Prefixes {
    type Target = Vec<Prefix>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
            Token::IRIRef(x) => {
                return self.1.join(&x).ok().map(|x| x.to_string());
            }
            _ => None,
        }
    }

    pub fn expand_json(&self, token: &Token) -> Option<String> {
        match token {
            Token::Str(pref, _) => {
                if let Some(x) = pref.find(':') {
                    let prefix = &pref[..x];
                    if let Some(exp) = self.0.iter().find(|x| &x.prefix == prefix) {
                        return Some(format!("{}{}", exp.url.as_str(), &pref[x + 1..]));
                    }
                } else {
                    if let Some(exp) = self.0.iter().find(|x| &x.prefix == pref) {
                        return Some(exp.url.as_str().to_string());
                    }
                }

                return Some(
                    self.1
                        .join(&pref)
                        .ok()
                        .map(|x| x.to_string())
                        .unwrap_or(pref.to_string()),
                );
            }
            _ => None,
        }
    }
}

#[derive(Resource, AsRef, Deref, AsMut, DerefMut, Debug, Default)]
pub struct SemanticTokensDict(pub HashMap<SemanticTokenType, usize>);

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
pub struct KeyWords(pub Vec<&'static str>);

#[derive(Component, Debug, Default)]
pub struct HoverRequest(pub Vec<String>, pub Option<lsp_types::Range>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct FormatRequest(pub Option<Vec<lsp_types::TextEdit>>);

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct InlayRequest(pub Option<Vec<lsp_types::InlayHint>>);

#[derive(Component, Debug)]
pub struct PrepareRenameRequest {
    pub range: Range,
    pub placeholder: String,
}

#[derive(Component, Debug)]
pub struct RenameEdits(pub Vec<(lsp_types::Url, lsp_types::TextEdit)>, pub String);
// #[derive(Component, Debug)]
// pub struct PrepareRenameRequest {
//     range: Range,
//     placeholder: String,
// }

#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Types(pub HashMap<Cow<'static, str>, Vec<TypeId>>);

#[derive(Resource, Debug, Default)]
pub struct TypeHierarchy<'a> {
    numbers: HashMap<Cow<'a, str>, TypeId>,
    nodes: Vec<Cow<'a, str>>,
    subclass: Vec<HashSet<TypeId>>,
    superclass: Vec<HashSet<TypeId>>,
}

impl<'a> TypeHierarchy<'a> {
    pub fn get_id(&mut self, class: &str) -> TypeId {
        if let Some(id) = self.numbers.get(class) {
            *id
        } else {
            let new_id = TypeId(self.nodes.len());
            let class_cow: Cow<'a, str> = Cow::Owned(class.to_string());
            self.nodes.push(class_cow.clone());
            self.numbers.insert(class_cow, new_id);
            self.subclass.push(HashSet::new());
            self.superclass.push(HashSet::new());
            new_id
        }
    }

    pub fn get_id_ref(&self, class: &str) -> Option<TypeId> {
        self.numbers.get(class).copied()
    }

    pub fn set_subclass_of(&mut self, class: TypeId, to: TypeId) {
        self.subclass[class.0].insert(to);
        self.superclass[to.0].insert(class);
    }

    pub fn iter_subclass<'b>(&'b self, id: TypeId) -> impl Iterator<Item = Cow<'a, str>> + 'b {
        let mut stack = std::collections::VecDeque::new();
        stack.push_back(id);
        let mut done = HashSet::new();
        std::iter::from_fn(move || {
            while let Some(id) = stack.pop_front() {
                if done.contains(&id) {
                    continue;
                }
                done.insert(id);

                self.subclass[id.0].iter().for_each(|i| stack.push_back(*i));
                return Some(self.nodes[id.0].clone());
            }

            None
        })
    }

    pub fn type_name(&self, id: TypeId) -> Cow<'a, str> {
        self.nodes[id.0].clone()
    }

    pub fn iter_superclass<'b>(&'b self, id: TypeId) -> impl Iterator<Item = Cow<'a, str>> + 'b {
        let mut stack = std::collections::VecDeque::new();
        stack.push_back(id);
        let mut done = HashSet::new();
        std::iter::from_fn(move || {
            while let Some(id) = stack.pop_front() {
                if done.contains(&id) {
                    continue;
                }
                done.insert(id);

                self.superclass[id.0]
                    .iter()
                    .for_each(|i| stack.push_back(*i));
                return Some(self.nodes[id.0].clone());
            }

            None
        })
    }
}

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
