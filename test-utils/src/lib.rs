use std::{
    collections::HashMap,
    fmt::Display,
    str::FromStr as _,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

use bevy_ecs::prelude::*;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    lock::Mutex,
    FutureExt as _,
};
use lsp_core::{
    client::{Client, ClientSync, Resp},
    components::*,
    lang::{DiagnosticItem, OtherPublisher},
    setup_schedule_labels,
    systems::handle_tasks,
    Parse,
};
use lsp_types::{Diagnostic, MessageType, TextDocumentItem, Url};
use ropey::Rope;

#[derive(Resource, Debug, Clone)]
pub struct TestClient {
    logs: Arc<Mutex<Vec<(MessageType, String)>>>,
    diagnostics: Arc<Mutex<Vec<(Url, Vec<lsp_types::Diagnostic>)>>>,
    locations: HashMap<String, String>,
    tasks_running: Arc<std::sync::atomic::AtomicU32>,
    executor: Arc<async_executor::Executor<'static>>,
}

impl TestClient {
    pub fn new() -> Self {
        // let pool = LocalPool::new();
        Self {
            logs: Default::default(),
            diagnostics: Default::default(),
            locations: Default::default(),
            tasks_running: Arc::new(AtomicU32::new(0)),
            executor: Arc::new(async_executor::Executor::new()),
        }
    }
}
impl Default for TestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TestClient {
    pub fn add_res(&mut self, loc: &str, cont: &str) {
        self.locations.insert(loc.to_string(), cont.to_string());
    }
}

impl TestClient {
    pub async fn await_futures<F: FnMut()>(&self, mut tick: F) {
        tick();
        while self.tasks_running.load(Ordering::Relaxed) != 0 {
            self.executor.tick().await;
            tick();
        }
    }
}

#[tower_lsp::async_trait]
impl Client for TestClient {
    async fn log_message<M: Display + Sync + Send + 'static>(&self, ty: MessageType, msg: M) -> () {
        let mut lock = self.logs.lock().await;
        lock.push((ty, msg.to_string()));
    }

    async fn publish_diagnostics(
        &self,
        uri: Url,
        diags: Vec<Diagnostic>,
        _version: Option<i32>,
    ) -> () {
        let mut lock = self.diagnostics.lock().await;
        lock.push((uri, diags));
    }
}

impl ClientSync for TestClient {
    fn spawn<F: std::future::Future<Output = ()> + Send + 'static>(&self, fut: F) {
        self.tasks_running.fetch_add(1, Ordering::AcqRel);
        let tr = self.tasks_running.clone();
        self.executor
            .spawn(async move {
                fut.await;
                tr.fetch_sub(1, Ordering::AcqRel);
            })
            .detach();
    }

    fn fetch(
        &self,
        url: &str,
        _headers: &std::collections::HashMap<String, String>,
    ) -> std::pin::Pin<
        Box<dyn Send + std::future::Future<Output = Result<lsp_core::client::Resp, String>>>,
    > {
        let body = self.locations.get(url).cloned();
        async move {
            let mut headers = Vec::new();
            async_std::task::sleep(Duration::from_millis(200)).await;
            headers.push(("Content-Type".to_string(), "text/turtle".to_string()));
            let status = body.is_some().then_some(200).unwrap_or(404);
            Ok(Resp {
                headers,
                body: body.unwrap_or_default(),
                status,
            })
        }
        .boxed()
    }
}

pub fn setup_world(
    client: TestClient,
    f: impl FnOnce(&mut World) -> (),
) -> (World, UnboundedReceiver<DiagnosticItem>) {
    let mut world = World::new();
    setup_schedule_labels(&mut world);

    let (tx, rx) = unbounded();
    world.insert_resource(CommandSender(tx));
    world.insert_resource(CommandReceiver(rx));
    world.insert_resource(client);

    world.schedule_scope(lsp_core::Tasks, |_, schedule| {
        schedule.add_systems(handle_tasks);
    });

    f(&mut world);

    let (publisher, rx) = OtherPublisher::new();
    world.insert_resource(publisher);

    (world, rx)
}

pub fn create_file(world: &mut World, content: &str, url: &str, extra: impl Bundle) -> Entity {
    let rope = Rope::from_str(content);
    let item = TextDocumentItem {
        version: 1,
        uri: Url::from_str(url).unwrap(),
        language_id: String::from("turtle"),
        text: String::new(),
    };
    let entity = world
        .spawn((
            Source(content.to_string()),
            RopeC(rope),
            Label(item.uri.clone()),
            Wrapped(item),
            Open,
        ))
        .insert(extra)
        .id();

    world.run_schedule(Parse);

    entity
}
