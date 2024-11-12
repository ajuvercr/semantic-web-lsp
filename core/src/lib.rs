use bevy_ecs::{
    schedule::{Schedule, ScheduleLabel},
    world::World,
};

pub mod client;
pub mod components;
pub mod lang;
pub mod model;
pub mod ns;
pub mod parent;
pub mod prefix;
pub mod systems;
pub mod triples;
pub mod utils;

pub fn setup_schedule_labels(world: &mut World) {
    world.add_schedule(Schedule::new(Parse));
    world.add_schedule(Schedule::new(Completion));
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
