use bevy_ecs::{schedule::ScheduleLabel, world::World};

pub use crate::systems::{validate_shapes, validate_with_updated_shapes};

/// [`ScheduleLabel`] related to the OnSave schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Label;

pub fn setup_schedule(world: &mut World) {
    let mut on_save = bevy_ecs::schedule::Schedule::new(Label);
    on_save.add_systems((validate_shapes, validate_with_updated_shapes));
    world.add_schedule(on_save);
}
