use chumsky::{Parser as _, Stream};
use lsp_core::util::Spanned;
use lsp_types::Url;
use model::Turtle;
use tokenizer::parse_tokens_str_safe;

pub mod formatter;
pub mod model;
pub mod parser;
// pub mod parser2;
pub mod tokenizer;

pub fn parse_source(url: &Url, string: &str) -> (Option<Turtle>, Vec<String>) {
    let parser = parser::turtle(url);

    let tokens = match parse_tokens_str_safe(string) {
        Ok(t) => t,
        Err(e) => {
            return (None, e.into_iter().map(|x| x.to_string()).collect());
        }
    };

    let end = string.len()..string.len();
    let stream = Stream::from_iter(
        end,
        tokens
            .into_iter()
            .map(|Spanned(x, y)| (x, y))
            .rev()
            .filter(|x| !x.0.is_comment()),
    );

    let (t, es) = parser.parse_recovery(stream);
    (t, es.into_iter().map(|x| x.to_string()).collect())
}
