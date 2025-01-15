use chumsky::{prelude::*, Error};
use lang_turtle::{named_node, triple, Prefix};
use lsp_core::{
    prelude::{spanned, Spanned},
    token::{SparqlExpr, SparqlKeyword, Token},
};

use crate::model::{
    Base, Bind, DatasetClause, Expression, GroupGraphPattern, GroupGraphPatternSub, Modifier,
    Prologue, Query, QueryClause, SelectClause, Solution, SubSelect, Variable, WhereClause,
};

fn sparql_kwd(
    kwd: SparqlKeyword,
) -> impl Parser<Token, Spanned<SparqlKeyword>, Error = Simple<Token>> + Clone {
    just(Token::SparqlKeyword(kwd.clone()))
        .to(kwd)
        .map_with_span(spanned)
}

fn prologue() -> impl Parser<Token, Prologue, Error = Simple<Token>> + Clone {
    let base = named_node()
        .map_with_span(spanned)
        .then(just(Token::SparqlBase).map_with_span(spanned))
        .map(|(iri, token)| Prologue::Base { token, iri });

    let prefix = named_node()
        .map_with_span(spanned)
        .then(select! { |span| Token::PNameLN(x, _) => Spanned(x.unwrap_or_default(), span)})
        .then(just(Token::SparqlPrefix).map_with_span(|_, s| s))
        .map(|((value, prefix), span)| {
            Prologue::Prefix(Prefix {
                span,
                prefix,
                value,
            })
        });

    base.or(prefix)
}

fn dataset_clause() -> impl Parser<Token, DatasetClause, Error = Simple<Token>> + Clone {
    named_node()
        .map_with_span(spanned)
        .then(sparql_kwd(SparqlKeyword::Named).or_not())
        .then(sparql_kwd(SparqlKeyword::From))
        .map(|((iri, named), from)| DatasetClause { from, named, iri })
}

fn expression() -> impl Parser<Token, Expression, Error = Simple<Token>> + Clone {
    todo()
}

fn bind() -> impl Parser<Token, Bind, Error = Simple<Token>> + Clone {
    variable()
        .map_with_span(spanned)
        .then(sparql_kwd(SparqlKeyword::As))
        .then(expression().map_with_span(spanned))
        .map(|((var, kwd), expr)| Bind { var, kwd, expr })
}

fn variable() -> impl Parser<Token, Variable, Error = Simple<Token>> + Clone {
    select! {
        Token::Variable(s) => Variable(s),
    }
}

fn select_clause() -> impl Parser<Token, SelectClause, Error = Simple<Token>> + Clone {
    let star = just(Token::SparqlExpr(SparqlExpr::Times))
        .to(Solution::All)
        .map_with_span(spanned)
        .map(|x| vec![x]);

    let others = bind()
        .delimited_by(just(Token::CurlClose), just(Token::CurlOpen))
        .map(Solution::VarAs)
        .or(variable().map(Solution::Var))
        .map_with_span(spanned)
        .repeated();

    // This might be different to support ask and describe
    star.or(others)
        .then(
            sparql_kwd(SparqlKeyword::Distinct)
                .or(sparql_kwd(SparqlKeyword::Reduced))
                .or_not(),
        )
        .then(sparql_kwd(SparqlKeyword::Select))
        .map(|((solutions, modifier), kwd)| SelectClause {
            kwd,
            modifier,
            solutions,
        })
}

fn sub_select() -> impl Parser<Token, SubSelect, Error = Simple<Token>> + Clone {
    recursive(|sub_select| {
        let modi = modifier().map_with_span(spanned).repeated();
        modi.then(where_clause(sub_select))
            .then(select_clause())
            .map(|((modifier, where_clause), select)| SubSelect {
                modifier,
                where_clause,
                select,
            })
    })
}

