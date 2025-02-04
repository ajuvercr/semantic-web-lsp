#![doc(html_logo_url = "https://ajuvercr.github.io/semantic-web-lsp/assets/icons/favicon.png", html_favicon_url = "https://ajuvercr.github.io/semantic-web-lsp/assets/icons/favicon.ico")]
use bevy_ecs::{component::Component, observer::Trigger, system::Commands, world::World};
use chumsky::prelude::Simple;
use lsp_core::{
    feature::diagnostics::publish_diagnostics,
    lang::{Lang, LangHelper},
    prelude::*,
    CreateEvent,
};
use lsp_types::SemanticTokenType;

pub mod ecs;
pub mod lang;

use crate::ecs::{setup_completion, setup_formatting, setup_parsing};

#[derive(Component)]
pub struct TurtleLang;

#[derive(Debug)]
pub struct TurtleHelper;
impl LangHelper for TurtleHelper {
    fn keyword(&self) -> &[&'static str] {
        &["@prefix", "@base", "a"]
    }
}

pub fn setup_world(world: &mut World) {
    let mut semantic_token_dict = world.resource_mut::<SemanticTokensDict>();
    TurtleLang::LEGEND_TYPES.iter().for_each(|lt| {
        if !semantic_token_dict.contains_key(lt) {
            let l = semantic_token_dict.0.len();
            semantic_token_dict.insert(lt.clone(), l);
        }
    });

    world.observe(|trigger: Trigger<CreateEvent>, mut commands: Commands| {
        println!("Turtle got create event");
        match &trigger.event().language_id {
            Some(x) if x == "turtle" => {
                println!(" --> its turtle");
                commands
                    .entity(trigger.entity())
                    .insert((TurtleLang, DynLang(Box::new(TurtleHelper))));
                return;
            }
            _ => {}
        }
        // pass
        if trigger.event().url.as_str().ends_with(".ttl") {
            println!(" --> its turtle");
            commands
                .entity(trigger.entity())
                .insert((TurtleLang, DynLang(Box::new(TurtleHelper))));
            return;
        }
    });

    world.schedule_scope(lsp_core::feature::DiagnosticsLabel, |_, schedule| {
        schedule.add_systems(publish_diagnostics::<TurtleLang>);
    });

    setup_parsing(world);
    setup_completion(world);
    setup_formatting(world);
}

impl Lang for TurtleLang {
    type Token = Token;

    type TokenError = Simple<char>;

    type Element = crate::lang::model::Turtle;

    type ElementError = (usize, Simple<Token>);

    const LANG: &'static str = "turtle";

    const TRIGGERS: &'static [&'static str] = &[":"];
    const CODE_ACTION: bool = true;
    const HOVER: bool = true;

    const LEGEND_TYPES: &'static [lsp_types::SemanticTokenType] = &[
        semantic_token::BOOLEAN,
        semantic_token::LANG_TAG,
        SemanticTokenType::COMMENT,
        SemanticTokenType::ENUM_MEMBER,
        SemanticTokenType::ENUM,
        SemanticTokenType::KEYWORD,
        SemanticTokenType::NAMESPACE,
        SemanticTokenType::NUMBER,
        SemanticTokenType::PROPERTY,
        SemanticTokenType::STRING,
        SemanticTokenType::VARIABLE,
    ];

    const PATTERN: Option<&'static str> = None;
}
