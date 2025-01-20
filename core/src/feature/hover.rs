use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

pub use crate::systems::hover_class;
pub use crate::systems::hover_property;
pub use crate::systems::hover_types;
pub use crate::systems::infer_types;
pub use crate::util::token::get_current_token;
pub use crate::util::triple::get_current_triple;

/// [`ScheduleLabel`] related to the Parse schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Label;

pub fn setup_schedule(world: &mut World) {
    let mut hover = Schedule::new(Label);
    hover.add_systems((
        infer_types,
        get_current_token,
        get_current_triple.after(get_current_token),
        hover_types
            .before(hover_class)
            .before(hover_property)
            .after(get_current_token)
            .after(infer_types),
        hover_class.after(get_current_token),
        hover_property.after(get_current_token),
    ));
    world.add_schedule(hover);
}
