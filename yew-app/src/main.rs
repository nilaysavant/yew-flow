use yew::prelude::*;

use yew_flow::components::node::RenderNodes;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <RenderNodes />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
