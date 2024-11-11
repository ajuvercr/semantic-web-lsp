use bevy_ecs::schedule::ScheduleLabel;

pub mod client;
pub mod components;
pub mod lang;
pub mod model;
pub mod ns;
pub mod parent;
pub mod prefix;
pub mod semantics;
pub mod systems;
pub mod utils;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Parse;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Completion;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Diagnostics;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Tasks;
