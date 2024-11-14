use bevy_ecs::{schedule::IntoSystemConfigs as _, system::Resource, world::World};
use completion::{subject_completion, turtle_prefix_completion};
use formatting::format_turtle_system;
use lsp_core::{
    client::{Client, ClientSync},
    systems::{derive_classes, get_current_token},
    Parse,
};
use parsing::{derive_triples, parse_source, parse_turtle_system};

mod lov;
use lov::fetch_lov_properties;

mod completion;
mod formatting;
mod parsing;

pub fn setup_parsing<C: Client + ClientSync + Resource>(world: &mut World) {
    world.schedule_scope(Parse, |_, schedule| {
        schedule.add_systems((
            parse_source,
            parse_turtle_system.after(parse_source),
            derive_triples.after(parse_turtle_system),
            fetch_lov_properties::<C>.after(parse_turtle_system),
            derive_classes.after(derive_triples),
        ));
    });
}

pub fn setup_formatting(world: &mut World) {
    world.schedule_scope(lsp_core::Format, |_, schedule| {
        schedule.add_systems(format_turtle_system);
    });
}

pub fn setup_completion(world: &mut World) {
    world.schedule_scope(lsp_core::Completion, |_, schedule| {
        schedule.add_systems((
            turtle_prefix_completion.after(get_current_token),
            subject_completion.after(get_current_token),
        ));
    });
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use lsp_core::{components::*, lang::DiagnosticItem, Diagnostics, Parse};
    use ropey::Rope;
    use test_utils::{create_file, setup_world, TestClient};

    use crate::TurtleComponent;

    #[test]
    fn diagnostics_work() {
        let (mut world, mut rx) = setup_world(TestClient::new(), crate::setup_world::<TestClient>);

        let t1 = "
@prefix foaf: <>.
            ";

        let t2 = "
@prefix foaf: <>.
foa:foaf
            ";

        let t3 = "
@prefix foaf: <>.
foa
            ";

        let entity = create_file(&mut world, t1, "http://example.com/ns#", TurtleComponent);
        world.run_schedule(Parse);
        world.run_schedule(Diagnostics);

        let mut get_diagnostics = move || {
            let mut out: Vec<DiagnosticItem> = Vec::new();
            while let Ok(Some(x)) = rx.try_next() {
                out.push(x);
            }
            out
        };
        let items = get_diagnostics();
        assert!(items[0].diagnostics.is_empty());

        world
            .entity_mut(entity)
            .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2))));
        world.run_schedule(Parse);
        world.run_schedule(Diagnostics);

        let items = get_diagnostics();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].diagnostics.len(), 1);

        world
            .entity_mut(entity)
            .insert((Source(t3.to_string()), RopeC(Rope::from_str(t2))));
        world.run_schedule(Parse);
        world.run_schedule(Diagnostics);

        let items = get_diagnostics();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].diagnostics.len(), 2);
    }

    #[test]
    fn fetch_lov_properties_test() {
        let mut client = TestClient::new();
        client.add_res("http://xmlns.com/foaf/0.1/", " @prefix foaf: <>. ");
        let (mut world, _) = setup_world(TestClient::new(), crate::setup_world::<TestClient>);

        let t1 = " @prefix foaf: <http://xmlns.com/foaf/0.1/>.";
        create_file(&mut world, t1, "http://example.com/ns#", TurtleComponent);

        assert_eq!(world.entities().len(), 1);
        let c = world.resource::<TestClient>().clone();
        block_on(c.await_futures(|| world.run_schedule(lsp_core::Tasks)));

        assert_eq!(world.entities().len(), 2);
    }
}
