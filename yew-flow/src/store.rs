use std::{cell::RefCell, rc::Rc};

use colorsys::Hsl;
use web_sys::Element;
use yew::prelude::*;

use crate::{
    components::{
        edge::models::Edge,
        node::models::{Node, NodeInput, NodeOutput},
        viewport::models::Viewport,
    },
    constants::{NODE_HEIGHT, NODE_WIDTH},
    types::{
        standard_id::{IdentifierExt, StandardId},
        standard_unit::StandardUnit,
    },
};

pub struct DragNodeCmd {
    pub x: StandardUnit,
    pub y: StandardUnit,
}

/// # Node Connectors
///
/// Node connectors. Either input or output.
#[derive(Debug, Clone, PartialEq)]
pub enum Connector {
    /// Input connector. Takes id of the input.
    Input(StandardId),
    /// Output connector. Takes id of the output.
    Output(StandardId),
}

pub struct NewEdgeDragActivateCmd {
    /// reference to the from connector
    pub from_reference: NodeRef,
    /// Type of from connector
    pub from_connector: Connector,
}

pub struct NewEdgeDragDeactivateCmd {
    /// reference to the to connector
    pub to_reference: Option<NodeRef>,
    /// Type of to connector
    pub to_connector: Option<Connector>,
}

pub struct DragEdgeCmd {
    // x cord to which dragged edge is ending.
    pub x: StandardUnit,
    // y cord to which dragged edge is ending.
    pub y: StandardUnit,
}

/// # Yew Flow Workspace Action
///
/// Actions to be dispatched to `WorkspaceStore`.
pub enum WorkspaceAction {
    /// Init/Re-init store
    Init(Viewport),
    /// When node drag needs to be activated.
    NodeDragActivate(StandardId),
    /// When node needs to be dragged.
    DragNode(DragNodeCmd),
    /// When node drag needs to be deactivated.
    NodeDragDeactivate,
    /// When new edge drag needs to be activated.
    NewEdgeDragActivate(NewEdgeDragActivateCmd),
    /// When new edge needs to be dragged out.
    DragEdge(DragEdgeCmd),
    /// When new edge drag needs to be deactivated.
    NewEdgeDragDeactivate(NewEdgeDragDeactivateCmd),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewEdgeDragMode {
    pub from_connector: Connector,
}

/// # User Interaction Mode
///
/// Enum of all modes of user interaction with the
/// flow workspace.
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionMode {
    /// No interaction mode.
    None,
    /// Node drag mode. Pass `node_id` of node being dragged.
    NodeDrag(StandardId),
    /// New Edge drag mode.
    NewEdgeDrag(NewEdgeDragMode),
}

/// # Yew Flow Workspace Store
///
/// Main state/store for `yew-flow`.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkspaceStore {
    pub viewport: Option<Viewport>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub interaction_mode: InteractionMode,
}

impl Default for WorkspaceStore {
    fn default() -> Self {
        // Generate a grid of nodes
        let auto_incr_id = Rc::new(RefCell::new(0..));
        let nodes = (0..1)
            .into_iter()
            .map(move |i| {
                let auto_incr_id = auto_incr_id.clone();
                (0..6).into_iter().map(move |j| {
                    let id = auto_incr_id.clone().borrow_mut().next().unwrap();
                    let mut color = Hsl::new(0., 100., 50., Some(0.8));
                    color.set_hue(360. / 15. * ((i * j) as f64));
                    Node {
                        id: StandardId::generate(),
                        title: format!("Node {}", id),
                        x: ((NODE_WIDTH + 10.) * i as f64) as StandardUnit,
                        y: ((NODE_HEIGHT + 10.) * j as f64) as StandardUnit,
                        color,
                        inputs: (0..3)
                            .into_iter()
                            .map(|input| NodeInput {
                                id: format!("node-{}--input-{}", id, input),
                                reference: NodeRef::default(),
                            })
                            .collect(),
                        outputs: (0..3)
                            .into_iter()
                            .map(|output| NodeOutput {
                                id: format!("node-{}--output-{}", id, output),
                                reference: NodeRef::default(),
                            })
                            .collect(),
                    }
                })
            })
            .flatten()
            .collect::<Vec<Node>>();
        Self {
            viewport: None,
            nodes,
            edges: (0..2)
                .into_iter()
                .map(|i| Edge {
                    id: StandardId::generate(),
                    from_output: Some(format!("node-{}--output-{}", i, 0)),
                    to_input: Some(format!("node-{}--input-{}", i + 1, 0)),
                    ..Default::default()
                })
                .collect(),
            interaction_mode: InteractionMode::None,
        }
    }
}

