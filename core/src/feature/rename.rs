use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

pub use crate::systems::prepare_rename;
pub use crate::systems::rename;
pub use crate::util::token::get_current_token;

/// [`ScheduleLabel`] related to the PrepareRename schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PrepareRename;

/// [`ScheduleLabel`] related to the Rename schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Rename;

pub fn setup_schedules(world: &mut World) {
    let mut prepare_rename_schedule = Schedule::new(PrepareRename);
    prepare_rename_schedule
        .add_systems((get_current_token, prepare_rename.after(get_current_token)));
    world.add_schedule(prepare_rename_schedule);

    let mut rename_schedule = Schedule::new(Rename);
    rename_schedule.add_systems((get_current_token, rename.after(get_current_token)));
    world.add_schedule(rename_schedule);
}
