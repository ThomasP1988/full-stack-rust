use shared::user::{CreateUser, User};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlDialogElement, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once, UseAsyncHandle};

use crate::{error::Error, services::users::create};

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    pub on_username_set: Callback<String, ()>,
}

#[function_component(UsernameDialog)]
pub fn username_dialog(props: &Props) -> Html {
    let user_dialog = use_node_ref();
    let username_input_handle = use_state(String::default);
    let username_input_value = (*username_input_handle).clone();

    let create_user: UseAsyncHandle<User, crate::error::Error> = {
        let username_input_value: String = username_input_value.clone();
        use_async(async move {
            create(CreateUser {
                username: username_input_value,
            })
            .await
        })
    };

    {
        let user_dialog = user_dialog.clone();

        use_effect_once(move || {
            match user_dialog
                .cast::<HtmlDialogElement>()
                .unwrap()
                .show_modal()
            {
                Ok(_) => (),
                Err(_) => (),
            };

            || ()
        });
    }

    let on_username_change: Callback<Event> = {
        let username_input_handle: UseStateHandle<String> = username_input_handle.clone();
        Callback::from(move |e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                username_input_handle.set(input.value());
            }
        })
    };

    let on_submit = {
        let create_user = create_user.clone();

        move |e: SubmitEvent| {
            e.stop_immediate_propagation();
            e.prevent_default();
            create_user.run();
        }
    };

    {
        let username_input_handle: UseStateHandle<String> = username_input_handle.clone();
        let user_dialog = user_dialog.clone();
        let on_username_set = props.on_username_set.clone();

        use_effect_with_deps(
            move |data| {
                if let Some(user) = &data {
                    username_input_handle.set("".to_owned());
                    user_dialog.cast::<HtmlDialogElement>().unwrap().close();
                    on_username_set.emit(user.username.clone());
                }
            },
            create_user.data.clone(),
        );
    }

    html! {
        <>
            <dialog ref={user_dialog} class="w-96">
                <form onsubmit={on_submit} class="flex gap-1.5 items-end">
                    <div class="flex grow-3 flex-col w-full">
                        <label for="default-input" class="block mb-2 text-sm font-medium text-gray-900 ">{"Username"}</label>
                        <input type="text" onchange={on_username_change} value={username_input_value} id="default-input" placeholder="Type username" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" />
                    </div>
                    <button type="submit" class="text-white w-32 h-10 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2">{"Let's chat"}</button>
                </form>
                {
                    if let Some(error) = &create_user.error {
                        match error {
                            Error::Forbidden(Some(msg)) => html!(<>{msg}</>),
                            _ =>  html!(<>{error}</>)
                        }
                    } else {
                        html!(<></>)
                    }
                }
            </dialog>
        </>
    }
}
