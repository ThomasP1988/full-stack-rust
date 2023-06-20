use crate::components::username_dialog::UsernameDialog;
use crate::{pages::chat::Chat, services::users::me};
use shared::user::User;
use yew::prelude::*;
use yew_hooks::{
    prelude::{use_async_with_options, UseAsyncOptions},
    UseAsyncHandle,
};

#[function_component(App)]
pub fn app() -> Html {

    let username_handle = use_state(String::default);
    let username_value = (*username_handle).clone();

    let me: UseAsyncHandle<User, crate::error::Error> =
        use_async_with_options(async move { me().await }, UseAsyncOptions::enable_auto());

    let on_username_set = {
        let username_handle = username_handle.clone();

        use_callback(
            move |username: String, username_handle | {
                username_handle.set(username);
            },
            username_handle,
        )
    };

    {
        let username_handle = username_handle.clone();
        let data = me.data.clone();
        use_effect_with_deps(
            {
                move |data: &Option<User>| {
                    if let Some(user) = data {
                        username_handle.set(user.username.to_owned());
                    }
                }
            },
            data,
        );
    }

    html! {
        <>
            <main class="flex flex-col h-screen">
                <header class="flex justify-center p-4 border-gray-800 bg-blue-200">
                    <div class="container max-w-4xl flex flex-row">
                        <div class="flex grow">
                            {"Rust Chat"}
                        </div>
                        <div class="flex grow-0">
                        {
                           username_value
                        }
                        </div>
                    </div>
                </header>
                <div class="flex grow-3 justify-center max-w-full pt-2 h-full">
                    <div class="container max-w-4xl h-full">
                        <Chat />
                    </div>
                </div>
            </main>
            {
                if let Some(_) = &me.error {
                    html!(<UsernameDialog on_username_set={on_username_set} />)
                } else {
                    html!(<></>)
                }
            }
        </>
    }
}
