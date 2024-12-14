use lsp_types::{Diagnostic, MessageType, Url};
use std::{collections::HashMap, fmt::Display, pin::Pin};

#[derive(Debug)]
pub struct Resp {
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub status: u16,
}

#[tower_lsp::async_trait]
pub trait Client: Clone + ClientSync {
    async fn log_message<M: Display + Sync + Send + 'static>(&self, ty: MessageType, msg: M) -> ();
    async fn publish_diagnostics(
        &self,
        uri: Url,
        diags: Vec<Diagnostic>,
        version: Option<i32>,
    ) -> ();

    // async fn fetch(
    //     &self,
    //     url: &str,
    //     headers: HeaderMap<String, String>,
    // ) -> std::result::Result<Resp, String>;
}

pub trait ClientSync {
    fn spawn<F: std::future::Future<Output = ()> + 'static>(&self, fut: F);
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Pin<Box<dyn Send + std::future::Future<Output = Result<Resp, String>>>>;
}
