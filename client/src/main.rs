// #![allow(unused)] // For beginning only.

mod api;
mod app;
mod components;
mod helpers;
mod layout;
mod pages;
mod router;
mod store;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
