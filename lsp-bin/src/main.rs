use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use futures::channel::mpsc::unbounded;
use futures::StreamExt as _;
use lsp_bin::backend::Backend;
use lsp_bin::TowerClient;
use lsp_core::client::Client;
use lsp_core::client::ClientSync;
use lsp_core::components::CommandSender;
use lsp_core::lang::OtherPublisher;
use lsp_core::setup_schedule_labels;
use std::fs::File;
use std::io;
use tower_lsp::LspService;
use tower_lsp::Server;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

fn setup_world<C: Client + ClientSync + Resource + Clone>(client: C) -> CommandSender {
    let mut world = World::new();

    setup_schedule_labels::<C>(&mut world);

    let (publisher, mut rx) = OtherPublisher::new();
    world.insert_resource(publisher);

    let c = client.clone();
    tokio::spawn(async move {
        while let Some(x) = rx.next().await {
            c.publish_diagnostics(x.uri, x.diagnostics, x.version).await;
        }
    });

    lang_turtle::setup_world::<C>(&mut world);
    lang_jsonld::setup_world::<C>(&mut world);

    let (tx, mut rx) = unbounded();
    let sender = CommandSender(tx);
    world.insert_resource(sender.clone());
    world.insert_resource(client.clone());

    tokio::spawn(async move {
        while let Some(mut x) = rx.next().await {
            world.commands().append(&mut x);
            world.flush_commands();
        }
    });

    sender
}

fn get_level() -> Level {
    match std::env::var_os("LOG")
        .and_then(|x| x.into_string().ok())
        .map(|x| x.to_lowercase())
    {
        Some(x) if &x == "info" => Level::INFO,
        Some(x) if &x == "debug" => Level::DEBUG,
        Some(x) if &x == "trace" => Level::TRACE,
        _ => Level::INFO,
    }
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
        .with_max_level(get_level())
        .with_writer(std::sync::Mutex::new(target))
        .init();

    info!("Hello world!");
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| {
        Backend::new(setup_world(TowerClient::new(client.clone())), client)
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
