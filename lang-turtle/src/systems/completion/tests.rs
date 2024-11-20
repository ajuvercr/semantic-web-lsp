use futures::executor::block_on;
use lsp_core::{components::*, Completion, Parse};
use ropey::Rope;
use test_log::test;
use test_utils::{create_file, setup_world, TestClient};
use tracing::{debug, info};

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

#[test_log::test]
fn test_autocomplete_classes() {
    println!("completion_event_works");
    let (mut world, _) = setup_world(TestClient::new(), crate::setup_world::<TestClient>);

    let t1 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.";

    let t2 = "@prefix foaf: <http://xmlns.com/foaf/0.1/>.
<> a foa";

    let entity = create_file(&mut world, t1, "http://example.com/ns#", TurtleLang);

    let c = world.resource::<TestClient>().clone();
    block_on(c.await_futures(|| world.run_schedule(lsp_core::Tasks)));

    world
        .entity_mut(entity)
        .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2))));
    world.run_schedule(Parse);

    block_on(c.await_futures(|| world.run_schedule(lsp_core::Tasks)));

    // start call completion
    world.entity_mut(entity).insert((
        CompletionRequest(vec![]),
        PositionComponent(lsp_types::Position {
            line: 1,
            character: 6,
        }),
    ));
    world.run_schedule(Completion);
    let completions = world
        .entity_mut(entity)
        .take::<CompletionRequest>()
        .expect("competion request")
        .0;

    for com in completions.iter() {
        debug!("Comp {}", com.edits[0].new_text);
    }
    assert_eq!(completions.len(), 14);
}

// 2024-11-19T10:12:38.527252Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:OnlineChatAccount
// 2024-11-19T10:12:38.527268Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:OnlineEcommerceAccount
// 2024-11-19T10:12:38.527272Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:OnlineGamingAccount
// 2024-11-19T10:12:38.527277Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:OnlineAccount
// 2024-11-19T10:12:38.527281Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:PersonalProfileDocument
// 2024-11-19T10:12:38.527286Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:Image
// 2024-11-19T10:12:38.527297Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:Project
// 2024-11-19T10:12:38.527301Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:Agent
// 2024-11-19T10:12:38.527306Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:Group
// 2024-11-19T10:12:38.527310Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:Organization
// 2024-11-19T10:12:38.527314Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:Document
// 2024-11-19T10:12:38.527318Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:Person
// 2024-11-19T10:12:38.527322Z DEBUG lang_turtle::systems::completion::tests: Comp foaf:LabelProperty

