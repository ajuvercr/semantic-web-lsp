use std::{
    collections::HashMap,
    fmt::Display,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use bevy_ecs::system::Resource;
use futures::FutureExt;
use lsp_core::{
    client::{Client, ClientSync, Resp},
    prelude::{Fs, FsTrait},
};
use lsp_types::{request::Request, Diagnostic, MessageType, TextEdit, Url, WorkspaceEdit};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::fetch::local_fetch;

#[derive(Serialize, Deserialize)]
struct ReadFileParams {
    url: lsp_types::Url,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ReadFileResult {
    Success { content: String },
    Failed { error: String },
}
struct ReadFile;

impl Request for ReadFile {
    type Params = ReadFileParams;

    type Result = ReadFileResult;

    const METHOD: &'static str = "custom/readFile";
}

#[derive(Debug)]
pub struct WebFs(tower_lsp::Client);
impl WebFs {
    pub fn new(client: &tower_lsp::Client) -> Fs {
        Fs(Arc::new(Self(client.clone())))
    }
}

#[tower_lsp::async_trait]
impl FsTrait for WebFs {
    fn virtual_url(&self, url: &str) -> Option<lsp_types::Url> {
        let st = if let Ok(url) = lsp_types::Url::parse(url) {
            format!("virtual://swls/{}", url.path())
        } else {
            format!("virtual://swls/{}", url)
        };
        lsp_types::Url::parse(&st).ok()
    }

    async fn read_file(&self, url: &lsp_types::Url) -> Option<String> {
        match self
            .0
            .send_request::<ReadFile>(ReadFileParams { url: url.clone() })
            .await
        {
            Ok(ReadFileResult::Success { content }) => Some(content),
            Ok(ReadFileResult::Failed { error }) => {
                tracing::error!("Failed (1) {:?}", error);
                None
            }
            Err(e) => {
                tracing::error!("Failed (2) {:?}", e);
                None
            }
        }
    }

    async fn write_file(&self, url: &lsp_types::Url, content: &str) -> Option<()> {
        let mut map = HashMap::new();
        map.insert(
            url.clone(),
            vec![TextEdit {
                new_text: content.to_string(),
                ..Default::default()
            }],
        );
        let edit = WorkspaceEdit {
            changes: Some(map),
            ..Default::default()
        };
        info!("Should apply edit, but am not doing that now");
        // self.0.apply_edit(edit).await;
        Some(())
    }
}

#[derive(Resource, Clone)]
pub struct WebClient {
    client: tower_lsp::Client,
}
impl WebClient {
    pub fn new(client: tower_lsp::Client) -> Self {
        Self { client }
    }
}

struct Sendable<T>(pub T);

// Safety: WebAssembly will only ever run in a single-threaded context.
unsafe impl<T> Send for Sendable<T> {}
impl<O, T> Future for Sendable<T>
where
    T: Future<Output = O>,
{
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safely access the inner future
        let inner = unsafe { self.map_unchecked_mut(|s| &mut s.0) };
        inner.poll(cx)
    }
}

impl ClientSync for WebClient {
    fn spawn<F: std::future::Future<Output = ()> + 'static>(&self, fut: F) {
        let _ = wasm_bindgen_futures::future_to_promise(async {
            fut.await;
            Ok("Good".into())
        });
    }

    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Pin<Box<dyn Send + std::future::Future<Output = Result<Resp, String>>>> {
        tracing::info!("Fetching {}", url);
        let (tx, rx) = futures::channel::oneshot::channel();
        let url = url.to_string();
        let headers = headers.clone();
        let fut = async move {
            let _ = local_fetch(url, headers, tx).await;
        };
        self.spawn(Sendable(fut));
        async move {
            match rx.await {
                Ok(x) => x,
                Err(_) => Err(String::from("Canceled")),
            }
        }
        .boxed()
    }
}

#[tower_lsp::async_trait]
impl Client for WebClient {
    async fn log_message<M: Display + Sync + Send + 'static>(&self, ty: MessageType, msg: M) -> () {
        self.client.log_message(ty, msg).await;
    }

    async fn publish_diagnostics(
        &self,
        uri: Url,
        diags: Vec<Diagnostic>,
        version: Option<i32>,
    ) -> () {
        info!(
            "Publishing {} diagnostics {} (version {:?})",
            diags.len(),
            uri,
            version
        );
        self.client.publish_diagnostics(uri, diags, version).await;
    }
}
