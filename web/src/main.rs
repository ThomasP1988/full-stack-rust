mod app;
mod services;
mod pages;
mod error;
mod components;
mod hooks;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
