use std::{cell::RefCell, rc::Rc};

use colorsys::Hsl;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

use crate::{
    components::{
        edge::{models::Edge, render_edge::RenderEdge},
        viewport::models::Viewport,
    },
    constants::{NODE_HEIGHT, NODE_WIDTH},
    store::{ActiveNodeMoveCmd, WorkspaceAction, WorkspaceStore},
};

use super::{models::Node, render_node::RenderNode};

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodeListProps {}

#[function_component(RenderNodeList)]
pub fn render_node_list(RenderNodeListProps {}: &RenderNodeListProps) -> Html {
    // log::info!("render_nodes");
    let container_ref = use_node_ref();
    let nodes_store = use_reducer(WorkspaceStore::default);
    let dispatcher = nodes_store.dispatcher();

    let on_container_mouse_move = {
        let container_ref = container_ref.clone();
        let nodes_store = nodes_store.clone();
        let viewport = Viewport::new(container_ref);
        Callback::from(move |e: MouseEvent| {
            if viewport.dimensions.width > 0 && viewport.dimensions.height > 0 {
                let x = viewport.relative_x_pos_from_abs(e.page_x(), Some(NODE_WIDTH));
                let y = viewport.relative_y_pos_from_abs(e.page_y(), Some(NODE_HEIGHT));
                nodes_store.dispatch(WorkspaceAction::ActiveNodeMove(ActiveNodeMoveCmd { x, y }))
            }
        })
    };

    let on_node_mouse_down = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| {
            dispatcher.dispatch(WorkspaceAction::NodeActivate(node.id))
        })
    });
    let on_node_mouse_up = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| {
            dispatcher.dispatch(WorkspaceAction::NodeDeactivate(node.id))
        })
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

    let render_edges = {
        let auto_id = Rc::new(RefCell::new(0..));
        log::info!("auto_id: {:?}", auto_id);
        nodes_store
            .nodes
            .clone()
            .iter()
            .zip(nodes_store.nodes.clone().iter().skip(1))
            .map(|(node1, node2)| {
                // log::info!("node1: {}, node2: {}", node1.id, node2.id);
                let (x1, y1) = node1.outputs[0]
                    .reference
                    .cast::<Element>()
                    .map_or((0, 0), |elm| {
                        let rect = elm.get_bounding_client_rect();
                        (rect.x() as i32, rect.y() as i32)
                    });
                let (x2, y2) = node2.inputs[0]
                    .reference
                    .cast::<Element>()
                    .map_or((0, 0), |elm| {
                        let rect = elm.get_bounding_client_rect();
                        (rect.x() as i32, rect.y() as i32)
                    });

                match node1.outputs[0].reference.cast::<HtmlElement>() {
                    Some(_) => {
                        let edge = Edge {
                            id: auto_id.clone().borrow_mut().next().unwrap(),
                            color: Hsl::new(0., 100., 100., Some(0.8)),
                            is_active: false,
                            x1,
                            y1,
                            x2,
                            y2,
                        };
                        log::info!("edge: {:?}", edge);
                        html! {
                            <RenderEdge
                                edge={edge.clone()}
                            />
                        }
                    }
                    None => html! {},
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
                "p-4",
                "pt-10"
            )}
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
                <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
                    // <path
                    //     d="M 100 0 C 200 0, 0 100, 100 100"
                    //     stroke="blue"
                    //     stroke-width="3px"
                    //     fill="transparent"
                    // />
                    {render_edges}
                </svg>
            </div>
        </div>
    }
}
