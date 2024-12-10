use lang_turtle::{Based, NamedNode, Prefix, Triple, TriplesBuilder, TurtleSimpleError};
use lsp_core::{
    model::Spanned,
    token::{SparqlKeyword, Token},
};

fn rev_range(range: &std::ops::Range<usize>, len: usize) -> std::ops::Range<usize> {
    (len - range.end)..(len - range.start)
}

fn fix_span<T>(spanned: &mut Spanned<T>, len: usize) {
    spanned.1 = rev_range(&spanned.1, len);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base {
    pub token: Spanned<Token>,
    pub iri: Spanned<NamedNode>,
}

impl Base {
    pub fn fix_spans(&mut self, len: usize) {
        fix_span(&mut self.token, len);
        fix_span(&mut self.iri, len);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Prologue {
    Base {
        token: Spanned<Token>,
        iri: Spanned<NamedNode>,
    },
    Prefix(Prefix),
}
impl Prologue {
    pub fn fix_spans(&mut self, len: usize) {
        match self {
            Prologue::Base { token, iri } => {
                fix_span(token, len);
                fix_span(iri, len);
            }
            Prologue::Prefix(pref) => {
                pref.fix_spans(len);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatasetClause {
    pub from: Spanned<SparqlKeyword>,
    pub named: Option<Spanned<SparqlKeyword>>,
    pub iri: Spanned<NamedNode>,
}
impl DatasetClause {
    pub fn fix_spans(&mut self, len: usize) {
        fix_span(&mut self.from, len);
        if let Some(ref mut n) = &mut self.named {
            fix_span(n, len);
        }
        fix_span(&mut self.iri, len);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupGraphPattern {
    SubSelect(Box<SubSelect>),
    GroupGraph(Vec<Spanned<GroupGraphPatternSub>>),
    Invalid,
}
impl GroupGraphPattern {
    pub fn fix_spans(&mut self, len: usize) {
        match self {
            GroupGraphPattern::SubSelect(sub_select) => sub_select.fix_spans(len),
            GroupGraphPattern::GroupGraph(vec) => vec.iter_mut().for_each(|x| {
                x.fix_spans(len);
                fix_span(x, len);
            }),
            GroupGraphPattern::Invalid => todo!(),
        }
    }

    pub fn ingest_triples<'a>(
        &'a self,
        builder: &mut TriplesBuilder<'a, Query>,
    ) -> Result<(), TurtleSimpleError> {
        match self {
            GroupGraphPattern::SubSelect(sub_select) => sub_select.ingest_triples(builder)?,
            GroupGraphPattern::GroupGraph(xs) => {
                for x in xs {
                    x.ingest_triples(builder)?;
                }
            }
            GroupGraphPattern::Invalid => todo!(),
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhereClause {
    pub kwd: Option<Spanned<SparqlKeyword>>,
    pub ggp: Spanned<GroupGraphPattern>,
}
impl WhereClause {
    pub fn fix_spans(&mut self, len: usize) {
        if let Some(ref mut n) = &mut self.kwd {
            fix_span(n, len);
        }
        fix_span(&mut self.ggp, len);
        self.ggp.fix_spans(len);
    }

    pub fn ingest_triples<'a>(
        &'a self,
        builder: &mut TriplesBuilder<'a, Query>,
    ) -> Result<(), TurtleSimpleError> {
        self.ggp.ingest_triples(builder)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bind {
    pub expr: Spanned<Expression>,
    pub kwd: Spanned<SparqlKeyword>,
    pub var: Spanned<Variable>,
}
impl Bind {
    pub fn fix_spans(&mut self, len: usize) {
        fix_span(&mut self.expr, len);
        fix_span(&mut self.kwd, len);
        fix_span(&mut self.var, len);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Solution {
    All,
    Var(Variable),
    VarAs(Bind),
}
impl Solution {
    pub fn fix_spans(&mut self, len: usize) {
        match self {
            Solution::All => {}
            Solution::Var(_) => {}
            Solution::VarAs(bind) => bind.fix_spans(len),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectClause {
    pub kwd: Spanned<SparqlKeyword>,
    pub modifier: Option<Spanned<SparqlKeyword>>,
    pub solutions: Vec<Spanned<Solution>>,
}
impl SelectClause {
    pub fn fix_spans(&mut self, len: usize) {
        if let Some(ref mut n) = &mut self.modifier {
            fix_span(n, len);
        }
        fix_span(&mut self.kwd, len);
        self.solutions.iter_mut().for_each(|x| {
            fix_span(x, len);
            x.fix_spans(len);
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstructClause {
    kwd: Spanned<SparqlKeyword>,
    template: Option<(
        Spanned<Token>,
        Vec<Spanned<GroupGraphPatternSub>>,
        Spanned<Token>,
    )>,
}
impl ConstructClause {
    pub fn fix_spans(&mut self, len: usize) {
        fix_span(&mut self.kwd, len);

        if let Some((ref mut t1, ref mut ggps, ref mut t2)) = &mut self.template {
            fix_span(t1, len);
            fix_span(t2, len);
            ggps.iter_mut().for_each(|x| {
                fix_span(x, len);
                x.fix_spans(len);
            });
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryClause {
    Select(SelectClause),
    Construct(ConstructClause),
    Invalid,
}

impl QueryClause {
    pub fn fix_spans(&mut self, len: usize) {
        match self {
            QueryClause::Select(select_clause) => select_clause.fix_spans(len),
            QueryClause::Construct(construct_clause) => construct_clause.fix_spans(len),
            QueryClause::Invalid => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubSelect {
    pub select: SelectClause,
    pub where_clause: WhereClause,
    pub modifier: Vec<Spanned<Modifier>>,
    // TODO values
}
impl SubSelect {
    pub fn fix_spans(&mut self, len: usize) {
        self.select.fix_spans(len);
        self.where_clause.fix_spans(len);
        self.modifier.iter_mut().for_each(|x| {
            fix_span(x, len);
            x.fix_spans(len);
        });
    }
    pub fn ingest_triples<'a>(
        &'a self,
        builder: &mut TriplesBuilder<'a, Query>,
    ) -> Result<(), TurtleSimpleError> {
        self.where_clause.ingest_triples(builder)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphPatternNotTriples {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupGraphPatternSub {
    Triple(Spanned<Triple>),
    Kwd(Spanned<SparqlKeyword>, Spanned<GroupGraphPattern>),
    Filter(Spanned<SparqlKeyword>, ()),
    Union(
        Spanned<GroupGraphPattern>,
        Vec<(Spanned<SparqlKeyword>, Spanned<GroupGraphPattern>)>,
    ),
    GraphOrService(
        Spanned<SparqlKeyword>,
        Option<Spanned<SparqlKeyword>>,
        Spanned<NamedNode>,
        Spanned<GroupGraphPattern>,
    ),
    Bind(
        Spanned<SparqlKeyword>,
        Spanned<Token>,
        Spanned<Bind>,
        Spanned<Token>,
    ),
    Inline(Spanned<()>),
}
impl GroupGraphPatternSub {
    pub fn fix_spans(&mut self, len: usize) {
        match self {
            GroupGraphPatternSub::Triple(triple) => {
                fix_span(triple, len);
                triple.fix_spans(len);
            }
            GroupGraphPatternSub::Kwd(spanned, spanned1) => {
                fix_span(spanned, len);
                fix_span(spanned1, len);
                spanned1.fix_spans(len);
            }
            GroupGraphPatternSub::Union(spanned, xs) => {
                fix_span(spanned, len);
                spanned.fix_spans(len);
                xs.iter_mut().for_each(|(ref mut kwd, ref mut x)| {
                    fix_span(kwd, len);
                    fix_span(x, len);
                    x.fix_spans(len);
                })
            }
            GroupGraphPatternSub::Filter(spanned, _) => {
                fix_span(spanned, len);
            }
            GroupGraphPatternSub::GraphOrService(spanned, spanned1, spanned2, spanned3) => {
                fix_span(spanned, len);
                if let Some(ref mut x) = spanned1 {
                    fix_span(x, len);
                }
                fix_span(spanned2, len);
                fix_span(spanned3, len);
                spanned3.fix_spans(len);
            }
            GroupGraphPatternSub::Bind(spanned, spanned1, spanned2, spanned3) => {
                fix_span(spanned, len);
                fix_span(spanned1, len);
                fix_span(spanned2, len);
                spanned2.fix_spans(len);
                fix_span(spanned3, len);
            }
            GroupGraphPatternSub::Inline(spanned) => {
                fix_span(spanned, len);
            }
        }
    }

    pub fn ingest_triples<'a>(
        &'a self,
        builder: &mut TriplesBuilder<'a, Query>,
    ) -> Result<(), TurtleSimpleError> {
        match self {
            GroupGraphPatternSub::Triple(triple) => builder.ingest(&triple)?,
            GroupGraphPatternSub::Kwd(_, x) => x.ingest_triples(builder)?,
            GroupGraphPatternSub::Union(x, xs) => {
                x.ingest_triples(builder)?;
                for (_, x) in xs {
                    x.ingest_triples(builder)?;
                }
            }
            GroupGraphPatternSub::GraphOrService(_, _, _, x) => x.ingest_triples(builder)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Modifier {
    GroupBy(Spanned<SparqlKeyword>, Spanned<SparqlKeyword>, Spanned<()>),
    Having(Spanned<SparqlKeyword>, Spanned<()>),
    OrderBy(Spanned<SparqlKeyword>, Spanned<SparqlKeyword>, Spanned<()>),
    LimitOffset(Spanned<SparqlKeyword>, Spanned<String>),
}

impl Modifier {
    pub fn fix_spans(&mut self, len: usize) {
        match self {
            Modifier::GroupBy(spanned, spanned1, spanned2) => {
                fix_span(spanned, len);
                fix_span(spanned1, len);
                fix_span(spanned2, len);
            }
            Modifier::Having(spanned, spanned1) => {
                fix_span(spanned, len);
                fix_span(spanned1, len);
            }
            Modifier::OrderBy(spanned, spanned1, spanned2) => {
                fix_span(spanned, len);
                fix_span(spanned1, len);
                fix_span(spanned2, len);
            }
            Modifier::LimitOffset(spanned, spanned1) => {
                fix_span(spanned, len);
                fix_span(spanned1, len);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub base: lsp_types::Url,
    pub prefixes: Vec<Spanned<Prefix>>,
    pub base_statement: Option<Spanned<Base>>,
    pub kwds: QueryClause,
    pub datasets: Vec<Spanned<DatasetClause>>,
    pub where_clause: Spanned<WhereClause>,
    pub modifier: Vec<Spanned<Modifier>>,
}
impl Query {
    pub fn fix_spans(&mut self, len: usize) {
        self.prefixes.iter_mut().for_each(|x| {
            fix_span(x, len);
            x.fix_spans(len);
        });
        if let Some(b) = &mut self.base_statement {
            b.fix_spans(len);
            fix_span(b, len);
        }
        self.kwds.fix_spans(len);
        self.datasets.iter_mut().for_each(|x| {
            fix_span(x, len);
            x.fix_spans(len);
        });
        fix_span(&mut self.where_clause, len);
        self.where_clause.fix_spans(len);
        self.modifier.iter_mut().for_each(|x| {
            fix_span(x, len);
            x.fix_spans(len);
        });
    }

    pub fn ingest_triples<'a>(
        &'a self,
        builder: &mut TriplesBuilder<'a, Query>,
    ) -> Result<(), TurtleSimpleError> {
        self.where_clause.ingest_triples(builder)?;
        Ok(())
    }

    pub fn set_base(&mut self, base: lsp_types::Url) {
        self.base = self
            .base_statement
            .as_ref()
            .iter()
            .find_map(|x| match x.iri.value() {
                NamedNode::Full(x) => lsp_types::Url::parse(&x).ok(),
                _ => None,
            })
            .unwrap_or_else(|| base.clone());
    }

    pub fn get_base(&self) -> &lsp_types::Url {
        &self.base
    }
}

impl Based for Query {
    fn get_base(&self) -> &lsp_types::Url {
        &self.base
    }

    fn prefixes(&self) -> &[Spanned<Prefix>] {
        &self.prefixes
    }
}

impl Default for Query {
    fn default() -> Self {
        Query {
            base: lsp_types::Url::parse("memory://somefile.sq").unwrap(),
            base_statement: None,
            prefixes: vec![],
            kwds: QueryClause::Invalid,
            datasets: vec![],
            where_clause: Spanned(
                WhereClause {
                    kwd: None,
                    ggp: Spanned(GroupGraphPattern::Invalid, 0..0),
                },
                0..0,
            ),
            modifier: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use sophia_iri::resolve::BaseIri;

    use crate::{parsing::parse, tokenizer};

    use super::*;

    fn parse_sparql(inp: &str) -> Query {
        let (tokens, _) = tokenizer::tokenize(inp);
        let (jsonld, _) = parse(
            inp,
            lsp_types::Url::parse("memory::myFile.sq").unwrap(),
            tokens,
        );

        jsonld.0
    }

    #[test]
    fn simple_sparql_1() {
        let st = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
PREFIX  ns:  <http://example.org/ns#>

SELECT  ?title ?price
{ ?x ns:price ?p .
  ?x dc:title ?title . 
  ?x ns:discount ?discount .
}
        "#;

        let query = parse_sparql(st);
        let base = BaseIri::new(query.base.to_string()).unwrap();
        let mut builder = TriplesBuilder::new(&query, base);
        query.ingest_triples(&mut builder).expect("builds fine");

        assert_eq!(builder.triples.len(), 3);
    }

    #[test]
    fn simple_sparql_2() {
        let st = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
PREFIX  ns:  <http://example.org/ns#>

SELECT  ?title ?price
{ ?x ns:price ?p .
  ?x dc:title ?title . 
  [ # this is one triple as well
  ] ns:discount ?discount .
}
        "#;

        let query = parse_sparql(st);
        let base = BaseIri::new(query.base.to_string()).unwrap();
        let mut builder = TriplesBuilder::new(&query, base);
        query.ingest_triples(&mut builder).expect("builds fine");

        for t in &builder.triples {
            println!("t {}", t);
        }

        assert_eq!(builder.triples.len(), 4);
    }
}
