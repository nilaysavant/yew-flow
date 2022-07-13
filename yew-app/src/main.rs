use yew::prelude::*;

use yew_flow::{store::WorkspaceStore, workspace::YewFlowValues, Workspace};

#[function_component(App)]
fn app() -> Html {
    let values = use_state(|| YewFlowValues {
        nodes: WorkspaceStore::default().nodes,
        edges: WorkspaceStore::default().edges,
    });
    let json_text = {
        let values = values.clone();
        use_memo(
            |values| {
                let values = values.clone();
                serde_json::to_string_pretty(&(*values).clone()).unwrap()
            },
            values,
        )
    };

    let on_change = {
        let values = values.clone();
        use_callback(move |new_values, _| values.set(new_values), ())
    };

    html! {
        <div class="flex flex-col min-h-0" style="width: 100vw; height: 100vh;">
            <Workspace
                values={(*values).clone()}
                {on_change}
            />
           <div class="flex-1 flex w-full px-4 bg-neutral-900">
            <div class="flex-1 p-2">
                <textarea
                    class="w-full h-full border-2 border-neutral-500 bg-slate-800 focus:outline-none focus:border-neutral-400 text-cyan-300 selection:bg-blue-500"
                    value={(*json_text).clone()}
                />
            </div>
            <div class="flex-1">{"B"}</div>
           </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
