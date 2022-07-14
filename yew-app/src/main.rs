use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use yew_flow::{store::WorkspaceStore, workspace::YewFlowValues, Workspace};

#[function_component(App)]
fn app() -> Html {
    let values = use_state(|| YewFlowValues {
        nodes: WorkspaceStore::default().nodes,
        edges: WorkspaceStore::default().edges,
    });
    let error = use_state(|| "".to_string());
    let text_area_ref = use_node_ref();
    let json_text = {
        let values = values.clone();
        use_memo(
            |values| {
                let values = values.clone();
                // pretty formatter with ident: ref: https://stackoverflow.com/questions/42722169/generate-pretty-indented-json-with-serde
                let mut ser = serde_json::Serializer::with_formatter(
                    Vec::new(),
                    PrettyFormatter::with_indent(b"    "),
                );
                let json_value = serde_json::to_value(&(*values).clone()).unwrap();
                json_value.serialize(&mut ser).unwrap();
                String::from_utf8(ser.into_inner()).unwrap()
            },
            values,
        )
    };

    let on_change = {
        let values = values.clone();
        use_callback(move |new_values, _| values.set(new_values), ())
    };

    html! {
        <div class="flex bg-neutral-900 p-4 text-neutral-300" style="width: 100vw; height: 100vh;">
            <div class="flex-1 mr-2 h-full flex flex-col min-h-0">
                <Workspace
                    values={(*values).clone()}
                    {on_change}
                />
            </div>
            <div class="flex-1 h-full">
                <textarea
                    ref={text_area_ref.clone()}
                    class="resize-none w-full h-full border-2 border-neutral-400 bg-slate-800 focus:outline-none focus:border-neutral-300 text-cyan-300 selection:bg-sky-700"
                    value={(*json_text).clone()}
                />
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
