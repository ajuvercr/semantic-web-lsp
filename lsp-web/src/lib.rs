mod fetch;
mod web_types;
use futures::FutureExt as _;
use lang_jsonld::JsonLd;
use lang_turtle::TurtleLang;
use lsp_bin::backend::Backend;
use lsp_core::client::{Client, ClientSync};
use lsp_core::prefix::Prefixes;

use lsp_types::Diagnostic;
use serde::Serializer;
use serde_json::json;
use tower_lsp::LanguageServer;
use tracing::info;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_types as wt;

const SER: serde_wasm_bindgen::Serializer = serde_wasm_bindgen::Serializer::json_compatible();
static mut LOG_FN: Option<js_sys::Function> = None;
static mut DIAGS_FN: Option<js_sys::Function> = None;
static mut READ_FN: Option<js_sys::Function> = None;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Add this line:
    let config = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_console_config(tracing_wasm::ConsoleConfig::ReportWithoutConsoleColor)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);

    Ok(())
}

fn publish_diagnostics(diags: JsValue) -> Result<(), String> {
    unsafe {
        let this = JsValue::null();
        if let Some(f) = &DIAGS_FN {
            if let Err(e) = f.call1(&this, &diags) {
                let msg: serde_json::Value = serde_wasm_bindgen::from_value(e).unwrap();
                return Err(format!("Call failed {:?}", msg));
            }
        } else {
            return Err("Not set!".to_string());
        }
    }
    Ok(())
}

fn log_message(msg: JsValue) -> Result<(), String> {
    unsafe {
        let this = JsValue::null();
        if let Some(f) = &LOG_FN {
            if let Err(e) = f.call1(&this, &msg) {
                let msg: serde_json::Value = serde_wasm_bindgen::from_value(e).unwrap();
                return Err(msg.to_string());
            }
        } else {
            return Err("Not set!".to_string());
        }
    }
    Ok(())
}

pub fn log_msg(msg: impl std::fmt::Display) {
    if let Err(e) = log_message(msg.to_string().into()) {
        let _ = log_message(format!("Failed logging msg {}", e).into());
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WebClient;

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }
}

#[wasm_bindgen]
pub fn set_logger(f: wt::SetLoggerFn) {
    unsafe {
        let jsvalue: JsValue = f.into();
        LOG_FN = Some(jsvalue.unchecked_into());
    }
}

#[wasm_bindgen]
pub fn set_diags(f: wt::SetDiagnosticsFn) {
    unsafe {
        let jsvalue: JsValue = f.into();
        DIAGS_FN = Some(jsvalue.unchecked_into());
    }
}

#[wasm_bindgen]
pub fn set_read_file(f: wt::SetReadFileFn) {
    unsafe {
        let jsvalue: JsValue = f.into();
        READ_FN = Some(jsvalue.unchecked_into());
    }
}

pub async fn read_file(location: &str) -> Result<String, String> {
    unsafe {
        let this = JsValue::null();
        if let Some(f) = &READ_FN {
            let fut = f
                .call1(&this, &location.into())
                .map_err(|e| format!("{:?}", e))?;

            let body = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(fut))
                .await
                .map_err(|e| format!("{:?}", e))?
                .as_string()
                .ok_or(String::from("Not a string"))?;

            return Ok(body);
        } else {
            return Err("Not set!".to_string());
        }
    }
}

#[wasm_bindgen]
pub fn create_webclient() -> WebClient {
    WebClient::new()
}

impl ClientSync for WebClient {
    fn spawn<O: Send + 'static, F: std::future::Future<Output = O> + Send + 'static>(
        &self,
        fut: F,
    ) {
        let _ = wasm_bindgen_futures::future_to_promise(async {
            fut.await;
            Ok("Good".into())
        });
    }

    fn fetch(
        &self,
        url: &str,
        headers: &std::collections::HashMap<String, String>,
    ) -> std::pin::Pin<
        Box<dyn Send + std::future::Future<Output = Result<lsp_core::client::Resp, String>>>,
    > {
        use futures::channel::oneshot;
        let (tx, rx) = oneshot::channel();
        let _ = wasm_bindgen_futures::future_to_promise(fetch::local_fetch(
            url.to_string(),
            headers.clone(),
            tx,
        ));

        async {
            match rx.await {
                Ok(Ok(x)) => Ok(x),
                Ok(Err(x)) => Err(x.to_string()),
                Err(_) => Err("Channel was canceled".to_string()),
            }
        }
        .boxed()
    }
}

