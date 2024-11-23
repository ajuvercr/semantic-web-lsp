use std::marker::PhantomData;
use std::marker::PhantomPinned;

use chumsky::prelude::*;
use chumsky::Error;
use chumsky::Parser;
use lsp_core::token::SparqlCall;
use lsp_core::token::SparqlExpr;
use text::ident;

pub fn parse_sparql_expr_token() -> impl Parser<char, SparqlExpr, Error = Simple<char>> {
    ident::<char, Simple<char>>().try_map(|x, span| match x.to_lowercase().as_str() {
        "||" => Ok(SparqlExpr::Or),
        "&&" => Ok(SparqlExpr::And),
        "=" => Ok(SparqlExpr::Equal),
        "!=" => Ok(SparqlExpr::NotEqual),
        "<" => Ok(SparqlExpr::Lt),
        "<=" => Ok(SparqlExpr::Lte),
        ">" => Ok(SparqlExpr::Gt),
        ">=" => Ok(SparqlExpr::Gte),
        "in" => Ok(SparqlExpr::In),
        "not" => Ok(SparqlExpr::Not),
        "+" => Ok(SparqlExpr::Plus),
        "-" => Ok(SparqlExpr::Minus),
        "*" => Ok(SparqlExpr::Times),
        "/" => Ok(SparqlExpr::Divide),
        "!" => Ok(SparqlExpr::Exclamation),
        _ => Err(Simple::expected_input_found(span, [], None)),
    })
}

pub fn parse_sparql_call_token() -> impl Parser<char, SparqlCall, Error = Simple<char>> {
    ident::<char, Simple<char>>().try_map(|x, span| match x.to_lowercase().as_str() {
        "str" => Ok(SparqlCall::Str),
        "lang" => Ok(SparqlCall::Lang),
        "langmatches" => Ok(SparqlCall::LangMatches),
        "langdir" => Ok(SparqlCall::LangDir),
        "datatype" => Ok(SparqlCall::Datatype),
        "bound" => Ok(SparqlCall::Bound),
        "iri" => Ok(SparqlCall::Iri),
        "uri" => Ok(SparqlCall::Uri),
        "bnode" => Ok(SparqlCall::Bnode),
        "rand" => Ok(SparqlCall::Rand),
        "abs" => Ok(SparqlCall::Abs),
        "ceil" => Ok(SparqlCall::Ceil),
        "floor" => Ok(SparqlCall::Floor),

        "!" => Ok(SparqlExpr::Exclamation),
        _ => Err(Simple::expected_input_found(span, [], None)),
    })
}
