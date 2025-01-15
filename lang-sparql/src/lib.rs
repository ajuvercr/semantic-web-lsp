#[macro_use]
extern crate tracing;

use bevy_ecs::prelude::*;
use chumsky::error::Simple;
use lsp_core::{
    client::Client, components::{DynLang, SemanticTokensDict}, features::diagnostic::systems::publish_diagnostics, lang::{Lang, LangHelper}, token::Membered, CreateEvent
};
use lsp_types::SemanticTokenType;
use model::Query;
use systems::{setup_completion, setup_parse};

pub mod parsing;
pub mod tokenizer;

pub mod model;
pub mod systems;

pub fn setup_world(world: &mut World) {
    let mut semantic_token_dict = world.resource_mut::<SemanticTokensDict>();
    [SemanticTokenType::VARIABLE].iter().for_each(|lt| {
        if !semantic_token_dict.contains_key(lt) {
            let l = semantic_token_dict.0.len();
            semantic_token_dict.insert(lt.clone(), l);
        }
    });
    world.observe(|trigger: Trigger<CreateEvent>, mut commands: Commands| {
        println!("Got create event");
        match &trigger.event().language_id {
            Some(x) if x == "sparql" => {
                println!(" --> its sparql");
                commands
                    .entity(trigger.entity())
                    .insert(Sparql)
                    .insert(DynLang(Box::new(SparqlHelper)));
                return;
            }
            _ => {}
        }

        if trigger.event().url.as_str().ends_with(".sq") {
            println!(" --> its sparql");
            commands
                .entity(trigger.entity())
                .insert(Sparql)
                .insert(DynLang(Box::new(SparqlHelper)));
            return;
        }
    });

    world.schedule_scope(lsp_core::Diagnostics, |_, schedule| {
        schedule.add_systems(publish_diagnostics::<Sparql>);
    });

    setup_parse(world);

    setup_completion(world);
}

#[derive(Debug, Component)]
pub struct Sparql;

impl Lang for Sparql {
    type Token = lsp_core::token::Token;

    type TokenError = Simple<char>;

    type Element = Query;

    type ElementError = (usize, Simple<lsp_core::token::Token>);

    const PATTERN: Option<&'static str> = None;

    const LANG: &'static str = "sparql";
    const CODE_ACTION: bool = false;
    const HOVER: bool = true;

    const TRIGGERS: &'static [&'static str] = &[];
    const LEGEND_TYPES: &'static [SemanticTokenType] = &[
        SemanticTokenType::VARIABLE,
        SemanticTokenType::STRING,
        SemanticTokenType::NUMBER,
        SemanticTokenType::KEYWORD,
        SemanticTokenType::PROPERTY,
        SemanticTokenType::ENUM_MEMBER,
    ];
}

lazy_static::lazy_static! {
    static ref KWDS: Vec<&'static str> = {
        let mut m = Vec::new();

        // lsp_core::token::SparqlCall::ITEMS.iter().for_each(|x| m.push(x.complete()));
        lsp_core::token::SparqlKeyword::ITEMS.iter().for_each(|x| m.push(x.complete()));
        lsp_core::token::SparqlAggregate::ITEMS.iter().for_each(|x| m.push(x.complete()));

        m
    };
}

#[derive(Debug)]
pub struct SparqlHelper;

impl LangHelper for SparqlHelper {
    fn keyword(&self) -> &[&'static str] {
        &KWDS
    }
}
