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
    pub prevent_changes: bool,
    #[serde(skip)]
    pub on_change: Callback<YewFlowValues>,
}

/// # Yew Flow Workspace
///
/// `yew-flow` canvas/work area where nodes
/// are rendered.
#[function_component(Workspace)]
pub fn workspace(
    WorkspaceProps {
        values,
        prevent_changes,
        on_change,
    }: &WorkspaceProps,
) -> Html {
    let container_ref = use_node_ref();
    let store = use_reducer(|| WorkspaceStore {
        nodes: values.nodes.clone(),
        edges: values.edges.clone(),
        ..Default::default()
    });
    let dispatcher = store.dispatcher();
    log::info!("store.lastNode: {:?}", store.nodes.last());
    log::info!("prevent: {:?}", prevent_changes.clone());

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
        let dispatcher = dispatcher.clone();
        let values = values.clone();
        let prevent_changes = prevent_changes.clone();
        use_effect_with_deps(
            // Re-run this on every change of container_ref
            move |(values, prevent_changes, dispatcher)| {
                if *prevent_changes {
                    // Re-init the workspace with changed
                    dispatcher.dispatch(WorkspaceAction::Init(Some(values.clone())));
                } else {
                    // Re-init the workspace with default values
                    dispatcher.dispatch(WorkspaceAction::Init(None));
                }
                || ()
            },
            (values, prevent_changes, dispatcher),
        )
    }

    {
        let container_ref = container_ref.clone();
        let dispatcher = dispatcher.clone();
        use_effect_with_deps(
            move |(container_ref, dispatcher)| {
                let viewport = Viewport::new(container_ref.clone());
                if viewport.dimensions.width > 0. && viewport.dimensions.height > 0. {
                    // Change viewport
                    dispatcher.dispatch(WorkspaceAction::ViewPortChange(viewport));
                }
                || ()
            },
            (container_ref, dispatcher),
        )
    }

    {
        let nodes = store.nodes.clone();
        let edges = store.edges.clone();
        let on_change = on_change.clone();
        let prevent_changes = prevent_changes.clone();
        use_effect_with_deps(
            // Re-run this on every change of nodes/edges to send the new values back to parent
            |(nodes, edges, on_change, prevent_changes)| {
                if !prevent_changes {
                    on_change.emit(YewFlowValues {
                        nodes: nodes.clone(),
                        edges: edges.clone(),
                    });
                }
                || ()
            },
            (nodes, edges, on_change, prevent_changes),
        )
    }

    html! {
        <div
            class={classes!(
                "flex-1",
                "min-h-0",
                "flex",
                "flex-col",
            )}
        >
            <div
                ref={container_ref}
                class={classes!(
                    "flex-1",
                    "min-h-0",
                    "flex",
                    "flex-col",
                    "w-full",
                    "text-neutral-50",
                    "bg-neutral-800",
                    "rounded-sm",
                    "border-neutral-400",
                    "border-2",
                    "relative",
                )}
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
