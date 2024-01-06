use yew::prelude::*;

#[function_component]
pub fn NavHeader() -> Html {

    let nav_items = ["Home", "About"];

    html! {
        <div
            class="flex"
            style="background-color:#404040"
        >
            <img
                width="200px"
                src="img/logo.png" alt="Logo" class="logo"
            />
            <span>{ nav_items[0] }</span>
            <span>{ nav_items[1] }</span>
        </div>
    }
}
