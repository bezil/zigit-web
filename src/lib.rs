use yew::prelude::*;
use components::nav_header::NavHeader;
use pages::home::Home;

mod components;
mod pages;
mod models;

#[function_component]
pub fn App() -> Html {
    let version = env!("CARGO_PKG_VERSION").to_string();

    html! {
        <div class="h-screen bg-zig-grey text-white font-mono">
            <NavHeader version= { version } />
            <Home />
        </div>
    }
}
