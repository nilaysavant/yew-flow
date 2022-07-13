use yew::prelude::*;

use yew_flow::{store::WorkspaceStore, workspace::YewFlowValues, Workspace};

#[function_component(App)]
fn app() -> Html {
    let values = use_state(|| YewFlowValues {
        nodes: WorkspaceStore::default().nodes,
        edges: WorkspaceStore::default().edges,
    });

    let on_change = {
        use_callback(
            |new_values, _| {
                log::info!("new_values: {:?}", new_values);
            },
            (),
        )
    };

    html! {
        <>
            <Workspace
                values={YewFlowValues {
                    edges: values.edges.clone(),
                    nodes: values.nodes.clone(),
                }}
                {on_change}
            />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
