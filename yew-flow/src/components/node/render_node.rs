use yew::prelude::*;

use crate::constants::{NODE_HEIGHT, NODE_WIDTH};

use super::models::{Node, NodeInput, NodeOutput};

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodeProps {
    pub node: Node,
    pub on_mouse_down: Callback<Node>,
    pub on_input_mouse_down: Callback<NodeInput>,
    pub on_input_mouse_up: Callback<NodeInput>,
    pub on_output_mouse_down: Callback<NodeOutput>,
    pub on_output_mouse_up: Callback<NodeOutput>,
    pub on_mouse_up: Callback<Node>,
    pub on_click: Callback<Node>,
}

#[function_component(RenderNode)]
pub fn render_node(
    RenderNodeProps {
        node,
        on_mouse_down,
        on_mouse_up,
        on_click,
        on_input_mouse_down,
        on_input_mouse_up,
        on_output_mouse_down,
        on_output_mouse_up,
    }: &RenderNodeProps,
) -> Html {
    // log::info!("render_node: {}", node.id);
    let render_inputs = node
        .inputs
        .iter()
        .map(|input| {
            let handle_mouse_down = {
                let on_input_mouse_down = on_input_mouse_down.clone();
                let input = input.clone();
                Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    on_input_mouse_down.emit(input.clone())
                })
            };
            let handle_mouse_up = {
                let on_input_mouse_up = on_input_mouse_up.clone();
                let input = input.clone();
                Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    on_input_mouse_up.emit(input.clone())
                })
            };
            html! {
                <span
                    key={input.id.clone()}
                    ref={input.reference.clone()}
                    onmousedown={handle_mouse_down}
                    onmouseup={handle_mouse_up}
                    class={classes!(
                        "bg-neutral-600",
                        "border-2",
                        "border-neutral-100",
                        "w-3",
                        "h-3",
                        "rounded-full",
                        "my-1",
                        )}
                />
            }
        })
        .collect::<Html>();
    let render_outputs = node
        .outputs
        .iter()
        .map(|output| {
            let handle_mouse_down = {
                let on_output_mouse_down = on_output_mouse_down.clone();
                let output = output.clone();
                Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    on_output_mouse_down.emit(output.clone())
                })
            };
            let handle_mouse_up = {
                let on_output_mouse_up = on_output_mouse_up.clone();
                let output = output.clone();
                Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    on_output_mouse_up.emit(output.clone())
                })
            };
            html! {
                <span
                    key={output.id.clone()}
                    ref={output.reference.clone()}
                    onmousedown={handle_mouse_down}
                    onmouseup={handle_mouse_up}
                    class={classes!(
                        "bg-neutral-600",
                        "border-2",
                        "border-neutral-100",
                        "w-3",
                        "h-3",
                        "rounded-full",
                        "my-1",
                        )}
                />
            }
        })
        .collect::<Html>();

    let handle_mouse_down = {
        let on_mouse_down = on_mouse_down.clone();
        let node = node.clone();
        Callback::from(move |_| on_mouse_down.emit(node.clone()))
    };
    let handle_mouse_up = {
        let on_mouse_up = on_mouse_up.clone();
        let node = node.clone();
        Callback::from(move |_| on_mouse_up.emit(node.clone()))
    };
    let handle_click = {
        let on_click = on_click.clone();
        let node = node.clone();
        Callback::from(move |_| on_click.emit(node.clone()))
    };

    let mut bg_color = node.color.clone();
    bg_color.set_lightness(25.);
    bg_color.set_saturation(50.);
    html! {
        <div
            key={node.id.clone()}
            onmousedown={handle_mouse_down}
            onmouseup={handle_mouse_up}
            onclick={handle_click}
            style={format!("width: {width}px; height: {height}px; left: {left}px; top: {top}px; border-color: {border_color}; background: {background};",
                width = NODE_WIDTH,
                height = NODE_HEIGHT,
                left = node.x,
                top = node.y,
                border_color = node.color.to_css_string(),
                background = bg_color.to_css_string(),
            )}
            class={classes!(
                "absolute",
                "border-2",
                "rounded-lg",
                "flex",
            )}
        >
            <div class={classes!(
                "w-full",
                "flex",
                "items-center",
                "justify-center",
                "select-none",
                "relative",
            )}
            >
                // inputs
                <span class={classes!(
                    "absolute",
                    "flex",
                    "flex-col",
                    "justify-center",
                    "h-full",
                    "-left-2",
                    )}
                >
                   {render_inputs}
                </span>
                // outputs
                <span class={classes!(
                    "absolute",
                    "flex",
                    "flex-col",
                    "justify-center",
                    "h-full",
                    "-right-2",
                    )}
                >
                   {render_outputs}
                </span>
                {format!("{}", node.title)}
                <br />
                {format!("({},{})", node.x, node.y)}
            </div>
        </div>
    }
}
