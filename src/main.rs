use yew::prelude::*;
use gloo_net::http::Request;
use gloo_net::http::RequestMode;
use components::nav_header::NavHeader;
use js_sys::JsString;

mod components;

#[function_component]
fn App() -> Html {
    let body = use_state(|| "".to_string());
    {
        let body = body.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let body_response: String = Request::get("http://localhost:8000/")
                    .mode(RequestMode::NoCors)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                web_sys::console::log_1(&JsString::from(body_response.clone()));

                body.set(body_response.clone());
            });
            || ()
        });
    }


    html! {
        <div class="h-screen bg-zig-grey text-white">
            <NavHeader />
            <p>{ (*body).clone() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
