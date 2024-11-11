use bevy_ecs::schedule::Schedule;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use futures::channel::mpsc::unbounded;
use futures::StreamExt as _;
use lsp_bin::backend::Backend;
use lsp_bin::TowerClient;
use lsp_core::client::Client;
use lsp_core::client::ClientSync;
use lsp_core::components::CommandReceiver;
use lsp_core::components::CommandSender;
use lsp_core::lang::OtherPublisher;
use lsp_core::systems::handle_tasks;
use lsp_core::Completion;
use lsp_core::Diagnostics;
use lsp_core::Parse;
use std::fs::File;
use std::io;
use std::sync::Mutex;
use tower_lsp::LspService;
use tower_lsp::Server;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

fn setup_world<C: Client + ClientSync + Resource + Clone>(client: C) -> World {
    let mut world = World::new();

    world.add_schedule(Schedule::new(lsp_core::Tasks));
    world.add_schedule(Schedule::new(st::Label));
    world.add_schedule(Schedule::new(Parse));
    world.add_schedule(Schedule::new(Completion));
    world.add_schedule(Schedule::new(Diagnostics));

    let (tx, rx) = unbounded();
    world.insert_resource(CommandSender(tx));
    world.insert_resource(CommandReceiver(rx));
    world.insert_resource(client.clone());

    world.schedule_scope(lsp_core::Tasks, |_, schedule| {
        schedule.add_systems(handle_tasks);
    });

    use lsp_core::systems::semantic_tokens as st;

    let (publisher, mut rx) = OtherPublisher::new();
    world.insert_resource(publisher);

    let c = client.clone();
    tokio::spawn(async move {
        while let Some(x) = rx.next().await {
            c.publish_diagnostics(x.uri, x.diagnostics, x.version).await;
        }
    });

    lang_turtle::setup_world::<C>(&mut world);

    world
}

#[tokio::main]
async fn main() {
    println!("Hello world");
    let target: Box<dyn io::Write + Send + Sync> = match File::create("/tmp/turtle-lsp.txt") {
        Ok(x) => Box::new(x),
        Err(_) => Box::new(std::io::stdout()),
    };

    std::panic::set_hook(Box::new(|_panic_info| {
        let backtrace = std::backtrace::Backtrace::capture();
        info!("My backtrace: {:#?}", backtrace);
    }));
    fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(Level::INFO)
        .with_writer(Mutex::new(target))
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) =
        LspService::build(|client| Backend::new(setup_world(TowerClient::new(client)))).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

// fn main() {
//     use bevy_ecs::prelude::*;
//     use lang_turtle::testing::*;
//     let mut world = World::new();
//
//     let schedule = Schedule::new(Parse);
//
//     world.add_schedule(schedule);
//     world.schedule_scope(Parse, |_, schedule| {
//         schedule.add_systems((
//             parse_source,
//             parse_turtle_system.after(parse_source),
//             // notify_parsed.after(parse_turtle_system),
//         ));
//     });
//
//     world.spawn((
//         TurtleComponent,
//         Source("@prefix foaf: <>.".to_string()),
//         Label("http://example.com/ns#".to_string()),
//     ));
//
//     world.flush();
//     println!("Here");
//
//     world.run_schedule(Parse);
//     world.run_schedule(Parse);
//
//     world.flush();
// }
