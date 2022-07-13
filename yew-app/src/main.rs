use yew::prelude::*;

use yew_flow::{workspace::YewFlowInitialState, Workspace};

#[function_component(App)]
fn app() -> Html {
    let initial_state = YewFlowInitialState {
        nodes: vec![],
        edges: vec![],
    };

    html! {
        <>
            <Workspace
                initial_state={initial_state}
            />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
