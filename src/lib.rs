use yew::prelude::*;
use gloo_net::http::Request;
use components::nav_header::NavHeader;
use gloo::console::log;

mod components;
mod models;

use models::home_header::NavigationItem;

#[function_component]
pub fn App() -> Html {
    let body = use_state(|| NavigationItem {
        id: 0,
        note: "Not Found".to_string(),
    });
    {
        let body = body.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let body_response: NavigationItem = Request::get("https://zigit.shuttleapp.rs/todos/1")
                    .send().await.expect("request failed")
                    .json().await.expect("text retrieve failed");
                log!(body_response.clone().note);
                body.set(body_response);
            });
            || ()
        });
    }

    html! {
        <div class="h-screen bg-zig-grey text-white font-mono">
            <NavHeader />
            <p>
            {   (*body).clone().note }
            </p>
        </div>
    }
}
