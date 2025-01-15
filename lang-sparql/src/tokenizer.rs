use std::{ops::Range, str::FromStr as _};

use chumsky::{prelude::*, Error};
use lsp_core::{
    prelude::{spanned, Spanned},
    token::{Membered, SparqlAggregate, SparqlCall, SparqlExpr, SparqlKeyword, Token},
};
use text::ident;
use token_helpers::*;

fn keywords() -> t!(Token) {
    let sparql_expr = |st: &str, span: &Range<usize>| {
        SparqlExpr::from_str(st)
            .map(Token::SparqlExpr)
            .map_err(|_| {
                Simple::custom(
                    span.clone(),
                    format!("Expected one of {:?}", SparqlExpr::ITEMS),
                )
            })
    };

    let sparql_call = |st: &str, span: &Range<usize>| {
        SparqlCall::from_str(st)
            .map(Token::SparqlCall)
            .map_err(|_| {
                Simple::custom(
                    span.clone(),
                    format!("Expected one of {:?}", SparqlCall::ITEMS),
                )
            })
    };

    let sparql_aggregate = |st: &str, span: &Range<usize>| {
        SparqlAggregate::from_str(st)
            .map(Token::SparqlAggregate)
            .map_err(|_| {
                Simple::custom(
                    span.clone(),
                    format!("Expected one of {:?}", SparqlAggregate::ITEMS),
                )
            })
    };

    let sparql_keyword = |st: &str, span: &Range<usize>| {
        SparqlKeyword::from_str(st)
            .map(Token::SparqlKeyword)
            .map_err(|_| {
                Simple::custom(
                    span.clone(),
                    format!("Expected one of {:?}", SparqlKeyword::ITEMS),
                )
            })
    };

    // these aren't found with ident()
    let extras = choice::<_, Simple<char>>((
        just::<char, &str, Simple<char>>("<"),
        just::<char, &str, Simple<char>>(">"),
        just::<char, &str, Simple<char>>("<="),
        just::<char, &str, Simple<char>>(">="),
        just::<char, &str, Simple<char>>("||"),
        just::<char, &str, Simple<char>>("&&"),
        just::<char, &str, Simple<char>>("+"),
        just::<char, &str, Simple<char>>("-"),
        just::<char, &str, Simple<char>>("*"),
        just::<char, &str, Simple<char>>("/"),
        just::<char, &str, Simple<char>>("!"),
    ))
    .map(String::from);

    ident::<char, Simple<char>>()
        .or(extras)
        .try_map(move |x, span| {
            sparql_expr(x.as_str(), &span)
                .or_else(|e| sparql_call(x.as_str(), &span).map_err(|e2| e2.merge(e)))
                .or_else(|e| sparql_aggregate(x.as_str(), &span).map_err(|e2| e2.merge(e)))
                .or_else(|e| sparql_keyword(x.as_str(), &span).map_err(|e2| e2.merge(e)))
        })
}

fn parse_variable() -> t!(Token) {
    one_of(['?', '$'])
        .ignore_then(varname().repeated().at_least(1))
        .collect()
        .map(Token::Variable)
}

pub fn parse_token() -> t!(Token) {
    choice((
        comment(),
        iri_ref(),
        parse_variable(),
        pname_ns(),
        blank_node_label(),
        integer(),
        strings(),
        tokens(),
        tokens_ext(),
        keywords(),
    ))
    .recover_with(skip_parser(invalid()))
}

pub fn parser() -> t!(Vec<Spanned<Token>>) {
    parse_token().map_with_span(spanned).padded().repeated()
}

