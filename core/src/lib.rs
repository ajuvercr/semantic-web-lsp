use bevy_ecs::{
    event::Event,
    schedule::{IntoSystemConfigs as _, Schedule, ScheduleLabel},
    system::Resource,
    world::World,
};
use client::Client;
use systems::{
    complete_class, complete_properties, defined_prefix_completion, derive_classes,
    derive_prefix_links, derive_properties, fetch_lov_properties, get_current_token,
    get_current_triple,
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
    let mut parse = Schedule::new(Parse);
    parse.add_systems((
        derive_prefix_links,
        derive_classes,
        derive_properties,
        fetch_lov_properties::<C>,
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

    world.add_schedule(Schedule::new(Diagnostics));
    world.add_schedule(Schedule::new(Tasks));
    world.add_schedule(Schedule::new(Format));
    world.add_schedule(Schedule::new(systems::SemanticTokensSchedule));
}

#[derive(Event)]
pub struct CreateEvent {
    pub url: lsp_types::Url,
    pub language_id: Option<String>,
}

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
