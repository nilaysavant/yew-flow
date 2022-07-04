use yew::prelude::*;

use yew_flow::components::node::render_node_list::RenderNodeList;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <RenderNodeList />
            <svg width="190" height="160" xmlns="http://www.w3.org/2000/svg">
                <path 
                    d="M 100 0 C 200 0, 0 100, 100 100" 
                    stroke="blue"
                    stroke-width="3px"
                    fill="transparent"
                />
            </svg>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
