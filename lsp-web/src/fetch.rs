use std::collections::HashMap;

// pub mod reqwest {
//     pub use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
//     pub use reqwest::Error;
//     pub use reqwest::StatusCode;
//     pub use reqwest::Url;
// }
use lsp_core::{
    // client::reqwest::{HeaderMap, HeaderName, HeaderValue, Url},
    client::Resp,
};
use serde::Serializer;
use serde_json::json;
use tracing::info;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::Response;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn fetch(url: JsValue, options: JsValue) -> Result<JsValue, JsValue>;
}

pub async fn try_fetch(url: String, headers: HashMap<String, String>) -> Result<Resp, String> {
    // if let Ok(url) = Url::parse(&url) {
    //     if url.scheme() == "file" {
    //         info!("Url scheme is file, let's do that! {}", url.path());
    //         let body = read_file(url.path()).await?;
    //         let status = 200;
    //         let headers = HeaderMap::new();
    //         return Ok(Resp {
    //             headers,
    //             body,
    //             status,
    //         });
    //     }
    // }

    let ser: serde_wasm_bindgen::Serializer = serde_wasm_bindgen::Serializer::json_compatible();
    let options_json = json!({ "headers": headers });
    // let url = format!("https://proxy.linkeddatafragments.org/{}", url);
    let options = ser
        .serialize_some(&options_json)
        .map_err(|_| String::from("failed to serialize headers"))?;

    let resp_value = fetch(url.clone().into(), options)
        .await
        .map_err(|e| format!("{:?}", e))?;
    info!("Got resp {}", url);

    // `resp_value` is a `Response` object.
    if !resp_value.is_instance_of::<Response>() {
        return Err("Not a response!".into());
    }

    let resp: Response = resp_value.dyn_into().unwrap();
    let status = resp.status();
    let headers = resp.headers();
    let headers: HashMap<String, String> =
        serde_wasm_bindgen::from_value(headers.into()).map_err(|e| e.to_string())?;

    let map: Vec<_> = headers.into_iter().map(|(k, v)| (k, v)).collect();

    // Convert this other `Promise` into a rust `Future`.
    let body = wasm_bindgen_futures::JsFuture::from(resp.text().map_err(|e| format!("{:?}", e))?)
        .await
        .map_err(|e| format!("{:?}", e))?
        .as_string()
        .ok_or(String::from("Not a string"))?;

    Ok(Resp {
        headers: map, // map,
        body,
        status,
    })
}

pub async fn local_fetch(
    url: String,
    headers: HashMap<String, String>,
    tx: futures::channel::oneshot::Sender<Result<Resp, String>>,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let resp = try_fetch(url, headers).await;

    tx.send(resp).unwrap();

    Ok("".into())
}
