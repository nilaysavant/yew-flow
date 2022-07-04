use yew::prelude::*;

use yew_flow::components::node::render_node_list::RenderNodeList;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <RenderNodeList />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
