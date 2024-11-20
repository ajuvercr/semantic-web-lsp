mod parse;
use bevy_ecs::{schedule::IntoSystemConfigs as _, world::World};
use lsp_core::{
    systems::{derive_classes, derive_prefix_links, derive_properties},
    Parse,
};
use parse::derive_triples;
pub use parse::{parse_jsonld_system, parse_source};

pub fn setup_parse(world: &mut World) {
    world.schedule_scope(Parse, |_, schedule| {
        schedule.add_systems((
            parse_source,
            parse_jsonld_system.after(parse_source),
            derive_triples
                .after(parse_jsonld_system)
                .before(derive_classes)
                .before(derive_prefix_links)
                .before(derive_properties),
            // fetch_lov_properties::<C>.after(parse_turtle_system),
        ));
    });
}

#[cfg(test)]
mod tests {
    use lsp_core::components::*;
    use test_utils::{create_file, setup_world, TestClient};

    use crate::JsonLd;

    #[test]
    fn parse_workds() {
        let (mut world, _) = setup_world(TestClient::new(), crate::setup_world);

        let t1 = r#"
{
    "@context" : { "foaf": "http://xmlns.com/foaf/0.1/" },
    "@id": "http://example.com/ns#me",
    "foaf:friend": "http://example.com/ns#you"
}"#;
        let entity = create_file(&mut world, t1, "http://example.com/ns#", JsonLd);

        let tokens = world.entity(entity).get::<Tokens>().expect("tokens exists");
        let jsonld = world
            .entity(entity)
            .get::<Element<JsonLd>>()
            .expect("jsonld exists");

        println!("Tokens {:?}", tokens);
        println!("JsonLd {:?}", jsonld);

        assert_eq!(tokens.len(), 17);
    }
}
