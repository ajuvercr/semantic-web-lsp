use sophia_api::{
    prelude::{Any, Dataset},
    quad::Quad,
    term::{BnodeId, GraphName, IriRef, Term, TermKind},
    MownStr,
};
use std::{borrow::Cow, hash::Hash, ops::Deref, usize};

use crate::ns::{owl, rdfs};

#[derive(Debug, Clone)]
pub struct MyQuad<'a> {
    pub subject: MyTerm<'a>,
    pub predicate: MyTerm<'a>,
    pub object: MyTerm<'a>,
    pub span: std::ops::Range<usize>,
}
impl<'a> std::fmt::Display for MyQuad<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}. # {:?}",
            self.subject, self.predicate, self.object, self.span
        )
    }
}

impl<'a> MyQuad<'a> {
    pub fn to_owned(&self) -> MyQuad<'static> {
        MyQuad {
            subject: self.subject.to_owned(),
            predicate: self.predicate.to_owned(),
            object: self.object.to_owned(),
            span: self.span.clone(),
        }
    }
}

impl<'a> Quad for MyQuad<'a> {
    type Term = MyTerm<'a>;

    fn s(&self) -> sophia_api::quad::QBorrowTerm<Self> {
        self.subject.borrow_term()
    }

    fn p(&self) -> sophia_api::quad::QBorrowTerm<Self> {
        self.predicate.borrow_term()
    }

    fn o(&self) -> sophia_api::quad::QBorrowTerm<Self> {
        self.object.borrow_term()
    }

    fn g(&self) -> GraphName<sophia_api::quad::QBorrowTerm<Self>> {
        None
    }

    fn to_spog(self) -> sophia_api::quad::Spog<Self::Term> {
        ([self.subject, self.predicate, self.object], None)
    }
}
// pub type MyQuad<'a> = ([MyTerm<'a>; 3], GraphName<MyTerm<'a>>);

#[derive(Debug, Clone, Eq)]
pub struct MyTerm<'a> {
    value: Cow<'a, str>,
    ty: Option<TermKind>,
    pub span: std::ops::Range<usize>,
}

impl Hash for MyTerm<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Ignore span
        self.value.hash(state);
        self.ty.hash(state);
    }
}

impl PartialEq for MyTerm<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Ignore span
        other.value == self.value && other.ty == self.ty
    }
}

impl<'a> std::fmt::Display for MyTerm<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.value)
    }
}

impl<'a> MyTerm<'a> {
    pub fn to_owned(&self) -> MyTerm<'static> {
        let value = Cow::Owned(self.value.to_string());
        MyTerm {
            value,
            ty: self.ty.clone(),
            span: self.span.clone(),
        }
    }
    pub fn named_node<T: Into<Cow<'a, str>>>(value: T, span: std::ops::Range<usize>) -> Self {
        Self {
            value: value.into(),
            ty: TermKind::Iri.into(),
            span,
        }
    }
    pub fn blank_node<T: Into<Cow<'a, str>>>(value: T, span: std::ops::Range<usize>) -> Self {
        Self {
            value: value.into(),
            ty: TermKind::BlankNode.into(),
            span,
        }
    }
    pub fn literal<T: Into<Cow<'a, str>>>(value: T, span: std::ops::Range<usize>) -> Self {
        Self {
            value: value.into(),
            ty: TermKind::Literal.into(),
            span,
        }
    }

    pub fn invalid(span: std::ops::Range<usize>) -> Self {
        Self {
            value: Cow::default(),
            ty: None,
            span,
        }
    }

    pub fn as_str(&'a self) -> &'a str {
        &self.value
    }
}

impl<'a> Term for MyTerm<'a> {
    type BorrowTerm<'x> = &'x Self
    where
        Self: 'x;

    fn kind(&self) -> sophia_api::term::TermKind {
        self.ty.unwrap_or(TermKind::Triple)
    }

    fn borrow_term(&self) -> Self::BorrowTerm<'_> {
        self
    }

    fn iri(&self) -> Option<sophia_api::term::IriRef<sophia_api::MownStr>> {
        self.is_iri()
            .then(|| IriRef::new_unchecked(MownStr::from_str(&self.value)))
    }

    fn bnode_id(&self) -> Option<sophia_api::term::BnodeId<sophia_api::MownStr>> {
        self.is_blank_node()
            .then(|| BnodeId::new_unchecked(MownStr::from_str(&self.value)))
    }

    fn lexical_form(&self) -> Option<sophia_api::MownStr> {
        self.is_literal().then(|| MownStr::from_str(&self.value))
    }

    fn datatype(&self) -> Option<sophia_api::term::IriRef<sophia_api::MownStr>> {
        None
    }

    fn language_tag(&self) -> Option<sophia_api::term::LanguageTag<sophia_api::MownStr>> {
        None
    }

    fn variable(&self) -> Option<sophia_api::term::VarName<sophia_api::MownStr>> {
        panic!("MyTerm does not supported variables")
    }

    fn triple(&self) -> Option<[Self::BorrowTerm<'_>; 3]> {
        panic!("MyTerm does not supported triples")
    }

    fn to_triple(self) -> Option<[Self; 3]>
    where
        Self: Sized,
    {
        panic!("MyTerm does not supported triples")
    }
}

#[derive(Default, Debug)]
pub struct Triples<'a> {
    pub base_url: String,
    pub triples: Vec<MyQuad<'a>>,
    pub base: Option<MyTerm<'a>>,
}

impl<'a> Triples<'a> {
    pub fn to_owned(&self) -> Triples<'static> {
        let triples = self.triples.iter().map(|q| q.to_owned()).collect();
        let base: Option<MyTerm<'static>> = self.base.as_ref().map(|x| x.to_owned());

        Triples {
            base,
            triples,
            base_url: self.base_url.clone(),
        }
    }

    pub fn imports(&self, cb: impl FnMut(IriRef<MownStr<'_>>) -> ()) {
        if let Some(ref base) = self.base {
            self.triples
                .quads_matching([base], [owl::imports], Any, Any)
                .flatten()
                .flat_map(|s| s.o().iri())
                .for_each(cb);
        }
    }

    pub fn sub_class_of(&self, mut cb: impl FnMut(IriRef<MownStr<'_>>, IriRef<MownStr<'_>>) -> ()) {
        self.triples
            .quads_matching(Any, [rdfs::subClassOf], Any, Any)
            .flatten()
            .flat_map(|s| match (s.s().iri(), s.o().iri()) {
                (Some(s), Some(o)) => Some((s, o)),
                _ => None,
            })
            .for_each(|(x, y)| cb(x, y));
    }
}

impl<'a> Deref for Triples<'a> {
    type Target = Vec<MyQuad<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.triples
    }
}