impl Reducible for WorkspaceStore {
    type Action = WorkspaceAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let viewport = self.viewport.clone();
        let mut nodes = self.nodes.clone();
        let mut edges = self.edges.clone();
        let mut interaction_mode = self.interaction_mode.clone();
        match action {
            WorkspaceAction::Init(viewport) => Self {
                viewport: Some(viewport),
                ..Default::default()
            }
            .into(),
            WorkspaceAction::DragNode(DragNodeCmd { x, y }) => {
                if let InteractionMode::NodeDrag(ref id) = interaction_mode {
                    let active_node = nodes.iter_mut().find(|n| n.id == *id);
                    if let Some(active_node) = active_node {
                        active_node.x = x; // assign new coord values
                        active_node.y = y;
                        let Node {
                            inputs, outputs, ..
                        } = active_node;
                        if let Some(ref viewport) = viewport {
                            for output in outputs.iter() {
                                edges.iter_mut().for_each(|edge| {
                                    if edge.from_output == Some(output.id.clone()) {
                                        if let Some(elm) = output.reference.cast::<Element>() {
                                            let x = elm.get_bounding_client_rect().x();
                                            let y = elm.get_bounding_client_rect().y();
                                            let x = viewport.relative_x_pos_from_abs(x, None);
                                            let y = viewport.relative_y_pos_from_abs(y, None);
                                            edge.x1 = x;
                                            edge.y1 = y;
                                        }
                                    }
                                });
                            }
                            for input in inputs.iter() {
                                edges.iter_mut().for_each(|edge| {
                                    if edge.to_input == Some(input.id.clone()) {
                                        if let Some(elm) = input.reference.cast::<Element>() {
                                            let x = elm.get_bounding_client_rect().x();
                                            let y = elm.get_bounding_client_rect().y();
                                            let x = viewport.relative_x_pos_from_abs(x, None);
                                            let y = viewport.relative_y_pos_from_abs(y, None);
                                            edge.x2 = x;
                                            edge.y2 = y;
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
                Self {
                    viewport,
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NodeDragActivate(id) => {
                interaction_mode = InteractionMode::NodeDrag(id);
                Self {
                    viewport,
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NodeDragDeactivate => {
                interaction_mode = InteractionMode::None;
                Self {
                    viewport,
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NewEdgeDragActivate(NewEdgeDragActivateCmd {
                from_reference,
                from_connector,
            }) => {
                interaction_mode = InteractionMode::NewEdgeDrag(NewEdgeDragMode {
                    from_connector: from_connector.clone(),
                });
                if let Some(elm) = from_reference.cast::<Element>() {
                    if let Some(ref viewport) = viewport {
                        if viewport.dimensions.width > 0. && viewport.dimensions.height > 0. {
                            let x = elm.get_bounding_client_rect().x();
                            let y = elm.get_bounding_client_rect().y();
                            let x = viewport.relative_x_pos_from_abs(x, None);
                            let y = viewport.relative_y_pos_from_abs(y, None);
                            let mut edge = Edge {
                                x1: x,
                                y1: y,
                                x2: x,
                                y2: y,
                                ..Default::default()
                            };
                            match from_connector {
                                Connector::Output(id) => {
                                    edge.x1 = x;
                                    edge.y1 = y;
                                    edge.from_output = Some(id.clone());
                                }
                                Connector::Input(id) => {
                                    edge.x2 = x;
                                    edge.y2 = y;
                                    edge.to_input = Some(id.clone());
                                }
                            }
                            edges.push(edge);
                        }
                    }
                }
                Self {
                    viewport,
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::DragEdge(DragEdgeCmd { x, y }) => {
                if let InteractionMode::NewEdgeDrag(NewEdgeDragMode { ref from_connector }) =
                    interaction_mode
                {
                    if let Some(edge) = edges.last_mut() {
                        match from_connector {
                            Connector::Output(id) => {
                                edge.x2 = x;
                                edge.y2 = y;
                            }
                            Connector::Input(id) => {
                                edge.x1 = x;
                                edge.y1 = y;
                            }
                        }
                    }
                }
                Self {
                    viewport,
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NewEdgeDragDeactivate(NewEdgeDragDeactivateCmd {
                to_reference,
                to_connector,
            }) => {
                if let Some(to_reference) = to_reference {
                    if let (Some(ref viewport), Some(elm), Some(to_connector), Some(edge)) = (
                        viewport.clone(),
                        to_reference.cast::<Element>(),
                        to_connector,
                        edges.last_mut(),
                    ) {
                        if viewport.dimensions.width > 0. && viewport.dimensions.height > 0. {
                            let x = elm.get_bounding_client_rect().x() as StandardUnit;
                            let y = elm.get_bounding_client_rect().y() as StandardUnit;
                            let x = viewport.relative_x_pos_from_abs(x, None);
                            let y = viewport.relative_y_pos_from_abs(y, None);
                            match to_connector {
                                Connector::Output(id) => {
                                    edge.x1 = x;
                                    edge.y1 = y;
                                    edge.from_output = Some(id.clone());
                                }
                                Connector::Input(id) => {
                                    edge.x2 = x;
                                    edge.y2 = y;
                                    edge.to_input = Some(id.clone());
                                }
                            }
                        } else {
                            // remove the temp edge if not connected
                            edges.pop();
                        }
                    } else {
                        // remove the temp edge if not connected
                        edges.pop();
                    }
                } else {
                    // remove the temp edge if not connected
                    edges.pop();
                }
                interaction_mode = InteractionMode::None; // reset interaction mode
                Self {
                    viewport,
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
        }
    }
}
