use bevy_ecs::{
    event::Event,
    schedule::{IntoSystemConfigs as _, Schedule, ScheduleLabel},
    system::{self, Resource},
    world::World,
};
use client::Client;
use components::{SemanticTokensDict, TypeHierarchy};
use systems::{
    basic_semantic_tokens, complete_class, complete_properties, defined_prefix_completion,
    derive_classes, derive_prefix_links, derive_properties, extract_type_hierarchy,
    fetch_lov_properties, get_current_token, get_current_triple, hover_class, hover_property,
    hover_types, infer_types, inlay_triples, prefixes, semantic_tokens_system,
};

pub mod client;
pub mod components;
pub mod lang;
pub mod model;
pub mod ns;
pub mod parent;
pub mod prefix;
pub mod systems;
pub mod token;
pub mod triples;
pub mod utils;

pub fn setup_schedule_labels<C: Client + Resource>(world: &mut World) {
    world.init_resource::<SemanticTokensDict>();
    world.init_resource::<TypeHierarchy<'static>>();

    let mut parse = Schedule::new(Parse);
    parse.add_systems((
        prefixes,
        systems::triples,
        derive_prefix_links.after(prefixes),
        derive_classes.after(systems::triples),
        derive_properties.after(systems::triples),
        fetch_lov_properties::<C>.after(prefixes),
        extract_type_hierarchy.after(systems::triples),
        infer_types.after(systems::triples),
    ));
    world.add_schedule(parse);

    let mut completion = Schedule::new(Completion);
    println!("Setting completion systems");
    completion.add_systems((
        get_current_token,
        get_current_triple.after(get_current_token),
        complete_class.after(get_current_triple),
        complete_properties.after(get_current_triple),
        defined_prefix_completion.after(get_current_token),
    ));
    world.add_schedule(completion);

    let mut hover = Schedule::new(Hover);
    hover.add_systems((
        get_current_token,
        get_current_triple.after(get_current_token),
        hover_types.before(hover_class).before(hover_property).after(get_current_token),
        hover_class.after(get_current_token),
        hover_property.after(get_current_token),
    ));
    world.add_schedule(hover);

    world.add_schedule(Schedule::new(Diagnostics));
    world.add_schedule(Schedule::new(Tasks));
    world.add_schedule(Schedule::new(Format));
    let mut inlay = Schedule::new(Inlay);
    inlay.add_systems(inlay_triples);
    world.add_schedule(inlay);

    let mut semantic_tokens = Schedule::new(systems::SemanticTokensSchedule);
    semantic_tokens.add_systems((
        basic_semantic_tokens,
        semantic_tokens_system.after(basic_semantic_tokens),
    ));
    world.add_schedule(semantic_tokens);
}

#[derive(Event)]
pub struct CreateEvent {
    pub url: lsp_types::Url,
    pub language_id: Option<String>,
}

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Hover;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Parse;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Completion;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Diagnostics;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Tasks;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Format;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Inlay;
