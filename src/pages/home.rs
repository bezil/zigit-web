use yew::prelude::*;
use gloo_net::http::Request;
use gloo::console::log;

use crate::models::home_header::NavigationItem;

#[function_component(Home)]
pub fn home_page() -> Html {
    let body = use_state(|| NavigationItem {
        id: 0,
        note: "Coming Soon".to_string(),
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
        <div class="grid h-60">
            <p class="place-self-center">
                { (*body).clone().note }
            </p>
        </div>
    }
}
