use crate::{
    hooks::use_message_stream::{use_message_stream, UseMessageStreamProps},
    services::messages::{add, list},
};
use web_sys::HtmlElement;

use gloo_console::log;
use shared::message::{ AddMessage, Message};

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once, UseAsyncHandle};

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {}

#[function_component(Chat)]
pub fn chat(_props: &Props) -> Html {
    let messages_container = use_node_ref();

    let message_input_handle = use_state(String::default);
    let message_input_value = (*message_input_handle).clone();

    let message_received_buffer_handle: UseStateHandle<Option<Message>> = use_state(|| None);
    let message_received_buffer_value = (*message_received_buffer_handle).clone();

    let messages_handler: yew::UseStateHandle<Vec<Message>> = use_state(|| vec![]);
    let messages_handler_value = (*messages_handler).clone();

    let scroll_down_chat = {

        let messages_container = messages_container.clone();

        move || {
            let message_container_elm = messages_container.cast::<HtmlElement>().unwrap();
            message_container_elm.set_scroll_top(message_container_elm.scroll_height())
        }
    };

    let on_new_message = {
        let message_received_buffer_handle = message_received_buffer_handle.clone();
    
        use_callback(
            move |message: Message, message_received_buffer_handle| {
                message_received_buffer_handle.set(Some(message));
            },
            message_received_buffer_handle,
        )
    };

    let connect = use_message_stream(UseMessageStreamProps {
        on_new_message: on_new_message,
    });

    {
        let mut messages_handler_value = messages_handler_value.clone();
        let messages_handler = messages_handler.clone();

        use_effect_with_deps(
            move |message_buffered| {
                let message = match message_buffered {
                    Some(message) => message,
                    None => return,
                };

                match messages_handler_value.last() {
                    Some(last_message) => {
                        if last_message == message {
                            return;
                        }
                    }
                    None => (),
                };

                messages_handler_value.push(message.to_owned());
                messages_handler.set(messages_handler_value);
            },
            message_received_buffer_value,
        );
    }

    {

        let scroll_down_chat =scroll_down_chat.clone();

        use_effect_with_deps(move |_| {
            scroll_down_chat();
        }, messages_handler_value.clone())
    }

    let list_messages: UseAsyncHandle<Vec<Message>, crate::error::Error> =
        use_async(async move { list().await });

    let add_message: UseAsyncHandle<Message, crate::error::Error> = {
        let message_input_value: String = message_input_value.clone();
        use_async(async move {
            add(AddMessage {
                message: message_input_value,
            })
            .await
        })
    };

    {
        let connect = connect.clone();
        let list_messages = list_messages.clone();
        use_effect_once(move || {
            let _ = connect.emit(());
            list_messages.run();
            || {}
        });
    };

    {
        let messages_handler: UseStateHandle<Vec<Message>> = messages_handler.clone();
        let mut messages_handler_value = messages_handler_value.clone();
        use_effect_with_deps(
            {
                move |data: &Option<Vec<Message>>| {
                    if let Some(data) = data {
                        log!("received data", data.len());
                        log!("received data ptr ", &messages_handler_value as *const _);
                        messages_handler_value.append(data.to_owned().as_mut());
                        messages_handler.set(messages_handler_value.clone());
                    }
                }
            },
            list_messages.data.clone(),
        );
    }

    let on_message_change: Callback<Event> = {
        let message_input_handle = message_input_handle.clone();

        Callback::from(move |e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                message_input_handle.set(input.value());
            }
        })
    };

    let on_submit = {
        let message_input_handle = message_input_handle.clone();
        let add_message: UseAsyncHandle<Message, crate::error::Error> = add_message.clone();

        move |e: SubmitEvent| {
            e.stop_immediate_propagation();
            e.prevent_default();
            add_message.run();
            message_input_handle.set("".to_owned());
        }
    };

    html! {
        <>
            <section id="messages" class="flex flex-col justify-end h-5/6">
                <div class="overflow-auto min-h-0" ref={messages_container}>
                    {
                    if list_messages.loading {
                        html! { "Loading" }
                    } else if let Some(error) = &list_messages.error {
                        html! { error }
                    } else if !messages_handler_value.is_empty() {
                        html!{
                            <>
                            {
                                for messages_handler_value.iter().map(|message| {
                                    let message: Message = message.clone();
                                    let date_str = message.date.format("%Y-%m-%d %H:%M:%S");
                                    html!{<div id={date_str.to_string()} class="flex gap-x-2 items-center">
                                        <p class="text-sm flex">
                                            {
                                                date_str.to_string()
                                            }
                                        </p>
                                        <p class="flex">
                                            {
                                                message.username
                                            }
                                            {
                                                ":"
                                            }
                                        </p>
                                        <p class="flex">
                                            {
                                                message.message
                                            }
                                        </p>
                                    </div>}
                                })
                            }
                            </>
                        }
                    } else {
                        html! { "Be the first to send a message" }
                    }
                }
                </div>
            </section>
            <section id="send-message" class="flex mt-3">
                <form class="w-full" onsubmit={on_submit}>
                    <div class="relative w-full">
                        <input onchange={on_message_change} value={message_input_value.clone()} type="search" id="search" class="block w-full p-4 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500" placeholder="Type message" />
                        <button type="submit" class="text-white absolute right-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2" disabled={message_input_value.clone().len() == 0}>{ "Send" }</button>
                    </div>
                </form>
            </section>
        </>
    }
}
