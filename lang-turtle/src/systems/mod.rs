use bevy_ecs::{
    schedule::IntoSystemConfigs as _,
    system::{Query, Resource},
    world::World,
};
use completion::{
    complete_class, subject_completion, turtle_lov_prefix_completion, turtle_prefix_completion,
};
use formatting::format_turtle_system;
use lsp_core::{
    client::{Client, ClientSync},
    systems::{derive_classes, get_current_token, get_current_triple},
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
            derive_prefix_links.after(parse_turtle_system),
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
            turtle_lov_prefix_completion.after(get_current_token),
            turtle_prefix_completion.after(get_current_token),
            subject_completion.after(get_current_token),
            complete_class.after(get_current_triple),
        ));
    });
}

use bevy_ecs::prelude::*;
use lsp_core::components::*;

use crate::TurtleLang;
fn derive_prefix_links(
    mut query: Query<
        (Entity, &Element<TurtleLang>, Option<&mut DocumentLinks>),
        Changed<Element<TurtleLang>>,
    >,
    mut commands: Commands,
) {
    const SOURCE: &'static str = "prefix import";
    for (e, turtle, mut links) in &mut query {
        let new_links: Vec<_> = turtle
            .prefixes
            .iter()
            .flat_map(|p| p.value.expand(&turtle))
            .flat_map(|n| lsp_types::Url::parse(&n))
            .map(|u| (u, SOURCE))
            .collect();
        if let Some(links) = links.as_mut() {
            links.retain(|e| e.1 != SOURCE);
        }
        match (new_links.is_empty(), links) {
            (false, None) => {
                commands.entity(e).insert(DocumentLinks(new_links));
            }
            (false, Some(mut links)) => {
                links.extend(new_links);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use chumsky::chain::Chain;
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
        assert_eq!(items[0].diagnostics.len(), 2);
        let msgs: Vec<_> = items[0].diagnostics.iter().map(|x| &x.message).collect();
        println!("t2 Diagnostics {:?}", msgs);

        world
            .entity_mut(entity)
            .insert((Source(t3.to_string()), RopeC(Rope::from_str(t2))));
        world.run_schedule(Parse);
        world.run_schedule(Diagnostics);

        let items = get_diagnostics();
        let msgs: Vec<_> = items[0].diagnostics.iter().map(|x| &x.message).collect();
        println!("t2 Diagnostics {:?}", msgs);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].diagnostics.len(), 5);
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

    #[test]
    fn turtle_does_prefix_links() {
        let (mut world, _) = setup_world(TestClient::new(), crate::setup_world::<TestClient>);

        let t1 = " @prefix foaf: <http://xmlns.com/foaf/0.1/>.";
        let entity = create_file(&mut world, t1, "http://example.com/ns#", TurtleComponent);

        let links: &DocumentLinks = world.entity(entity).get().expect("document links exists");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0.as_str(), "http://xmlns.com/foaf/0.1/");
        assert_eq!(links[0].1, "prefix import");
    }
}
