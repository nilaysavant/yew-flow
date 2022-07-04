use yew::prelude::*;

use crate::constants::{NODE_HEIGHT, NODE_WIDTH};

use super::models::Node;

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodeProps {
    pub node: Node,
    pub on_mouse_down: std::rc::Rc<Callback<Node>>,
    pub on_mouse_up: std::rc::Rc<Callback<Node>>,
    pub on_click: std::rc::Rc<Callback<Node>>,
}

#[function_component(RenderNode)]
pub fn render_node(
    RenderNodeProps {
        node,
        on_mouse_down,
        on_mouse_up,
        on_click,
    }: &RenderNodeProps,
) -> Html {
    log::info!("render_single_node: {}", node.id);
    let render_inputs = node
        .inputs
        .iter()
        .map(|input| {
            html! {
                <span
                    key={input.id.clone()}
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
            html! {
                <span
                    key={output.id.clone()}
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
            key={node.id}
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
