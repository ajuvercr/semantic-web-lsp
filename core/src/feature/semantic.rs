use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

pub use crate::systems::basic_semantic_tokens;
pub use crate::systems::semantic_tokens_system;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Label;
pub fn setup_world(world: &mut World) {
    let mut semantic_tokens = bevy_ecs::schedule::Schedule::new(Label);
    semantic_tokens.add_systems((
        basic_semantic_tokens,
        semantic_tokens_system.after(basic_semantic_tokens),
    ));
    world.add_schedule(semantic_tokens);
}
