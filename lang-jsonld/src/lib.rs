use std::sync::Arc;

use bevy_ecs::prelude::*;
use chumsky::prelude::Simple;
use futures::lock::Mutex;
use hashbrown::HashMap;
use json_ld::syntax::Value;
use lsp_core::{lang::Lang, model::Spanned, systems::publish_diagnostics};
use lsp_types::SemanticTokenType;
use systems::setup_parse;

use self::parser::Json;

mod contexts;
// mod loader;
// pub mod parent;
pub mod triples;
pub mod parser;
pub mod systems;
pub mod tokenizer;

pub type Cache = Arc<Mutex<HashMap<String, Spanned<Value>>>>;

pub fn setup_world(world: &mut World) {
    world.schedule_scope(lsp_core::Diagnostics, |_, schedule| {
        schedule.add_systems(publish_diagnostics::<JsonLd>);
    });
    setup_parse(world);
}

#[derive(Debug, Component)]
pub struct JsonLd;

impl Lang for JsonLd {
    type Token = lsp_core::token::Token;

    type TokenError = Simple<char>;

    type Element = Json;

    type ElementError = Simple<lsp_core::token::Token>;

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
