use bevy_ecs::prelude::*;
use chumsky::prelude::Simple;
use lsp_core::{
    client::Client,
    components::{DynLang, SemanticTokensDict},
    features::diagnostic::systems::publish_diagnostics,
    lang::{Lang, LangHelper},
    systems::{basic_semantic_tokens, semantic_tokens_system},
    token::Token,
    utils::Spanned,
    CreateEvent,
};
use lsp_types::SemanticTokenType;
use ropey::Rope;
use systems::{highlight_named_nodes, keyword_highlight, setup_parse};

use self::parser::Json;

pub mod parser;
pub mod systems;
pub mod tokenizer;
pub mod triples;

pub fn setup_world(world: &mut World) {
    let mut semantic_token_dict = world.resource_mut::<SemanticTokensDict>();
    JsonLd::LEGEND_TYPES.iter().for_each(|lt| {
        if !semantic_token_dict.contains_key(lt) {
            let l = semantic_token_dict.0.len();
            semantic_token_dict.insert(lt.clone(), l);
        }
    });
    world.observe(|trigger: Trigger<CreateEvent>, mut commands: Commands| {
        println!("Got create event");
        match &trigger.event().language_id {
            Some(x) if x == "jsonld" => {
                println!(" --> its jsonld");
                commands
                    .entity(trigger.entity())
                    .insert(JsonLd)
                    .insert(DynLang(Box::new(JsonLdHelper)));
                return;
            }
            _ => {}
        }
        // pass
        if trigger.event().url.as_str().ends_with(".jsonld") {
            println!(" --> its jsonld");
            commands
                .entity(trigger.entity())
                .insert(JsonLd)
                .insert(DynLang(Box::new(JsonLdHelper)));
            return;
        }
    });

    world.schedule_scope(lsp_core::systems::SemanticTokensSchedule, |_, schedule| {
        schedule.add_systems((
            highlight_named_nodes
                .before(keyword_highlight)
                .after(basic_semantic_tokens),
            keyword_highlight
                .before(semantic_tokens_system)
                .after(basic_semantic_tokens),
        ));
    });

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

#[derive(Debug)]
pub struct JsonLdHelper;
impl LangHelper for JsonLdHelper {
    fn get_relevant_text(
        &self,
        token: &Spanned<Token>,
        rope: &Rope,
    ) -> (String, std::ops::Range<usize>) {
        let r = token.span();
        match token.value() {
            Token::Str(st, _) => (st.clone(), r.start + 1..r.end - 1),
            _ => (self._get_relevant_text(token, rope), r.clone()),
        }
    }

    fn keyword(&self) -> &[&'static str] {
        &[]
    }
}
