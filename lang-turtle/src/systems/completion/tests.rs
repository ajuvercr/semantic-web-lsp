use chumsky::chain::Chain;
use completion::CompletionRequest;
use futures::executor::block_on;
use lsp_core::{components::*, lang::LangHelper, prelude::*, Tasks};
use ropey::Rope;
use test_log::test;
use test_utils::{create_file, setup_world, TestClient};
use tracing::info;

use crate::{TurtleHelper, TurtleLang};

#[test]
fn completion_event_works() {
    println!("completion_event_works");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world);

    let t1 = "
@prefix foaf: <http://xmlns.com/foaf/0.1/>.
            ";

    let t2 = "
@prefix foaf: <http://xmlns.com/foaf/0.1/>.
foa
            ";

    let entity = create_file(&mut world, t1, "http://example.com/ns#", "turtle", Open);

    world
        .entity_mut(entity)
        .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2))));
    world.run_schedule(ParseLabel);

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 2,
            character: 0,
        }),
    ));

    world.run_schedule(CompletionLabel);
    let m_completions = world.entity_mut(entity).take::<CompletionRequest>();

    assert!(m_completions.is_some());
    let completions = m_completions.unwrap().0;
    assert_eq!(completions.len(), 4 + TurtleHelper.keyword().len());
}

#[test_log::test]
fn completion_event_works_multiple_files() {
    info!("Testing multiple files");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world);
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
        "turtle",
        Open,
    );

    create_file(
        &mut world,
        t2,
        "http://example.com/second_file#",
        "turtle",
        Open,
    );

    world
        .entity_mut(entity)
        .insert((Source(t1_2.to_string()), RopeC(Rope::from_str(t1_2)), Open));
    world.run_schedule(ParseLabel);

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 2,
            character: 0,
        }),
    ));
    world.run_schedule(CompletionLabel);

    let completions = world
        .entity_mut(entity)
        .take::<CompletionRequest>()
        .expect("Completions exists")
        .0;

    assert_eq!(completions.len(), 1 + TurtleHelper.keyword().len());
}

#[test_log::test]
fn test_autocomplete_classes() {
    println!("completion_event_works");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world);

    let t1 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.";

    let t2 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.
<> a foa";

    let entity = create_file(&mut world, t1, "http://example.com/ns#", "turtle", Open);

    let c = world.resource::<TestClient>().clone();
    block_on(c.await_futures(|| world.run_schedule(Tasks)));

    world
        .entity_mut(entity)
        .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2)), Open));
    world.run_schedule(ParseLabel);

    block_on(c.await_futures(|| world.run_schedule(Tasks)));

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 1,
            character: 6,
        }),
    ));
    world.run_schedule(CompletionLabel);
    let completions = world
        .entity_mut(entity)
        .take::<CompletionRequest>()
        .expect("competion request")
        .0;

    for c in &completions {
        println!("c {:?} {:?}\n\n", c.label, c._documentation);
    }
    assert_eq!(
        completions.len(),
        4 /* prefix.cc */ + 13 /*completions */ + TurtleHelper.keyword().len()
    );
}

#[test_log::test]
fn test_autocomplete_properties() {
    println!("completion_event_works");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world);

    let t1 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.";

    let t2 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.
<> foaf:";

    let entity = create_file(&mut world, t1, "http://example.com/ns#", "turtle", Open);

    let c = world.resource::<TestClient>().clone();
    block_on(c.await_futures(|| world.run_schedule(Tasks)));

    world
        .entity_mut(entity)
        .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2)), Open));
    world.run_schedule(ParseLabel);

    block_on(c.await_futures(|| world.run_schedule(Tasks)));

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 1,
            character: 4,
        }),
    ));
    world.run_schedule(CompletionLabel);
    let completions = world
        .entity_mut(entity)
        .take::<CompletionRequest>()
        .expect("competion request")
        .0;

    assert_eq!(completions.len(), 62 + TurtleHelper.keyword().len());
}

#[test_log::test]
fn test_autocomplete_properties_2() {
    println!("completion_event_works");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world);

    let t1 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.
<> a foaf:Person;
    foaf:name \"Arthur\".";

    let t2 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.
<> a foaf:Person;
    foaf:
    foaf:name \"Arthur\".";

    let entity = create_file(&mut world, t1, "http://example.com/ns#", "turtle", Open);

    let c = world.resource::<TestClient>().clone();
    block_on(c.await_futures(|| world.run_schedule(Tasks)));

    world
        .entity_mut(entity)
        .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2)), Open));
    world.run_schedule(ParseLabel);

    block_on(c.await_futures(|| world.run_schedule(Tasks)));

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 2,
            character: 5,
        }),
    ));
    world.run_schedule(CompletionLabel);
    let completions = world
        .entity_mut(entity)
        .take::<CompletionRequest>()
        .expect("competion request")
        .0;

    assert_eq!(completions.len(), 0 + TurtleHelper.keyword().len());
}
