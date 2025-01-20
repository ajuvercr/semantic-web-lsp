use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;

use crate::client::Client;

pub use crate::systems::derive_classes;
pub use crate::systems::derive_prefix_links;
pub use crate::systems::derive_properties;
pub use crate::systems::derive_shapes;
pub use crate::systems::extract_type_hierarchy;
pub use crate::systems::fetch_lov_properties;
pub use crate::systems::infer_types;

/// Parse schedule barrier, after this system, triples should be derived
pub fn triples() {}
/// Parse schedule barrier, after this system, prefixes should be derived
pub fn prefixes() {}

/// [`ScheduleLabel`] related to the Parse schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Label;

pub fn setup_schedule<C: Client + Resource>(world: &mut World) {
    let mut parse_schedule = bevy_ecs::schedule::Schedule::new(Label);
    parse_schedule.add_systems((
        prefixes,
        triples,
        derive_prefix_links.after(prefixes),
        derive_classes.after(triples),
        derive_properties.after(triples),
        fetch_lov_properties::<C>.after(prefixes),
        extract_type_hierarchy.after(triples),
        infer_types.after(triples),
        derive_shapes.after(triples),
    ));
    world.add_schedule(parse_schedule);
}
