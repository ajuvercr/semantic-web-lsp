use bevy_ecs::schedule::ScheduleLabel;
// use lang_turtle::TurtleLang;
// use lsp_bin::backend::Backend;
use lsp_core::lang::Label;
use lsp_core::lang::Source;
use lsp_core::Parse;
// use lsp_core::prefix::Prefixes;
// use std::fs::File;
// use std::io;
// use std::sync::Mutex;
// use tower_lsp::LspService;
// use tower_lsp::Server;
// use tracing::info;
// use tracing::Level;
// use tracing_subscriber::fmt;

// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// async fn main() {
//     let target: Box<dyn io::Write + Send + Sync> = match File::create("/tmp/turtle-lsp.txt") {
//         Ok(x) => Box::new(x),
//         Err(_) => Box::new(std::io::stdout()),
//     };
//
//     std::panic::set_hook(Box::new(|_panic_info| {
//         let backtrace = std::backtrace::Backtrace::capture();
//         info!("My backtrace: {:#?}", backtrace);
//     }));
//     fmt()
//         .with_file(true)
//         .with_line_number(true)
//         .with_max_level(Level::INFO)
//         .with_writer(Mutex::new(target))
//         .init();
//
//     let stdin = tokio::io::stdin();
//     let stdout = tokio::io::stdout();
//
//     info!("Creating prefixes");
//     let prefix = Prefixes::new().await.expect("Initalize prefixes");
//     info!("Created prefixes");
//
//     let (service, socket) = LspService::build(|client| {
//         Backend::<_, TurtleLang>::new(client, (prefix, Default::default()))
//     })
//     .finish();
//     Server::new(stdin, stdout, socket).serve(service).await;
// }
//

fn main() {
    use bevy_ecs::prelude::*;
    use lang_turtle::testing::*;
    let mut world = World::new();

    let schedule = Schedule::new(Parse);

    world.add_schedule(schedule);
    world.schedule_scope(Parse, |_, schedule| {
        schedule.add_systems((
            parse_source,
            parse_turtle_system.after(parse_source),
            // notify_parsed.after(parse_turtle_system),
        ));
    });

    world.spawn((
        TurtleComponent,
        Source("@prefix foaf: <>.".to_string()),
        Label("http://example.com/ns#".to_string()),
    ));

    world.flush();
    println!("Here");

    world.run_schedule(Parse);
    world.run_schedule(Parse);

    world.flush();
}
