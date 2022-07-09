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
    store::{DragNodeCmd, NewEdgeDragActivateCmd, WorkspaceAction, WorkspaceStore},
};

use super::{
    models::{Node, NodeInput, NodeOutput},
    render_node::RenderNode,
};

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodeListProps {}

#[function_component(RenderNodeList)]
pub fn render_node_list(RenderNodeListProps {}: &RenderNodeListProps) -> Html {
    // log::info!("render_nodes");
    let container_ref = use_node_ref();
    let store = use_reducer(WorkspaceStore::default);
    let dispatcher = store.dispatcher();

    let on_container_mouse_move = {
        let container_ref = container_ref.clone();
        let store = store.clone();
        let viewport = Viewport::new(container_ref);
        Callback::from(move |e: MouseEvent| {
            if viewport.dimensions.width > 0 && viewport.dimensions.height > 0 {
                let x = viewport.relative_x_pos_from_abs(e.page_x(), Some(NODE_WIDTH));
                let y = viewport.relative_y_pos_from_abs(e.page_y(), Some(NODE_HEIGHT));

                store.dispatch(WorkspaceAction::DragNode(DragNodeCmd { x, y }))
            }
        })
    };

    let on_node_mouse_down = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| {
            dispatcher.dispatch(WorkspaceAction::NodeDragActivate(node.id))
        })
    });
    let on_node_mouse_up = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| dispatcher.dispatch(WorkspaceAction::NodeDragDeactivate))
    });
    let on_node_click = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| {
            // if node.is_active {
            //     store.dispatch(NodesAction::Deactivate(node.id));
            // } else {
            //     store.dispatch(NodesAction::Activate(node.id));
            // }
        })
    });
    let on_node_input_mouse_down = use_ref(|| {
        let dispatcher = dispatcher.clone();
        let container_ref = container_ref.clone();
        let viewport = Viewport::new(container_ref);
        Callback::from(move |input: NodeInput| {
            if let Some(elm) = input.reference.cast::<Element>() {
                if viewport.dimensions.width > 0 && viewport.dimensions.height > 0 {
                    let x1 = elm.get_bounding_client_rect().x() as i32;
                    let y1 = elm.get_bounding_client_rect().y() as i32;
                    let x1 = viewport.relative_x_pos_from_abs(x1, None);
                    let y1 = viewport.relative_y_pos_from_abs(y1, None);
                    dispatcher.dispatch(WorkspaceAction::NewEdgeDragActivate(
                        NewEdgeDragActivateCmd { x1, y1 },
                    ))
                }
            }
        })
    });
    let on_node_input_mouse_up = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |input: NodeInput| {})
    });
    let on_node_output_mouse_down = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |output: NodeOutput| {})
    });
    let on_node_output_mouse_up = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |output: NodeOutput| {})
    });

    let render_nodes = {
        store
            .nodes
            .iter()
            .map(|node| {
                html! {
                    <RenderNode
                        node={node.clone()}
                        on_mouse_down={on_node_mouse_down.clone()}
                        on_mouse_up={on_node_mouse_up.clone()}
                        on_click={on_node_click.clone()}
                        on_input_mouse_down={on_node_input_mouse_down.clone()}
                        on_input_mouse_up={on_node_input_mouse_up.clone()}
                        on_output_mouse_down={on_node_output_mouse_down.clone()}
                        on_output_mouse_up={on_node_output_mouse_up.clone()}
                    />
                }
            })
            .collect::<Html>()
    };

    let render_edges = {
        let container_ref = container_ref.clone();
        let viewport = Viewport::new(container_ref);
        let auto_id = Rc::new(RefCell::new(0..));
        log::info!("auto_id: {:?}", auto_id);
        store
            .nodes
            .clone()
            .iter()
            .zip(store.nodes.clone().iter().skip(1))
            .map(|(node1, node2)| {
                // Get x and y for Node1 output
                let (x1, y1) = node1.outputs[1]
                    .reference
                    .cast::<Element>()
                    .map_or((0, 0), |elm| {
                        let rect = elm.get_bounding_client_rect();
                        // Convert from abs pos to relative wrt viewport
                        let x = viewport.relative_x_pos_from_abs(rect.x() as i32, None);
                        let y = viewport.relative_y_pos_from_abs(rect.y() as i32, None) + 4; // minor adjustments for precision
                        (x, y)
                    });
                // Get x and y for Node2 input
                let (x2, y2) = node2.inputs[1]
                    .reference
                    .cast::<Element>()
                    .map_or((0, 0), |elm| {
                        let rect = elm.get_bounding_client_rect();
                        // Convert from abs pos to relative wrt viewport
                        let x = viewport.relative_x_pos_from_abs(rect.x() as i32, None);
                        let y = viewport.relative_y_pos_from_abs(rect.y() as i32, None) + 4; // minor adjustments for precision
                        (x, y)
                    });
                let edge = Edge {
                    id: auto_id.clone().borrow_mut().next().unwrap(),
                    color: Hsl::new(0., 100., 100., Some(0.8)),
                    x1,
                    y1,
                    x2,
                    y2,
                };
                html! {
                    <RenderEdge
                        edge={edge.clone()}
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
                "p-4",
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
