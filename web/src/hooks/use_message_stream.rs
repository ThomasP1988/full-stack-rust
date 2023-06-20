use crate::services::request::WS_ROOT;
use anyhow::Error;
use gloo_console::log;
use shared::message::Message;
use wasm_bindgen::JsValue;
use yew::{hook, use_callback, use_state, Callback};
use yew_websocket::macros::Json;
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

type ConnectFn = Callback<()>;

#[derive(Clone)]
pub struct UseMessageStreamProps {
    pub on_new_message: Callback<Message, ()>,
}

#[hook]
pub fn use_message_stream(props: UseMessageStreamProps) -> ConnectFn {
    let ws_handle: yew::UseStateHandle<Option<WebSocketTask>> = use_state(|| None);

    let on_message_received: Callback<_, ()> = {
        let on_new_message = props.on_new_message.clone();
        use_callback(
            move |Json(data): Json<Result<Message, Error>>, on_new_message| match data {
                Ok(res) => {
                    log!("new message : {}", JsValue::from(res.message.clone()));
                    on_new_message.emit(res.clone())
                }
                Err(e) => log!("error socket: {}", JsValue::from(e.to_string())),
            },
            on_new_message.clone(),
        )
    };

    let on_ws_status_change = use_callback(
        move |status: WebSocketStatus, _| match status {
            WebSocketStatus::Opened => log!("socket opened"),
            WebSocketStatus::Closed => log!("socket closed"),
            WebSocketStatus::Error => log!("socket error"),
        },
        (),
    );

    let connect: ConnectFn = {
        let on_new_message = props.on_new_message.clone();

        use_callback(
            move |_: (), (on_message_received, on_ws_status_change, _)| {
                log!("connect CB");
                match WebSocketService::connect(
                    format!("{}/{}", WS_ROOT, "ws").as_str(),
                    on_message_received.clone(),
                    on_ws_status_change.clone(),
                ) {
                    Ok(data) => {
                        log!("websocket connected");
                        ws_handle.set(Some(data));
                    }
                    Err(e) => log!(
                        "error connecting to websocket",
                        JsValue::from(e.to_string())
                    ),
                };
            },
            (
                on_message_received.clone(),
                on_ws_status_change.clone(),
                on_new_message.clone(),
            ),
        )
    };


    connect
}