fn group_graph_pattern_sub(
    ggp: impl Parser<Token, GroupGraphPattern, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, GroupGraphPatternSub, Error = Simple<Token>> + Clone {
    let next_check = none_of([Token::CurlOpen]).rewind();

    let trip = triple()
        .map_with_span(spanned)
        .map(GroupGraphPatternSub::Triple)
        .labelled("triple");
    let kwd = just(Token::CurlClose)
        .rewind()
        .ignore_then(ggp.clone())
        .clone()
        .map_with_span(spanned)
        .then(sparql_kwd(SparqlKeyword::Minus).or(sparql_kwd(SparqlKeyword::Optional)))
        .map(|(ggp, kwd)| GroupGraphPatternSub::Kwd(kwd, ggp));

    let union = just(Token::CurlClose)
        .rewind()
        .ignore_then(ggp.clone())
        .map_with_span(spanned)
        .then(
            sparql_kwd(SparqlKeyword::Union)
                .then(ggp.map_with_span(spanned))
                .repeated(),
        )
        .map(|(start, rest)| GroupGraphPatternSub::Union(start, rest));

    // TODO add the others
    //

    next_check.ignore_then(trip.or(kwd).or(union).labelled("group_graph_pattern_sub"))
}

fn expect_it(
    token: Token,
    st: &'static str,
) -> impl Parser<Token, Token, Error = Simple<Token>> + Clone {
    just(token.clone()).or(none_of([token.clone()]).try_map(move |x: Token, span| {
        println!("{} didn't expect {}", st, x);
        Err(Simple::expected_input_found(
            span,
            [Some(token.clone())],
            Some(x.clone()),
        ))
    }))
}

fn group_graph_pattern(
    select: impl Parser<Token, SubSelect, Error = Simple<Token>> + Clone + 'static,
) -> impl Parser<Token, GroupGraphPattern, Error = Simple<Token>> + Clone {
    let s = select.clone();
    recursive(|ggp| {
        let select = s
            .clone()
            .map(Box::from)
            .map(GroupGraphPattern::SubSelect)
            .labelled("sub_select");

        let gg = group_graph_pattern_sub(ggp)
            .map_with_span(spanned)
            .repeated()
            .map(GroupGraphPattern::GroupGraph);

        let close = expect_it(Token::CurlClose, "close").labelled("CurlClose");
        let open = expect_it(Token::CurlOpen, "open").labelled("CurlOpen");

        close.ignore_then(gg.or(select)).then_ignore(open)
    })
}

fn where_clause(
    select: impl Parser<Token, SubSelect, Error = Simple<Token>> + Clone + 'static,
) -> impl Parser<Token, WhereClause, Error = Simple<Token>> + Clone {
    group_graph_pattern(select)
        .map_with_span(spanned)
        .then(sparql_kwd(SparqlKeyword::Where).or_not())
        .map(|(ggp, kwd)| WhereClause { ggp, kwd })
}

fn modifier() -> impl Parser<Token, Modifier, Error = Simple<Token>> + Clone {
    let num = select!(
        Token::Number(x) => x,
    )
    .map_with_span(spanned);
    let limit_offset = num
        .then(sparql_kwd(SparqlKeyword::Limit).or(sparql_kwd(SparqlKeyword::Offset)))
        .map(|(num, kwd)| Modifier::LimitOffset(kwd, num));
    limit_offset
}

pub fn query(base: lsp_types::Url) -> impl Parser<Token, Query, Error = Simple<Token>> + Clone {
    let prologues = prologue().map_with_span(spanned).repeated().map(|xs| {
        let mut base = None;
        let mut prefixes = vec![];
        xs.into_iter().for_each(|Spanned(x, span)| match x {
            Prologue::Base { token, iri } => base = Some(Spanned(Base { token, iri }, span)),
            Prologue::Prefix(prefix) => prefixes.push(Spanned(prefix, span)),
        });
        (base, prefixes)
    });
    let kwds = select_clause().map(QueryClause::Select);
    let datasets = dataset_clause().map_with_span(spanned).repeated();
    let where_clause = where_clause(sub_select()).map_with_span(spanned);
    let modifiers = modifier().map_with_span(spanned).repeated();

    modifiers
        .then(where_clause)
        .then(datasets)
        .then(kwds)
        .then(prologues)
        .map(
            move |((((modifier, where_clause), datasets), kwds), (base_statement, prefixes))| {
                Query {
                    base_statement,
                    prefixes,
                    base: base.clone(),
                    modifier,
                    where_clause,
                    datasets,
                    kwds,
                }
            },
        )
}

pub fn parse(
    source: &str,
    base: lsp_types::Url,
    tokens: Vec<Spanned<Token>>,
) -> (Spanned<Query>, Vec<(usize, Simple<Token>)>) {
    let len = source.len();
    let rev_range = |range: std::ops::Range<usize>| (len - range.end)..(len - range.start);
    let stream = chumsky::Stream::from_iter(
        0..len,
        tokens
            .into_iter()
            .rev()
            .filter(|x| !x.is_comment())
            .map(|Spanned(x, s)| (x, rev_range(s))),
    );

    let parser = query(base)
        .map_with_span(spanned)
        .then_ignore(end().recover_with(skip_then_retry_until([])));
    let (mut json, json_errors) = parser.parse_recovery(stream);

    json.iter_mut().for_each(|query| query.fix_spans(len));
    let json_errors: Vec<_> = json_errors.into_iter().map(|error| (len, error)).collect();
    (
        json.unwrap_or(Spanned(Query::default(), 0..source.len())),
        json_errors,
    )
}

#[cfg(test)]
mod tests {
    use chumsky::Stream;

    use crate::{parsing::select_clause, tokenizer};

    use super::*;
    pub fn parse_it<T, P: Parser<Token, T, Error = Simple<Token>>>(
        turtle: &str,
        parser: P,
    ) -> (Option<T>, Vec<Simple<Token>>) {
        let (tokens, _) = tokenizer::tokenize(turtle);
        for token in &tokens {
            println!("token {:?}", token);
        }
        let end = turtle.len()..turtle.len();
        let stream = Stream::from_iter(
            end,
            tokens
                .into_iter()
                .map(|Spanned(x, y)| (x, y))
                .rev()
                .filter(|x| !x.0.is_comment()),
        );

        parser
            .then_ignore(chumsky::prelude::end())
            .parse_recovery(stream)
    }
    #[test]
    fn parse_prologue() {
        let inp = r#"
PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
        "#;

        let (q, tok) = parse_it(inp, prologue());

        assert_eq!(tok, vec![]);
        assert!(q.is_some());
    }

    #[test]
    fn parse_select_clause() {
        let inp = r#"
SELECT  ?title ?price
        "#;

        let (q, tok) = parse_it(inp, select_clause());

        println!("q {:?}", q);

        for t in &tok {
            println!("t {:?}", t);
        }

        assert_eq!(tok, vec![]);
        assert!(q.is_some());
    }

    #[test]
    fn parse_triple() {
        let inp = r#"
  ?x ns:discount ?discount .
        "#;

        let (q, tok) = parse_it(inp, triple());

        assert_eq!(tok, vec![]);
        assert!(q.is_some());
    }

    #[test]
    fn parse_group_graph_pattern_sub() {
        let inp = r#"
 ?x ns:price ?p .
        "#;

        let (q, tok) = parse_it(
            inp,
            group_graph_pattern_sub(group_graph_pattern(sub_select())),
        );

        assert_eq!(tok, vec![]);
        assert!(q.is_some());
    }

    #[test]
    fn parse_group_graph_pattern() {
        let inp = r#"{
    ?x ns:price ?p .
}"#;

        let (q, tok) = parse_it(inp, group_graph_pattern(sub_select()));

        assert_eq!(tok, vec![]);
        assert!(q.is_some());
    }

    #[test]
    fn simple_test() {
        let inp = r#"PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
PREFIX  ns:  <http://example.org/ns#>
SELECT  ?title ?price
{ ?x ns:price ?p .
  ?x dc:title ?title . 
  ?x ns:discount ?discount .
}
        "#;

        let (q, tok) = parse_it(
            inp,
            query(lsp_types::Url::parse("memory://myFile.sq").unwrap()),
        );

        assert_eq!(tok, vec![]);
        assert!(q.is_some());
    }
}
