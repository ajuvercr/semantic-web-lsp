mod formatter;
mod model;
mod parser2;
mod systems;
mod utils;

use bevy_ecs::component::Component;
use bevy_ecs::observer::Trigger;
use bevy_ecs::system::Commands;
use bevy_ecs::world::World;
use lsp_core::components::{DynLang, SemanticTokensDict};
use lsp_core::features::diagnostic::systems::publish_diagnostics;
use lsp_core::token::semantic_token;
use lsp_core::CreateEvent;
pub use parser2::parse_turtle;
// pub mod shacl;
pub use lsp_core::token;
pub mod tokenizer;
use lsp_types::SemanticTokenType;
use systems::{setup_completion, setup_formatting, setup_parsing};

use chumsky::prelude::Simple;
pub use model::*;

pub use parser2::*;

use lsp_core::lang::{Lang, LangHelper};

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

    world.schedule_scope(lsp_core::Diagnostics, |_, schedule| {
        schedule.add_systems(publish_diagnostics::<TurtleLang>);
    });

    setup_parsing(world);
    setup_completion(world);
    setup_formatting(world);
}

impl Lang for TurtleLang {
    type Token = token::Token;

    type TokenError = Simple<char>;

    type Element = model::Turtle;

    type ElementError = (usize, Simple<token::Token>);

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
