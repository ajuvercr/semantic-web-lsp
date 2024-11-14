use bevy_ecs::{
    schedule::{IntoSystemConfigs as _, Schedule, ScheduleLabel},
    world::World,
};
use systems::{get_current_token, get_current_triple};

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

pub fn setup_schedule_labels(world: &mut World) {
    world.add_schedule(Schedule::new(Parse));

    let mut completion = Schedule::new(Completion);
    completion.add_systems((
        get_current_token,
        get_current_triple.after(get_current_token),
    ));
    world.add_schedule(completion);

    world.add_schedule(Schedule::new(Diagnostics));
    world.add_schedule(Schedule::new(Tasks));
    world.add_schedule(Schedule::new(Format));
    world.add_schedule(Schedule::new(systems::SemanticTokensSchedule));
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
