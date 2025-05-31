use lsp_core::prelude::Token;
use similar::{Change, ChangeTag};

#[derive(Debug)]
pub struct Ctx {
    changes: Vec<Change<Token>>,
    at: usize,
}

impl Ctx {
    pub fn peek(&self) -> Option<&Change<Token>> {
        self.changes.get(self.at)
    }

    pub fn inc(&mut self) -> T {
        let out = self.at;
        self.at += 1;
        T { index: out }
    }

    pub fn insert(&self) -> bool {
        self.changes
            .get(self.at)
            .map(|x| x.tag() == ChangeTag::Insert)
            .unwrap_or_default()
    }

    pub fn delete(&self) -> bool {
        self.changes
            .get(self.at)
            .map(|x| x.tag() == ChangeTag::Delete)
            .unwrap_or_default()
    }

    pub fn equal(&self) -> bool {
        self.changes
            .get(self.at)
            .map(|x| x.tag() == ChangeTag::Equal)
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct T {
    /// index inside the old token array
    index: usize,
}

#[derive(Debug, Clone)]
enum Mark {
    Invalid(T),
    Missing,
    Mark(T),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Term {
    Missing,
    NamedNode(T),
    // BlankNode(BlankNode),
}

impl Term {
    pub fn apply(&mut self, ctx: &mut Ctx) -> bool {
        match self {
            Term::Missing => {
                if ctx.insert() {
                    match ctx.peek().map(|x| x.value_ref()) {
                        Some(Token::IRIRef(_))
                        | Some(Token::PNameLN(_, _))
                        | Some(Token::Invalid(_)) => {
                            *self = Term::NamedNode(ctx.inc());
                            true
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            Term::NamedNode(t) if ctx.equal() => {
                t.index = ctx.peek().unwrap().new_index().unwrap();
                true
            }
            _ if ctx.delete() => {
                ctx.inc();
                *self = Term::Missing;

                self.apply(ctx);

                true
            }
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Term::Missing
    }
}

// #[derive(Debug, Clone)]
// enum BlankNode {
//     Named(T),
//     Unnamed { start: Mark, po: Vec<PO>, end: Mark },
// }

#[derive(Debug, Clone)]
struct PO {
    predicate: Term,
    objects: Vec<Object>,
    command: Option<T>,
}

impl PO {
    pub fn apply(&mut self, ctx: &mut Ctx) {
        self.predicate.apply(ctx);
    }

    pub fn is_empty(&self) -> bool {
        if !self.predicate.is_empty() {
            return false;
        }

        if !self.objects.is_empty() {
            return false;
        }

        self.command.is_none()
    }
}

#[derive(Debug, Clone)]
struct Object {
    object: Term,
    commad: Option<T>,
}

#[derive(Debug, Clone)]
struct Triple {
    subject: Term,
    po: Vec<PO>,
}
impl Triple {
    pub fn apply(&mut self, ctx: &mut Ctx) {
        self.subject.apply(ctx);
    }
}
