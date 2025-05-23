use std::{
    collections::HashMap,
    fmt::Display,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use bevy_ecs::system::Resource;
use futures::FutureExt;
use lsp_core::client::{Client, ClientSync, Resp};
use lsp_types::{Diagnostic, MessageType, Url};
use tracing::info;

use crate::fetch::local_fetch;

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
