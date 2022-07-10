use yew::prelude::*;

use crate::{
    components::{
        edge::{models::Edge, render_edge::RenderEdge},
        viewport::{self, models::Viewport},
    },
    constants::{NODE_HEIGHT, NODE_WIDTH},
    store::{
        Connector, DragEdgeCmd, DragNodeCmd, InteractionMode, NewEdgeDragActivateCmd,
        NewEdgeDragDeactivateCmd, NewEdgeDragMode, WorkspaceAction, WorkspaceStore,
    },
};

use super::{
    models::{Node, NodeInput, NodeOutput},
    render_node::RenderNode,
};

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodeListProps {}

#[function_component(RenderNodeList)]
pub fn render_node_list(RenderNodeListProps {}: &RenderNodeListProps) -> Html {
    let container_ref = use_node_ref();
    let store = use_reducer(WorkspaceStore::default);
    let dispatcher = store.dispatcher();
    log::info!("store.interaction_mode: {:?}", store.interaction_mode);
    // log::info!(
    //     "store.edge[0]: {:?}",
    //     store
    //         .edges
    //         .last()
    //         .map(|e| (e.from_output.as_ref(), e.to_input.as_ref()))
    // );

    let on_container_mouse_move = {
        let container_ref = container_ref.clone();
        let dispatcher = dispatcher.clone();
        let interaction_mode = store.interaction_mode.clone();
        use_callback(
            move |e: MouseEvent, (container_ref, dispatcher, interaction_mode)| {
                let viewport = Viewport::new(container_ref.clone());
                if viewport.dimensions.width > 0. && viewport.dimensions.height > 0. {
                    match interaction_mode {
                        InteractionMode::None => {
                            // dispatcher.dispatch(WorkspaceAction::DragNode(DragNodeCmd { x, y }))
                        }
                        InteractionMode::NodeDrag(_) => {
                            let x = viewport
                                .relative_x_pos_from_abs(e.page_x().into(), Some(NODE_WIDTH));
                            let y = viewport
                                .relative_y_pos_from_abs(e.page_y().into(), Some(NODE_HEIGHT));
                            dispatcher.dispatch(WorkspaceAction::DragNode(DragNodeCmd { x, y }))
                        }
                        InteractionMode::NewEdgeDrag(_) => {
                            let x = viewport.relative_x_pos_from_abs(e.page_x().into(), None);
                            let y = viewport.relative_y_pos_from_abs(e.page_y().into(), None);
                            dispatcher.dispatch(WorkspaceAction::DragEdge(DragEdgeCmd { x, y }))
                        }
                    }
                }
            },
            (container_ref, dispatcher, interaction_mode),
        )
    };
    let on_container_mouse_up = {
        let dispatcher = dispatcher.clone();
        let interaction_mode = store.interaction_mode.clone();
        use_callback(
            move |_: MouseEvent, (dispatcher, interaction_mode)| {
                match interaction_mode {
                    InteractionMode::None => {
                        // dispatch(WorkspaceAction::DragNode(DragNodeCmd { x, y }))
                    }
                    InteractionMode::NodeDrag(_) => {
                        dispatcher.dispatch(WorkspaceAction::NodeDragDeactivate)
                    }
                    InteractionMode::NewEdgeDrag(_) => dispatcher.dispatch(
                        WorkspaceAction::NewEdgeDragDeactivate(NewEdgeDragDeactivateCmd {
                            to_reference: None,
                            to_connector: None,
                        }),
                    ),
                }
            },
            (dispatcher, interaction_mode),
        )
    };
    let on_node_mouse_down = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |node: Node, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NodeDragActivate(node.id))
            },
            dispatcher,
        )
    };
    let on_node_mouse_up = {
        let dispatcher = dispatcher.clone();
        let interaction_mode = store.interaction_mode.clone();
        use_callback(
            move |node: Node, (dispatcher, interaction_mode)| {
                match interaction_mode {
                    InteractionMode::None => {
                        // dispatch(WorkspaceAction::DragNode(DragNodeCmd { x, y }))
                    }
                    InteractionMode::NodeDrag(_) => {
                        dispatcher.dispatch(WorkspaceAction::NodeDragDeactivate)
                    }
                    InteractionMode::NewEdgeDrag(_) => dispatcher.dispatch(
                        WorkspaceAction::NewEdgeDragDeactivate(NewEdgeDragDeactivateCmd {
                            to_reference: None,
                            to_connector: None,
                        }),
                    ),
                }
            },
            (dispatcher, interaction_mode),
        )
    };
    let on_node_click = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |node: Node, _| {
                // if node.is_active {
                //     store.dispatch(NodesAction::Deactivate(node.id));
                // } else {
                //     store.dispatch(NodesAction::Activate(node.id));
                // }
            },
            (),
        )
    };
    let on_node_input_mouse_down = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |input: NodeInput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragActivate(
                    NewEdgeDragActivateCmd {
                        from_reference: input.reference,
                        from_connector: Connector::Input(input.id),
                    },
                ))
            },
            dispatcher,
        )
    };
    let on_node_input_mouse_up = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |input: NodeInput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragDeactivate(
                    NewEdgeDragDeactivateCmd {
                        to_reference: Some(input.reference),
                        to_connector: Some(Connector::Input(input.id)),
                    },
                ))
            },
            dispatcher,
        )
    };
    let on_node_output_mouse_down = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |output: NodeOutput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragActivate(
                    NewEdgeDragActivateCmd {
                        from_reference: output.reference,
                        from_connector: Connector::Output(output.id),
                    },
                ))
            },
            dispatcher,
        )
    };
    let on_node_output_mouse_up = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |output: NodeOutput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragDeactivate(
                    NewEdgeDragDeactivateCmd {
                        to_reference: Some(output.reference),
                        to_connector: Some(Connector::Output(output.id)),
                    },
                ))
            },
            dispatcher,
        )
    };

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
        store
            .edges
            .clone()
            .iter()
            .map(|edge| {
                html! {
                    <RenderEdge
                        edge={edge.clone()}
                    />
                }
            })
            .collect::<Html>()
    };

    {
        let container_ref = container_ref.clone();
        let dispatcher = dispatcher.clone();
        use_effect_with_deps(
            // Re-run this on every change of container_ref
            move |container_ref| {
                let viewport = Viewport::new(container_ref.clone());
                if viewport.dimensions.width > 0. && viewport.dimensions.height > 0. {
                    // Re-init the workspace as container/viewport has changed
                    dispatcher.dispatch(WorkspaceAction::Init(viewport.clone()));
                }
                || ()
            },
            container_ref,
        )
    }

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
                onmouseup={on_container_mouse_up}
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
