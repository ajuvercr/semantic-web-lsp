use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

/// [`ScheduleLabel`] related to the Inlay schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Label;
pub fn setup_schedule(world: &mut World) {
    let inlay = bevy_ecs::schedule::Schedule::new(Label);
    // inlay.add_systems(inlay_triples);
    world.add_schedule(inlay);
}
