use bevy_ecs::{
    component::Component,
    schedule::{IntoSystemConfigs, Schedule, ScheduleLabel},
    world::World,
};

pub use crate::{
    systems::{prepare_rename, rename},
    util::token::get_current_token,
};

/// [`Component`] indicating that the current document is currently handling a PrepareRename request.
#[derive(Component, Debug)]
pub struct PrepareRenameRequest {
    pub range: lsp_types::Range,
    pub placeholder: String,
}

/// [`ScheduleLabel`] related to the PrepareRename schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PrepareRename;

/// [`Component`] indicating that the current document is currently handling a Rename request,
/// collecting [TextEdits](`lsp_types::TextEdit`).
#[derive(Component, Debug)]
pub struct RenameEdits(pub Vec<(lsp_types::Url, lsp_types::TextEdit)>, pub String);

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
