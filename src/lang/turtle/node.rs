use std::ops::Range;

use enum_methods::{EnumIntoGetters, EnumIsA, EnumToGetters};
use lsp_types::SemanticTokenType;

use super::{Literal, NamedNode};
use lsp_core::lang::{self, Token};
use lsp_core::model::Spanned;

#[derive(Clone, Debug, PartialEq, EnumIntoGetters, EnumIsA, EnumToGetters)]
pub enum Leaf {
    Literal(Literal),
    NamedNode(NamedNode),
    BlankNode(String),
    Invalid,
}

impl Token for Leaf {
    fn token(&self) -> Option<lsp_types::SemanticTokenType> {
        let t = match self {
            Leaf::Literal(lit) => match lit {
                Literal::RDF(_) => SemanticTokenType::STRING,
                Literal::Boolean(_) => SemanticTokenType::ENUM_MEMBER,
                Literal::Numeric(_) => SemanticTokenType::NUMBER,
            },
            Leaf::NamedNode(_) => SemanticTokenType::FUNCTION,
            Leaf::BlankNode(_) => SemanticTokenType::STRING,
            Leaf::Invalid => return None,
        };
        Some(t)
    }
}

impl Into<Leaf> for Literal {
    fn into(self) -> Leaf {
        Leaf::Literal(self)
    }
}

impl Into<Leaf> for NamedNode {
    fn into(self) -> Leaf {
        Leaf::NamedNode(self)
    }
}

impl Into<Leaf> for String {
    fn into(self) -> Leaf {
        Leaf::BlankNode(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PO {
    pub predicate: usize,
    pub objects: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, EnumIntoGetters, EnumIsA, EnumToGetters)]
pub enum Node {
    Leaf(Leaf),
    Triple { subject: usize, po: Vec<PO> },
    BlankNode { po: Vec<PO> },
    Base(Range<usize>, usize),
    Prefix(Range<usize>, Spanned<String>, usize),
    Root(Vec<usize>),
    Collection(Vec<usize>),
    Invalid,
}

impl Node {
    pub fn ty_str(&self) -> &'static str {
        match self {
            Node::Leaf(Leaf::Invalid) => "leaf(invalid)",
            Node::Leaf(Leaf::Literal(_)) => "leaf(literal)",
            Node::Leaf(Leaf::NamedNode(_)) => "leaf(namednode)",
            Node::Leaf(Leaf::BlankNode(_)) => "leaf(blanknod)",
            Node::Triple { .. } => "triple",
            Node::BlankNode { .. } => "blanknode",
            Node::Base(_, _) => "base",
            Node::Prefix(_, _, _) => "prefix",
            Node::Root(_) => "root",
            Node::Collection(_) => "collection",
            Node::Invalid => "invalid",
        }
    }
}

impl lang::Node<Leaf> for Node {
    fn leaf<'a>(&'a self) -> Option<&'a Leaf> {
        match self {
            Node::Leaf(leaf) => Some(leaf),
            _ => None,
        }
    }
}

pub use helpers::new_turtle;
mod helpers {
    use lsp_core::parent::ParentingSystem;

    use crate::lang::turtle::{Base, BlankNode, NamedNode, Prefix, Term, Triple, Turtle};
    use lsp_core::model::{spanned, Spanned};

    use super::{Leaf, Node, PO};

    type This = ParentingSystem<Spanned<Node>>;
    // impl ParentingSystem<Spanned<Node>> {
    fn increment(this: &mut This, parent: usize) -> usize {
        this.add(spanned(Node::Invalid, 0..0), parent)
    }

    fn add_leaf<N: Into<Leaf>>(
        this: &mut This,
        Spanned(nn, span): Spanned<N>,
        parent: usize,
    ) -> usize {
        let element = increment(this, parent);
        this.objects[element] = spanned(Node::Leaf(nn.into()), span);
        element
    }

