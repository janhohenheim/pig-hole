use bevy::log;
use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, Response};

// Source: https://github.com/vleue/jornet/blob/2a414a8f85f975ae8d54b9e3ceab348db7c6250d/bevy-jornet/src/http.rs#L12-L25

pub async fn put<TBody: Serialize, TResponse: DeserializeOwned>(
    url: &str,
    body: TBody,
) -> TResponse {
    #[cfg(not(target_arch = "wasm32"))]
    let result = ureq::put(url)
        .send_json(&body)
        .unwrap()
        .into_json()
        .unwrap();
    #[cfg(target_arch = "wasm32")]
    let result = request(url, body, "PUT").await;

    result
}

#[cfg(target_arch = "wasm32")]
async fn request<TBody: Serialize, TResponse: DeserializeOwned>(
    url: &str,
    body: TBody,
    method: &str,
) -> TResponse {
    let mut headers = HashMap::new();
    headers.insert("Content-Type", "application/json");
    let mut opts = RequestInit::new();
    let json = serde_json::to_string(&body).unwrap();
    log::info!("{} {} {}", method, url, json);
    opts.method(method)
        .body(Some(&JsValue::from_str(&json)))
        .headers(&JsValue::from_serde(&headers).unwrap());

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    await_promise(window.fetch_with_request(&request)).await
}

#[cfg(target_arch = "wasm32")]
async fn await_promise<T: DeserializeOwned>(promise: Promise) -> T {
    let resp_value = JsFuture::from(promise).await.unwrap();
    let resp: Response = resp_value.dyn_into().unwrap();
    let val = JsFuture::from(resp.json().unwrap()).await.unwrap();
    val.into_serde().unwrap()
}

pub async fn get<T: DeserializeOwned>(url: &str) -> T {
    #[cfg(not(target_arch = "wasm32"))]
    let result = ureq::get(url).call().unwrap().into_json().unwrap();
    #[cfg(target_arch = "wasm32")]
    let result = {
        let window = web_sys::window().unwrap();
        await_promise(window.fetch_with_str(url)).await
    };

    result
}
