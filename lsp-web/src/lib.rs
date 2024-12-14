mod client;
mod fetch;

use bevy_ecs::{system::Resource, world::World};
use client::WebClient;
use futures::{channel::mpsc::unbounded, stream::TryStreamExt, StreamExt as _};
use lsp_core::{
    backend::Backend,
    client::{Client, ClientSync},
    components::{CommandSender, SemanticTokensDict},
    lang::OtherPublisher,
    setup_schedule_labels,
};
use lsp_types::SemanticTokenType;
use tower_lsp::{LspService, Server};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::stream::JsStream;

fn setup_global_subscriber() {
    use tracing_subscriber::fmt::format::Pretty;
    use tracing_subscriber::prelude::*;
    use tracing_web::{performance_layer, MakeWebConsoleWriter};

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .without_time() // std::time is not available in browsers, see note below
        .with_writer(MakeWebConsoleWriter::new()); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init(); // Install these as subscribers to tracing events
}

fn setup_world<C: Client + ClientSync + Resource + Clone>(
    client: C,
) -> (CommandSender, Vec<SemanticTokenType>) {
    let mut world = World::new();

    setup_schedule_labels::<C>(&mut world);

    let (publisher, mut rx) = OtherPublisher::new();
    world.insert_resource(publisher);

    let c = client.clone();
    client.spawn(async move {
        while let Some(x) = rx.next().await {
            c.publish_diagnostics(x.uri, x.diagnostics, x.version).await;
        }
    });

    lang_turtle::setup_world(&mut world);
    lang_jsonld::setup_world(&mut world);
    lang_sparql::setup_world(&mut world);

    let (tx, mut rx) = unbounded();
    let sender = CommandSender(tx);
    world.insert_resource(sender.clone());
    world.insert_resource(client.clone());

    let r = world.resource::<SemanticTokensDict>();
    let mut semantic_tokens: Vec<_> = (0..r.0.len()).map(|_| SemanticTokenType::KEYWORD).collect();
    r.0.iter()
        .for_each(|(k, v)| semantic_tokens[*v] = k.clone());

    client.spawn(async move {
        while let Some(mut x) = rx.next().await {
            world.commands().append(&mut x);
            world.flush_commands();
        }
    });

    (sender, semantic_tokens)
}

#[wasm_bindgen]
pub struct ServerConfig {
    into_server: js_sys::AsyncIterator,
    from_server: web_sys::WritableStream,
}

#[wasm_bindgen]
impl ServerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(into_server: js_sys::AsyncIterator, from_server: web_sys::WritableStream) -> Self {
        Self {
            into_server,
            from_server,
        }
    }
}

// NOTE: we don't use web_sys::ReadableStream for input here because on the
// browser side we need to use a ReadableByteStreamController to construct it
// and so far only Chromium-based browsers support that functionality.

// NOTE: input needs to be an AsyncIterator<Uint8Array, never, void> specifically
#[wasm_bindgen]
pub async fn serve(config: ServerConfig) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"server::serve".into());

    setup_global_subscriber();

    let ServerConfig {
        into_server,
        from_server,
    } = config;

    let input = JsStream::from(into_server);
    let input = input
        .map_ok(|value| {
            value
                .dyn_into::<js_sys::Uint8Array>()
                .expect("could not cast stream item to Uint8Array")
                .to_vec()
        })
        .map_err(|_err| std::io::Error::from(std::io::ErrorKind::Other))
        .into_async_read();

    let output = JsCast::unchecked_into::<wasm_streams::writable::sys::WritableStream>(from_server);
    let output = wasm_streams::WritableStream::from_raw(output);
    let output = output.try_into_async_write().map_err(|err| err.0)?;

    let (service, socket) = LspService::build(|client| {
        let (sender, rt) = setup_world(WebClient::new(client.clone()));
        Backend::new(sender, client, rt)
    })
    .finish();

    // Server::new(stdin, stdout, socket).serve(service).await;
    //
    // let (service, messages) =
    //     LspService::new(|client| demo_lsp_server::Server::new(client, language));
    Server::new(input, output, socket).serve(service).await;

    Ok(())
}
