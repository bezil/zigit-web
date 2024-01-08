use yew::prelude::*;

#[function_component]
pub fn NavHeader() -> Html {

    let nav_items = ["Home", "About"];

    html! {
        <div
            class="flex items-center px-4 py-3 text-white w-full
            bg-zig-grey/50 border-b border-b-white/20"
        >
            <img
                width="100px"
                src="img/logo.png" alt="zigit" class="logo"
            />
            <div class="flex justify-center space-x-8 w-full uppercase">
            <span>{ nav_items[0] }</span>
            <span>{ nav_items[1] }</span>
            </div>
        </div>
    }
}
