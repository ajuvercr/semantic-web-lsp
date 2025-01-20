use bevy_ecs::{schedule::ScheduleLabel, world::World};
/// [`ScheduleLabel`] related to the Format schedule, this is language specific
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Label;

pub fn setup_schedule(world: &mut World) {
    let format = bevy_ecs::schedule::Schedule::new(Label);
    // inlay.add_systems(inlay_triples);
    world.add_schedule(format);
}
