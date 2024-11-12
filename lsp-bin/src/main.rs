use bevy_ecs::schedule::Schedule;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use futures::channel::mpsc::unbounded;
use futures::lock::Mutex;
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
use lsp_core::Format;
use lsp_core::Parse;
use std::fs::File;
use std::io;
use std::sync::Arc;
use tower_lsp::LspService;
use tower_lsp::Server;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

fn setup_world<C: Client + ClientSync + Resource + Clone>(client: C) -> Arc<Mutex<World>> {
    let mut world = World::new();

    world.add_schedule(Schedule::new(lsp_core::Tasks));
    world.add_schedule(Schedule::new(st::Label));
    world.add_schedule(Schedule::new(Parse));
    world.add_schedule(Schedule::new(Completion));
    world.add_schedule(Schedule::new(Diagnostics));
    world.add_schedule(Schedule::new(Format));

    use lsp_core::systems::semantic_tokens as st;

    let (publisher, mut rx) = OtherPublisher::new();
    world.insert_resource(publisher);

    let handle = std::thread::current();
    info!("Threaad name {:?}", handle.id());

    let c = client.clone();
    tokio::spawn(async move {
        while let Some(x) = rx.next().await {
            c.publish_diagnostics(x.uri, x.diagnostics, x.version).await;
        }
    });

    lang_turtle::setup_world::<C>(&mut world);

    let (tx, mut rx) = unbounded();
    world.insert_resource(CommandSender(tx));
    // world.insert_resource(CommandReceiver(rx));
    world.insert_resource(client.clone());
    // world.schedule_scope(lsp_core::Tasks, |_, schedule| {
    //     schedule.add_systems(handle_tasks);
    // });

    let out = Arc::new(Mutex::new(world));
    let w = out.clone();
    tokio::spawn(async move {
        while let Some(mut x) = rx.next().await {
            let mut world = w.lock().await;
            world.commands().append(&mut x);
            world.flush_commands();
        }
    });

    out
}

#[tokio::main]
async fn main() {
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
        .with_writer(std::sync::Mutex::new(target))
        .init();

    info!("Hello world!");
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) =
        LspService::build(|client| Backend::new(setup_world(TowerClient::new(client)))).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

