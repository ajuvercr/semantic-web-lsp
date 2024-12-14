use bevy_ecs::system::Resource;
use futures::FutureExt;
use lsp_core::client::{Client, ClientSync, Resp};
use lsp_types::{Diagnostic, MessageType, Url};
use std::{collections::HashMap, fmt::Display, pin::Pin};
use tracing::info;

#[derive(Resource, Clone)]
pub struct WebClient {
    client: tower_lsp::Client,
}
impl WebClient {
    pub fn new(client: tower_lsp::Client) -> Self {
        Self { client }
    }
}

impl ClientSync for WebClient {
    fn spawn<F: std::future::Future<Output = ()> + Send + 'static>(&self, fut: F) {
        let _ = wasm_bindgen_futures::future_to_promise(async {
            info!("Spawning future");
            fut.await;
            info!("Future ended");
            Ok("Good".into())
        });
    }

    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Pin<Box<dyn Send + std::future::Future<Output = Result<Resp, String>>>> {
        todo!()
    }
}

#[tower_lsp::async_trait]
impl Client for WebClient {
    async fn log_message<M: Display + Sync + Send + 'static>(&self, ty: MessageType, msg: M) -> () {
        // self.client.log_message(ty, msg).await;
    }

    async fn publish_diagnostics(
        &self,
        uri: Url,
        diags: Vec<Diagnostic>,
        version: Option<i32>,
    ) -> () {
        // self.client.publish_diagnostics(uri, diags, version).await;
    }
}
