use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::{
    components::{
        edge::{models::Edge, render_edge_list::RenderEdgeList},
        node::{models::Node, render_node_list::RenderNodeList},
        viewport::models::Viewport,
    },
    constants::{NODE_HEIGHT, NODE_WIDTH},
    store::{
        DragEdgeCmd, DragNodeCmd, InteractionMode, NewEdgeDragDeactivateCmd, WorkspaceAction,
        WorkspaceStore,
    },
};

/// # Initial State
///
/// Initial state of the workspace.
#[derive(Debug, Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct YewFlowValues {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProps {
    pub values: YewFlowValues,
    #[serde(skip)]
    pub on_change: Callback<YewFlowValues>,
}

/// # Yew Flow Workspace
///
/// `yew-flow` canvas/work area where nodes
/// are rendered.
#[function_component(Workspace)]
pub fn workspace(WorkspaceProps { values, on_change }: &WorkspaceProps) -> Html {
    let container_ref = use_node_ref();
    let store = use_reducer(|| WorkspaceStore {
        nodes: values.nodes.clone(),
        edges: values.edges.clone(),
        ..Default::default()
    });
    let dispatcher = store.dispatcher();
    log::info!("store.interaction_mode: {:?}", store.interaction_mode);

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

    {
        let nodes = store.nodes.clone();
        let edges = store.edges.clone();
        let on_change = on_change.clone();
        use_effect_with_deps(
            // Re-run this on every change of nodes/edges to send the new values back to parent
            |(nodes, edges, on_change)| {
                on_change.emit(YewFlowValues {
                    nodes: nodes.clone(),
                    edges: edges.clone(),
                });
                || ()
            },
            (nodes, edges, on_change),
        )
    }

    html! {
        <div
            class={classes!(
                "flex",
                "flex-col",
                "min-h-0",
                "p-2",
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
                <RenderNodeList
                    nodes={store.nodes.clone()}
                    interaction_mode={store.interaction_mode.clone()}
                    dispatcher={dispatcher.clone()}
                />
               <RenderEdgeList
                    edges={store.edges.clone()}
                />
            </div>
        </div>
    }
}
