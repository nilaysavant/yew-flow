use yew::prelude::*;

use yew_flow::Workspace;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Workspace />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