    fn add_base(
        this: &mut This,
        Spanned(Base(span, nn), total_span): Spanned<Base>,
        parent: usize,
    ) -> usize {
        let element = increment(this, parent);
        let child = add_leaf(this, nn, element);
        this.objects[element] = spanned(Node::Base(span, child), total_span);
        element
    }

    fn add_prefix(
        this: &mut This,
        Spanned(
            Prefix {
                span,
                prefix,
                value,
            },
            total_span,
        ): Spanned<Prefix>,
        parent: usize,
    ) -> usize {
        let element = increment(this, parent);
        let child = add_leaf(this, value, element);
        this.objects[element] = spanned(Node::Prefix(span, prefix, child), total_span);
        element
    }

    fn add_term(this: &mut This, Spanned(term, span): Spanned<Term>, parent: usize) -> usize {
        match term {
            Term::Literal(x) => add_leaf(this, Spanned(x, span), parent),
            Term::NamedNode(x) => add_leaf(this, Spanned(x, span), parent),
            Term::BlankNode(x) => add_bnode(this, Spanned(x, span), parent),
            Term::Collection(x) => add_collection(this, Spanned(x, span), parent),
            Term::Invalid => todo!(),
        }
    }

    fn add_collection(
        this: &mut This,
        Spanned(terms, span): Spanned<Vec<Spanned<Term>>>,
        parent: usize,
    ) -> usize {
        let element = increment(this, parent);

        let nodes: Vec<_> = terms
            .into_iter()
            .map(|t| add_term(this, t, element))
            .collect();

        this.objects[element] = spanned(Node::Collection(nodes), span);
        element
    }

    fn add_bnode(
        this: &mut This,
        Spanned(bnode, span): Spanned<BlankNode>,
        parent: usize,
    ) -> usize {
        let element = increment(this, parent);

        let po = match bnode {
            BlankNode::Named(b) => return add_leaf(this, Spanned(b, span), element),
            BlankNode::Unnamed(po) => po,
            BlankNode::Invalid => todo!(),
        };

        let children = extract_po(this, po, element);
        this.objects[element] = spanned(Node::BlankNode { po: children }, span);
        element
    }

    fn add_triple(
        this: &mut This,
        Spanned(Triple { subject, po }, total_span): Spanned<Triple>,
        parent: usize,
    ) -> usize {
        let element = increment(this, parent);
        let subject = match subject.0 {
            Term::BlankNode(b) => add_bnode(this, Spanned(b, subject.1), element),
            Term::NamedNode(n) => add_leaf(this, Spanned(n, subject.1), element),
            _ => add_leaf(this, Spanned(NamedNode::Invalid, subject.1), element),
        };

        let po = extract_po(this, po, element);
        this.objects[element] = spanned(Node::Triple { subject, po }, total_span);

        element
    }

    fn extract_po(
        this: &mut This,
        po: Vec<Spanned<super::super::model::PO>>,
        parent: usize,
    ) -> Vec<PO> {
        let mut children = Vec::new();
        for Spanned(p, __) in po {
            let nn = add_leaf(
                this,
                p.predicate
                    .map(|x| x.named_node().unwrap_or(&NamedNode::Invalid).clone()),
                parent,
            );
            let objects = p
                .object
                .into_iter()
                .map(|x| add_term(this, x, parent))
                .collect();

            children.push(PO {
                predicate: nn,
                objects,
            });
        }
        children
    }

    pub fn new_turtle(Spanned(turtle, span): Spanned<Turtle>) -> This {
        let mut out = This::new();
        let this = &mut out;
        let root_id = increment(this, 0);
        let mut root = Vec::new();

        if let Some(b) = turtle.base {
            root.push(add_base(this, b, root_id));
        }

        for prefix in turtle.prefixes {
            root.push(add_prefix(this, prefix, root_id));
        }

        for triple in turtle.triples {
            root.push(add_triple(this, triple, root_id));
        }

        this.objects[root_id] = spanned(Node::Root(root), span.clone());
        out
    }
}
