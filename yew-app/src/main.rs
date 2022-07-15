use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use yew_flow::{store::WorkspaceStore, workspace::YewFlowValues, Workspace};

use crate::utils::flow_utils::{parse_flow_json_text_to_values, values_to_flow_json_text};

mod utils;

#[function_component(App)]
fn app() -> Html {
    let prevent_changes = use_state(|| false);
    let values = use_state(|| {
        let WorkspaceStore { nodes, edges, .. } = WorkspaceStore::generate();
        YewFlowValues { nodes, edges }
    });
    let error = use_state(|| None);
    let text_area_ref = use_node_ref();
    let json_text = use_state(|| {
        let WorkspaceStore { nodes, edges, .. } = WorkspaceStore::generate();
        let values = YewFlowValues { nodes, edges };
        values_to_flow_json_text(&values).unwrap()
    });

    let on_change = {
        let values_setter = values.setter().clone();
        let json_text_setter = json_text.setter().clone();
        let prevent_changes = prevent_changes.clone();
        use_callback(
            move |new_values: YewFlowValues, (prevent_changes, values_setter)| {
                if !*prevent_changes.clone() {
                    values_setter.set(new_values.clone());
                    match values_to_flow_json_text(&new_values) {
                        Ok(json_text) => json_text_setter.set(json_text),
                        Err(e) => log::error!("{}", e),
                    }
                }
            },
            (prevent_changes, values_setter),
        )
    };

    let on_key_up = {
        let json_text_setter = json_text.setter().clone();
        let text_area_ref = text_area_ref.clone();
        use_callback(
            move |_, (json_text_setter, text_area_ref)| {
                if let Some(elm) = text_area_ref.cast::<HtmlTextAreaElement>() {
                    json_text_setter.set(elm.value())
                }
            },
            (json_text_setter, text_area_ref),
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

    {
        let values_setter = values.setter().clone();
        let error_setter = error.setter().clone();
        let json_text = json_text.clone();
        use_effect_with_deps(
            |(json_text, values_setter, error_setter)| {
                match parse_flow_json_text_to_values(json_text) {
                    Ok(values) => {
                        values_setter.set(values);
                        error_setter.set(None);
                    }
                    Err(e) => {
                        error_setter.set(Some(e.to_string()));
                        log::error!("{:?}", e);
                    }
                };
                || ()
            },
            (json_text, values_setter, error_setter),
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
