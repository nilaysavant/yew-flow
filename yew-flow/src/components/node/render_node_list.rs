use web_sys::HtmlElement;
use yew::prelude::*;

use crate::constants::{NODE_HEIGHT, NODE_WIDTH};

use super::{
    models::Node,
    render_node::RenderNode,
    store::{MoveActiveCmd, NodesAction, NodesState},
};

/// Used to store container dimensions like
/// **offsets**, **width**, **height** etc
#[derive(Debug, Clone, Copy)]
struct ContainerDimensions {
    offset_left: i32,
    offset_top: i32,
    width: i32,
    height: i32,
}

impl Default for ContainerDimensions {
    fn default() -> Self {
        Self {
            offset_left: Default::default(),
            offset_top: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodeListProps {}

#[function_component(RenderNodeList)]
pub fn render_node_list(RenderNodeListProps {}: &RenderNodeListProps) -> Html {
    // log::info!("render_nodes");
    let container_ref = use_node_ref();
    let nodes_store = use_reducer(NodesState::default);
    let dispatcher = nodes_store.dispatcher();

    let on_container_mouse_move = {
        let container_ref = container_ref.clone();
        let nodes_store = nodes_store.clone();
        let mut container_dimensions = ContainerDimensions::default();
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            // set proper container offset values
            container_dimensions.offset_left = container.offset_left();
            container_dimensions.offset_top = container.offset_top();
            container_dimensions.width = container.client_width();
            container_dimensions.height = container.client_height();
        }
        Callback::from(move |e: MouseEvent| {
            if container_dimensions.width > 0 && container_dimensions.height > 0 {
                let x = (e.page_x() - container_dimensions.offset_left - NODE_WIDTH / 2)
                    .clamp(0, container_dimensions.width - NODE_WIDTH)
                    as u64;
                let y = (e.page_y() - container_dimensions.offset_top - NODE_HEIGHT / 2)
                    .clamp(0, container_dimensions.height - NODE_HEIGHT)
                    as u64;
                nodes_store.dispatch(NodesAction::MoveActive(MoveActiveCmd { x, y }))
            }
        })
    };

    let on_node_mouse_down = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| dispatcher.dispatch(NodesAction::Activate(node.id)))
    });
    let on_node_mouse_up = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| dispatcher.dispatch(NodesAction::Deactivate(node.id)))
    });
    let on_node_click = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| {
            // if node.is_active {
            //     nodes_store.dispatch(NodesAction::Deactivate(node.id));
            // } else {
            //     nodes_store.dispatch(NodesAction::Activate(node.id));
            // }
        })
    });

    let render_nodes = {
        nodes_store
            .nodes
            .iter()
            .map(|node| {
                html! {
                    <RenderNode
                        node={node.clone()}
                        on_mouse_down={on_node_mouse_down.clone()}
                        on_mouse_up={on_node_mouse_up.clone()}
                        on_click={on_node_click.clone()}
                    />
                }
            })
            .collect::<Html>()
    };

    html! {
        <div
            class={classes!(
                "flex",
                "flex-col",
                "min-h-0",
                "p-4")}
        >
            <div
                ref={container_ref}
                class={classes!(
                    "text-neutral-50",
                    "bg-neutral-800",
                    "rounded-sm",
                    "border-neutral-400",
                    "border-2",
                    "relative",
                )}
                style={format!("width: 100%; height: 400px;")}
                onmousemove={on_container_mouse_move}
            >
                {render_nodes}
            </div>
        </div>
    }
}
