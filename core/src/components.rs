use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    lang::{Lang, LangHelper},
    prelude::*,
    systems::TypeId,
    util::token::Token,
};
use bevy_ecs::{prelude::*, world::CommandQueue};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use lsp_types::{Position, Range, SemanticToken, SemanticTokenType};

/// [`Component`] that contains the parsed semantic element (i.e. Turtle, JSONLD).
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Element<L: Lang>(pub Spanned<L::Element>);

/// One defined prefix, maps prefix to url
#[derive(Debug, Clone)]
pub struct Prefix {
    pub prefix: String,
    pub url: lsp_types::Url,
}

/// [`Component`] that containing defined prefixes and base URL.
///
/// [`lsp_core`](crate) uses [`Prefixes`] in different systems, for example
/// - to check for undefined prefixes diagnostics with
/// [`undefined_prefix`](crate::prelude::systems::undefined_prefix)
/// - derive linked documents [`DocumentLinks`] with
/// [`derive_prefix_links`](crate::prelude::systems::derive_prefix_links)
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

/// [`Resource`] mapping a ['SemanticTokenType'] to their used index.
///
/// This index is important because with LSP, are retrieved during startup, then only indexes are
/// used to indicate semantic token types.
#[derive(Resource, AsRef, Deref, AsMut, DerefMut, Debug, Default)]
pub struct SemanticTokensDict(pub HashMap<SemanticTokenType, usize>);

/// Simple wrapper structure that derives [`Component`]
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Wrapped<E>(pub E);

/// Simple wrapper for errors that derives [`Component`]
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Errors<E>(pub Vec<E>);

/// [`Component`] containing the current source code as [`String`]
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Source(pub String);

/// [`Component`] containing the current source code as [`ropey::Rope`]
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct RopeC(pub ropey::Rope);

/// [`Component`] that allows for language specific implementation for certain things, reducing
/// code duplication.
#[derive(Component, Debug, AsRef, Deref)]
pub struct DynLang(pub Box<dyn LangHelper + 'static + Send + Sync>);

/// [`Component`] indicating whether or not the document is actually open.
///
/// Documents that are not [`Open`] don't publish diagnostics for example
#[derive(Component, Debug)]
pub struct Open;

/// [`Component`] indicating whether or not the document is dirty, a dirty document parsed with
/// errors.
///
/// A document is often Dirty, computational intens calculation can be done on documents that are
/// not dirty, like [`derive_classes`](crate::prelude::systems::derive_classes) and [`derive_properties`](crate::prelude::systems::derive_properties).
#[derive(Component, Debug)]
pub struct Dirty;

/// [`Component`] containing the [`lsp_types::URL`] of the current document.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Label(pub lsp_types::Url);

/// [`Resource`] used to receive command queues. These command queues are handled with [`handle_tasks`](crate::prelude::systems::handle_tasks).
#[derive(Resource, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct CommandReceiver(pub UnboundedReceiver<CommandQueue>);

/// [`Resource`] used to send command queues, allowing for async operations.
#[derive(Resource, AsRef, Deref, AsMut, DerefMut, Debug, Clone)]
pub struct CommandSender(pub UnboundedSender<CommandQueue>);

/// [`Component`] used to remember the linked documents.
///
/// This is used, for example, to only suggest properties defined in a linked document.
/// Or only validate with shapes found in linked documents.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug, Clone)]
pub struct DocumentLinks(pub Vec<(lsp_types::Url, &'static str)>);

/// [`Component`] used to wrap an incoming [`lsp_types::Position`].
///
/// This component is translated into [`TokenComponent`] and [`TripleComponent`]
/// with [`get_current_token`](crate::prelude::systems::get_current_token)
/// and [get_current_triple](crate::prelude::systems::get_current_triple) respectively.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct PositionComponent(pub Position);

/// [`Component`] containing the typical keywords for the current language.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct KeyWords(pub Vec<&'static str>);

/// [`Component`] indicating that the current document is currently handling a Hightlight request.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct HighlightRequest(pub Vec<SemanticToken>);

/// [`Component`] indicating that the current document is currently handling a Hover request.
#[derive(Component, Debug, Default)]
pub struct HoverRequest(pub Vec<String>, pub Option<lsp_types::Range>);

/// [`Component`] indicating that the current document is currently handling a Format request.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct FormatRequest(pub Option<Vec<lsp_types::TextEdit>>);

/// [`Component`] indicating that the current document is currently handling a Inlay request.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct InlayRequest(pub Option<Vec<lsp_types::InlayHint>>);

/// [`Component`] indicating that the current document is currently handling a PrepareRename request.
#[derive(Component, Debug)]
pub struct PrepareRenameRequest {
    pub range: Range,
    pub placeholder: String,
}

/// [`Component`] indicating that the current document is currently handling a Rename request,
/// collecting [TextEdits](`lsp_types::TextEdit`).
#[derive(Component, Debug)]
pub struct RenameEdits(pub Vec<(lsp_types::Url, lsp_types::TextEdit)>, pub String);

/// maps terms to all known correct types.
#[derive(Component, AsRef, Deref, AsMut, DerefMut, Debug)]
pub struct Types(pub HashMap<Cow<'static, str>, Vec<TypeId>>);

/// [`Resource`] used to set and get all super and subtypes starting from a [`TypeId`]
///
/// Example
/// ```
/// use lsp_core::components::TypeHierarchy;
///
/// let mut hierarchy = TypeHierarchy::default();
/// let image_id = hierarchy.get_id("http://xmlns.com/foaf/0.1/Image");
/// let document_id = hierarchy.get_id("http://xmlns.com/foaf/0.1/Document");
/// hierarchy.set_subclass_of(image_id, document_id);
///
/// for ty in hierarchy.iter_superclass(document_id) {
///     // first "http://xmlns.com/foaf/0.1/Document"
///     // then "http://xmlns.com/foaf/0.1/Image"
///     println!("Type {}", ty);
/// }
/// ```
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
