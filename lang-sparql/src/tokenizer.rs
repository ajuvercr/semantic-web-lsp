use chumsky::prelude::*;
use lsp_core::{
    model::{spanned, Spanned},
    token::Token,
};
use token_helpers::*;

pub fn parse_token() -> t!(Token) {
    choice((
        keywords(),
        comment(),
        iri_ref(),
        pname_ns(),
        blank_node_label(),
        lang_tag(),
        integer(),
        strings(),
        tokens(),
    ))
    .recover_with(skip_parser(invalid()))
}

pub fn parser() -> t!(Vec<Spanned<Token>>) {
    parse_token()
        .map_with_span(spanned)
        .padded()
        .repeated()
        .then_ignore(end().recover_with(skip_then_retry_until([])))
}

pub fn tokenize(st: &str) -> (Vec<Spanned<Token>>, Vec<Simple<char>>) {
    let parser = parser()
        .then_ignore(end().recover_with(skip_then_retry_until([])))
        .padded();

    let (json, errs) = parser.parse_recovery(st);

    (json.unwrap_or_default(), errs)
}
