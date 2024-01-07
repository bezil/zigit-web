use yew::prelude::*;
use gloo_net::http::Request;
use components::nav_header::NavHeader;
use js_sys::JsString;

mod components;
mod models;

use models::home_header::NavigationItem;

#[function_component]
fn App() -> Html {
    let body = use_state(|| NavigationItem {
        id: 0,
        note: "Not Found".to_string(),
    });
    {
        let body = body.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let body_response: NavigationItem = Request::get("https://zigit.shuttleapp.rs/todos/1")
                    .send()
                    .await
                    .unwrap()
                    .json().await.expect("text retrieve failed");
                web_sys::
                    console::log_1(&JsString::from(body_response.clone().note));

                body.set(body_response);
            });
            || ()
        });
    }

    html! {
        <div class="h-screen bg-zig-grey text-white">
            <NavHeader />
            <p> {   (*body).clone().note }   </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