#[tower_lsp::async_trait]
impl Client for WebClient {
    async fn log_message<M: std::fmt::Display + Sync + Send + 'static>(
        &self,
        _ty: lsp_types::MessageType,
        msg: M,
    ) -> () {
        if let Err(e) = log_message(msg.to_string().into()) {
            let _ = log_message(format!("Failed logging msg {}", e).into());
        }
    }

    async fn publish_diagnostics(
        &self,
        uri: lsp_types::Url,
        diags: Vec<Diagnostic>,
        _version: Option<i32>,
    ) -> () {
        let json = json!({
            "uri": uri.as_str(),
            "diagnostics": diags
        });
        let diags = SER.serialize_some(&json).unwrap();
        if let Err(e) = publish_diagnostics(diags) {
            let _ = log_message(format!("Failed publishing diags {}", e).into());
        }
    }
}

macro_rules! gen {
    ($class:path ; $($fn:tt $ty:path)* ) => {
        #[wasm_bindgen]
        impl $class {
            $(
            pub async fn $fn(&self, params: $ty) -> Result<JsValue, JsValue> {
                log_message(format!("Running {}", stringify!($fn)).into())?;
                let params = serde_wasm_bindgen::from_value(params.into())?;
                let out = self.inner.$fn(params).await
                    .map_err(|e| format!("{} failed {}", stringify!($fn), e.to_string()))?;
                let out = SER.serialize_some(&out)?;
                Ok(out)
            }
            )*
        }
    };

    ($class:ty , $other:ty $( , $others:ty )*; $($fn:ident $ty:path)*) => {
            gen!($other $(, $others)* ; $($fn $ty)*);
            gen!($class ; $($fn $ty)*);
    };
}

macro_rules! gen2 {
    ($class:path; $($fn:ident $ty:path)*) => {
        #[wasm_bindgen]
        impl $class {
            $(
            pub async fn $fn(&self, params: $ty) -> Result<JsValue, JsValue> {
                log_message(format!("Running {}", stringify!($fn)).into())?;
                let params = match serde_wasm_bindgen::from_value(params.into()) {
                    Ok(x) => x,
                    Err(e) => {
                        log_message(format!("Error {}", e).into())?;
                        return Err(e.to_string().into());
                    }
                };
                self.inner.$fn(params).await;
                Ok("Ok".to_string().into())
            }
            )*
        }
    };
    ($class:ty , $other:ty $( , $others:ty )*; $($fn:ident $ty:path)*) => {

            gen2!($other $(, $others)* ; $($fn $ty)*);
            gen2!($class ; $($fn $ty)*);
    };
}

#[wasm_bindgen]
pub struct TurtleWebBackend {
    inner: Backend<WebClient, TurtleLang>,
}

#[wasm_bindgen]
pub async fn turtle_backend(client: WebClient) -> Option<TurtleWebBackend> {
    let prefixes = Prefixes::new().await?;

    Some(TurtleWebBackend {
        inner: Backend::new(client, (prefixes, Default::default())),
    })
}

#[wasm_bindgen]
pub struct JsonLDWebBackend {
    inner: Backend<WebClient, JsonLd>,
}

#[wasm_bindgen]
impl JsonLDWebBackend {
    #[wasm_bindgen(constructor)]
    pub fn new(client: WebClient) -> Self {
        info!("jsonld Webclient started");
        Self {
            inner: Backend::new(client, Default::default()),
        }
    }
}

gen!(TurtleWebBackend, JsonLDWebBackend ; initialize wt::InitializeParams prepare_rename wt::PrepareRenameParams  rename wt::RenameParams semantic_tokens_full wt::SemanticTokensParams completion wt::CompletionParams formatting wt::DocumentFormattingParams code_action wt::CodeActionParams );

gen2!(TurtleWebBackend, JsonLDWebBackend; did_open  wt::DidOpenTextDocumentParams did_change wt::DidChangeTextDocumentParams did_save wt::DidSaveTextDocumentParams);
