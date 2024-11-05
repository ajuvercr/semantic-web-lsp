use futures::FutureExt;
use lsp_types::{Diagnostic, MessageType, Url};
use std::{collections::HashMap, fmt::Display, pin::Pin};

pub mod reqwest {
    pub use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
    pub use reqwest::Error;
    pub use reqwest::StatusCode;
    pub use reqwest::Url;
}
use reqwest::*;

#[derive(Debug)]
pub struct Resp {
    pub headers: HeaderMap,
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
    fn spawn<O: Send + 'static, F: std::future::Future<Output = O> + Send + 'static>(&self, fut: F);
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Pin<Box<dyn Send + std::future::Future<Output = Result<Resp, String>>>>;
}

impl ClientSync for tower_lsp::Client {
    fn spawn<O: Send + 'static, F: std::future::Future<Output = O> + Send + 'static>(
        &self,
        fut: F,
    ) {
        tokio::spawn(fut);
    }

    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Pin<Box<dyn Send + std::future::Future<Output = Result<Resp, String>>>> {
        use tokio::{fs::File, io::AsyncReadExt};
        use tracing::{debug, error, info};

        let m_url = reqwest::Url::parse(url);

        let client = ::reqwest::Client::new();
        let builder = client.get(url);
        let builder = headers
            .into_iter()
            .fold(builder, |builder, (k, v)| builder.header(k, v));

        return async {
            let url = m_url.map_err(|_| String::from("invalid url!"))?;
            info!("Found url {} {}", url.scheme(), url);
            if url.scheme() == "file" {
                let mut file = File::open(url.path())
                    .await
                    .map_err(|_| format!("File not found {}", url.path()))?;
                let mut body = String::new();
                file.read_to_string(&mut body)
                    .await
                    .map_err(|_| format!("Failed to read file"))?;
                let status = 200;
                let headers = HeaderMap::new();
                return Ok(Resp {
                    headers,
                    body,
                    status,
                });
            }

            debug!("sending blocking");
            let resp = match builder.send().await {
                Ok(x) => x,
                Err(e) => {
                    error!(error = ?e);
                    return Err(e.to_string());
                }
            };

            let status = resp.status().as_u16();
            let headers = resp.headers().clone();
            debug!("got resp");
            let body = resp.text().await.unwrap();

            Ok(Resp {
                headers,
                body,
                status,
            })
        }
        .boxed();
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
