use lsp_types::{Diagnostic, MessageType, Url};
use std::fmt::Display;

#[tower_lsp::async_trait]
pub trait Client: Clone + ClientSync {
    async fn log_message<M: Display + Sync + Send + 'static>(&self, ty: MessageType, msg: M) -> ();
    async fn publish_diagnostics(
        &self,
        uri: Url,
        diags: Vec<Diagnostic>,
        version: Option<i32>,
    ) -> ();
}

pub trait ClientSync {
    fn spawn<O: Send + 'static, F: std::future::Future<Output = O> + Send + 'static>(&self, fut: F);
}

impl ClientSync for tower_lsp::Client {
    fn spawn<O: Send + 'static, F: std::future::Future<Output = O> + Send + 'static>(
        &self,
        fut: F,
    ) {
        tokio::spawn(fut);
    }
}

#[tower_lsp::async_trait]
impl Client for tower_lsp::Client {
    async fn log_message<M: Display + Sync + Send + 'static>(&self, ty: MessageType, msg: M) -> () {
        self.log_message(ty, msg).await;
    }

    async fn publish_diagnostics(
        &self,
        uri: Url,
        diags: Vec<Diagnostic>,
        version: Option<i32>,
    ) -> () {
        self.publish_diagnostics(uri, diags, version).await;
    }
}
