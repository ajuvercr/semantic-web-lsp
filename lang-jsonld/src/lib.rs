use std::sync::Arc;

use chumsky::prelude::Simple;
use futures::lock::Mutex;
use hashbrown::HashMap;
use json_ld_syntax::context::Value;
use locspan::Span;
use lsp_core::lang::Lang;
use lsp_types::SemanticTokenType;

use self::{parser::Json, tokenizer::JsonToken};

mod contexts;
mod loader;
pub mod parent;
pub mod parser;
pub mod tokenizer;

pub type Cache = Arc<Mutex<HashMap<String, Value<Span>>>>;

#[derive(Debug)]
pub struct JsonLd;

impl Lang for JsonLd {
    type Token = JsonToken;

    type TokenError = Simple<char>;

    type Element = Json;

    type ElementError = Simple<JsonToken>;

    const PATTERN: Option<&'static str> = None;

    const LANG: &'static str = "jsonld";
    const CODE_ACTION: bool = false;
    const HOVER: bool = true;

    const TRIGGERS: &'static [&'static str] = &["@", "\""];
    const LEGEND_TYPES: &'static [SemanticTokenType] = &[
        SemanticTokenType::VARIABLE,
        SemanticTokenType::STRING,
        SemanticTokenType::NUMBER,
        SemanticTokenType::KEYWORD,
        SemanticTokenType::PROPERTY,
        SemanticTokenType::ENUM_MEMBER,
    ];
}

#[cfg(test)]
mod tests {
    use iref::{Iri, IriRefBuf};

    #[test]
    fn test_iri_resolve() {
        let resolved: Result<_, iref::Error> = (|| {
            let base_iri = Iri::new("http://a/b/c/d;p?q")?;
            let iri_ref = IriRefBuf::new("tetten")?;

            Ok(iri_ref.resolved(base_iri))
        })();

        assert!(resolved.is_ok());
        let resolved = resolved.unwrap();
        assert_eq!(resolved, "http://a/b/c/tetten");
    }

    #[test]
    fn test_iri_resolve_abs() {
        let resolved: Result<_, iref::Error> = (|| {
            let base_iri = Iri::new("http://a/b/c/d;p?q")?;
            let iri_ref = IriRefBuf::new("http://tetten.com")?;

            Ok(iri_ref.resolved(base_iri))
        })();

        assert!(resolved.is_ok());
        let resolved = resolved.unwrap();
        assert_eq!(resolved, "http://tetten.com");
    }
}
