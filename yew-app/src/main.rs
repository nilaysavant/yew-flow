use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use yew_flow::{store::WorkspaceStore, workspace::YewFlowValues, Workspace};

#[function_component(App)]
fn app() -> Html {
    let prevent_changes = use_state(|| false);
    let values = use_state(|| {
        let WorkspaceStore { nodes, edges, .. } = WorkspaceStore::generate();
        YewFlowValues { nodes, edges }
    });
    let error = use_state(|| None);
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
        let prevent_changes = prevent_changes.clone();
        use_callback(
            move |new_values, prevent_changes| {
                if !*prevent_changes.clone() {
                    values.set(new_values)
                }
            },
            prevent_changes,
        )
    };

    let on_key_up = {
        let set_values = values.setter().clone();
        let set_error = error.setter().clone();
        let text_area_ref = text_area_ref.clone();
        use_callback(
            move |_, (set_values, text_area_ref, set_error)| {
                set_error.set(None);
                if let Some(elm) = text_area_ref.cast::<HtmlTextAreaElement>() {
                    match serde_json::from_str::<YewFlowValues>(&elm.value()) {
                        Ok(values) => set_values.set(values),
                        Err(e) => {
                            set_error.set(Some(e.to_string()));
                            log::error!("{:?}", e);
                        }
                    }
                }
            },
            (set_values, text_area_ref, set_error),
        )
    };

    let on_focus = {
        let prevent_changes = prevent_changes.setter().clone();
        use_callback(
            move |_, prevent_changes| {
                prevent_changes.set(true);
            },
            prevent_changes,
        )
    };

    let on_blur = {
        let prevent_changes = prevent_changes.setter().clone();
        use_callback(
            move |_, prevent_changes| {
                prevent_changes.set(false);
            },
            prevent_changes,
        )
    };

    html! {
        <div class="flex flex-col bg-neutral-900 text-neutral-300 p-4" style="width: 100vw; height: 100vh;">
            <div class="flex">
                <span>
                    {"YewFlow Demo (Work in Progress)"}
                </span>
            </div>
            <div class="flex-1 min-h-0 flex">
                <div class="flex-1 mr-2 h-full flex flex-col min-h-0">
                    <Workspace
                        values={(*values).clone()}
                        {on_change}
                        prevent_changes={(*prevent_changes).clone()}
                    />
                </div>
                <div class="basis-1/3 h-full flex flex-col">
                    <textarea
                        type="text"
                        ref={text_area_ref.clone()}
                        class="flex-1 resize-none w-full h-full border-2 border-neutral-400 bg-slate-800 focus:outline-none focus:border-neutral-300 text-cyan-300 selection:bg-sky-700"
                        value={(*json_text).clone()}
                        onkeyup={on_key_up}
                        onfocus={on_focus}
                        onblur={on_blur}
                    />
                    if let Some(error) = (*error).clone() {
                        <span class="text-red-500 p-1">{error}</span>
                    }
                </div>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