pub fn tokenize(st: &str) -> (Vec<Spanned<Token>>, Vec<Simple<char>>) {
    let parser = parser().then_ignore(end().recover_with(skip_then_retry_until([])));

    let (json, errs) = parser.parse_recovery(st);

    (json.unwrap_or_default(), errs)
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn parse_random_tokens_1() {
        let inp = r#"
PREFIX ent:  <http://org.example.com/employees#>
DESCRIBE ?x WHERE { ?x ent:employeeId "1234" }
        "#;

        let (tok, er) = tokenize(inp);
        assert_eq!(tok.len(), 11);
        assert_eq!(er, vec![]);
    }

    #[test]
    fn parse_random_tokens_2() {
        let inp = r#"
PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
SELECT  ?title
WHERE   { 
    ?x dc:title ?title
    FILTER regex(?title, "^SPARQL") 
}
        "#;

        let (tok, er) = tokenize(inp);
        assert_eq!(tok.len(), 18);
        assert_eq!(er, vec![]);
    }

    #[test]
    fn parse_random_tokens_3() {
        let inp = r#"
PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
PREFIX  ns:  <http://example.org/ns#>

SELECT  ?title ?price
WHERE   {
    ?x ns:price ?price .
    FILTER (?price < 30.5)
    ?x dc:title ?title . 
}
        "#;

        let (tok, er) = tokenize(inp);
        assert_eq!(tok.len(), 26);
        assert_eq!(er, vec![]);
    }

    #[test]
    fn parse_random_tokens_4() {
        let inp = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?name ?mbox
WHERE  {
    ?x foaf:name  ?name .
    OPTIONAL { ?x  foaf:mbox  ?mbox }
}
        "#;

        let (tok, er) = tokenize(inp);
        assert_eq!(tok.len(), 19);
        assert_eq!(er, vec![]);
    }

    #[test]
    fn parse_random_tokens_5() {
        let inp = r#"
PREFIX foaf:    <http://xmlns.com/foaf/0.1/>
ASK  {
   ?x foaf:name  "Alice" ;
      foaf:mbox  <mailto:alice@work.example>
}
        "#;

        let (tok, er) = tokenize(inp);
        assert_eq!(tok.len(), 12);
        assert_eq!(er, vec![]);
    }

    #[test]
    fn parse_random_tokens_6() {
        let inp = r#"
PREFIX  dc: <http://purl.org/dc/elements/1.1/>
PREFIX app: <http://example.org/ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

CONSTRUCT { ?s ?p ?o } WHERE
{
    GRAPH ?g { ?s ?p ?o } .
    ?g dc:publisher <http://www.w3.org/> .
    ?g dc:date ?date .
    FILTER ( app:customDate(?date) > "2005-02-28T00:00:00Z"^^xsd:dateTime ) .
}
        "#;

        let (tok, er) = tokenize(inp);
        assert_eq!(tok.len(), 46);
        assert_eq!(er, vec![]);
    }

    #[test]
    fn parse_random_tokens_7() {
        let inp = r#"
PREFIX foaf:    <http://xmlns.com/foaf/0.1/>
PREFIX vcard:   <http://www.w3.org/2001/vcard-rdf/3.0#>

CONSTRUCT {
     ?x  vcard:N _:v .
    _:v vcard:givenName ?gname .
    _:v vcard:familyName ?fname
} WHERE {
    { ?x foaf:firstname ?gname } UNION  { ?x foaf:givenname   ?gname } .
    { ?x foaf:surname   ?fname } UNION  { ?x foaf:family_name ?fname } .
}
        "#;

        let (tok, er) = tokenize(inp);
        assert_eq!(tok.len(), 47);
        assert_eq!(er, vec![]);
    }

    #[test]
    fn parse_random_tokens_8() {
        let inp = r#"
PREFIX  dc:  <http://purl.org/dc/elements/1.1/>
PREFIX  ns:  <http://example.org/ns#>
SELECT  ?title (?p*(1-?discount) AS ?price)
{ ?x ns:price ?p .
  ?x dc:title ?title . 
  [] ns:discount ?discount 
}
        "#;

        let (tok, er) = tokenize(inp);
        for t in &tok {
            println!("t {:?}", t);
        }
        assert_eq!(tok.len(), 33);
        assert_eq!(er, vec![]);
    }
}
