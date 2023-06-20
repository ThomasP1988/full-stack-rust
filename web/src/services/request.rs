use gloo_net::http::{Headers, Method, Request};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error};

pub const API_ROOT: &str = "http://127.0.0.1:8081";
pub const WS_ROOT: &str = "ws://127.0.0.1:8081";

pub async fn request<B, T>(method: Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = matches!(method, Method::POST) || matches!(method, Method::PUT);
    let url: String = format!("{}{}", API_ROOT, url);
    let mut builder = Request::new(&url);

    if allow_body {
        match serde_json::to_string(&body) {
            Ok(body_stringified) => {
                builder = builder.body(wasm_bindgen::JsValue::from_str(&body_stringified));
            }
            Err(e) => {
                println!("{}", e);
                return Err(Error::DeserializeError);
            }
        };
    };

    let headers = Headers::new();
    headers.set("Content-Type", "application/json");
    builder = builder.headers(headers);
    builder = builder.method(method);
    builder = builder.credentials(web_sys::RequestCredentials::Include);
    builder = builder.mode(web_sys::RequestMode::Cors);

    match builder.send().await {
        Ok(res) => {
            if res.ok() {
                match res.json::<T>().await {
                    Ok(data) => return Ok(data),
                    Err(e) => {
                        println!("{}", e);
                        return Err(Error::DeserializeError);
                    }
                };
            } else {
                return match res.status() {
                    401 => Err(Error::Unauthorized),
                    403 => {
                        let data: Result<String, _> = res.text().await;
                        if let Ok(data) = data {
                            Err(Error::Forbidden(Some(data)))
                        } else {
                            Err(Error::Forbidden(None))
                        }
                    }
                    404 => Err(Error::NotFound),
                    500 => Err(Error::InternalServerError),
                    _ => Err(Error::RequestError),
                };
            }
        }
        Err(e) => {
            println!("{}", e);
            return Err(Error::RequestError);
        }
    }
}

pub async fn get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::GET, url, ()).await
}

pub async fn post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(Method::POST, url, body).await
}
