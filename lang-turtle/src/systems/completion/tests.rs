use lsp_core::{components::*, Completion, Parse};
use ropey::Rope;
use test_log::test;
use test_utils::{create_file, setup_world, TestClient};
use tracing::info;

use crate::TurtleLang;

// Note:
// - I had to change how the paring worked and only update the turtle component when it was
// succesful, this had a similar problem without working with a ecs
// This should just be fixed in parsing allowing for errors and trying to extract 'most' of the
// content
#[test]
fn completion_event_works() {
    println!("completion_event_works");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world::<TestClient>);

    let t1 = "
@prefix foaf: <>.
            ";

    let t2 = "
@prefix foaf: <>.
foa
            ";

    let entity = create_file(&mut world, t1, "http://example.com/ns#", TurtleLang);

    world
        .entity_mut(entity)
        .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2))));
    world.run_schedule(Parse);

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 2,
            character: 0,
        }),
    ));
    world.run_schedule(Completion);
    let m_completions = world.entity_mut(entity).take::<CompletionRequest>();

    assert!(m_completions.is_some());
    let completions = m_completions.unwrap().0;
    println!("completions {:?}\n\n", completions);
    assert_eq!(completions.len(), 1);
}

#[test_log::test]
fn completion_event_works_multiple_files() {
    info!("Testing multiple files");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world::<TestClient>);
    let t1_1 = "
@prefix foaf: <http://xmlns.com/foaf/0.1/>.
            ";

    let t1_2 = "
@prefix foaf: <http://xmlns.com/foaf/0.1/>.
foaf:
            ";

    let t2 = "
@prefix foaf: <http://xmlns.com/foaf/0.1/>.

foaf:me foaf:friend <#me>.
            ";

    let entity = create_file(
        &mut world,
        t1_1,
        "http://example.com/first_file#",
        TurtleLang,
    );

    create_file(
        &mut world,
        t2,
        "http://example.com/second_file#",
        TurtleLang,
    );

    world
        .entity_mut(entity)
        .insert((Source(t1_2.to_string()), RopeC(Rope::from_str(t1_2))));
    world.run_schedule(Parse);

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 2,
            character: 0,
        }),
    ));
    world.run_schedule(Completion);

    let completions = world
        .entity_mut(entity)
        .take::<CompletionRequest>()
        .expect("Completions exists")
        .0;

    println!("completions {:?}\n\n", completions);

    assert_eq!(completions.len(), 1);
}
