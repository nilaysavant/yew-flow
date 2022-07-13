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
    let on_submit = {
        let text_area_ref = text_area_ref.clone();
        let values = values.clone();
        let error = error.clone();
        use_callback(
            move |_e, text_area_ref| {
                if let Some(elm) = text_area_ref.cast::<HtmlTextAreaElement>() {
                    match serde_json::from_str::<YewFlowValues>(&elm.value()) {
                        Ok(new_values) => values.set(new_values),
                        Err(e) => {
                            error.set(e.to_string());
                            log::error!("could not deserialize json: {:?}", e)
                        }
                    }
                }
            },
            text_area_ref,
        )
    };

    html! {
        <div class="flex flex-col min-h-0 bg-neutral-900 p-4 text-neutral-300" style="width: 100vw; height: 100vh;">
            <Workspace
                values={(*values).clone()}
                {on_change}
            />
           <div class="flex-1 flex w-full">
            <div class="flex-1 mr-4 mt-4">
                <textarea
                    ref={text_area_ref.clone()}
                    class="resize-none w-full h-full border-2 border-neutral-400 bg-slate-800 focus:outline-none focus:border-neutral-300 text-cyan-300 selection:bg-sky-700"
                    value={(*json_text).clone()}
                />
            </div>
            <div class="flex-1 mt-4">
                <button
                    onclick={on_submit}
                    class="bg-slate-700 px-6 py-4 rounded-sm border-2 border-neutral-400 hover:bg-slate-600 active:bg-slate-500"
                >
                    {"Submit"}
                </button>
                <div class="text-red-500">{(*error).clone()}</div>
            </div>
           </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
