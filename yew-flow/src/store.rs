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
};

pub struct DragNodeCmd {
    pub x: i32,
    pub y: i32,
}

/// # Node Connectors
///
/// Node connectors. Either input or output.
#[derive(Debug, Clone, PartialEq)]
pub enum Connector {
    /// Input connector.
    Input,
    /// Output connector.
    Output,
}

pub struct NewEdgeDragActivateCmd {
    pub viewport: Viewport,
    /// reference to the from connector
    pub from_reference: NodeRef,
    /// Type of from connector
    pub from_connector: Connector,
}

pub struct NewEdgeDragDeactivateCmd {
    pub viewport: Option<Viewport>,
    /// reference to the to connector
    pub to_reference: Option<NodeRef>,
}

pub struct DragEdgeCmd {
    // x cord to which dragged edge is ending.
    pub x: i32,
    // y cord to which dragged edge is ending.
    pub y: i32,
}

/// # Yew Flow Workspace Action
///
/// Actions to be dispatched to `WorkspaceStore`.
pub enum WorkspaceAction {
    /// Init/Re-init store
    Init,
    /// When node drag needs to be activated.
    NodeDragActivate(usize),
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
    NodeDrag(usize),
    /// New Edge drag mode.
    NewEdgeDrag(NewEdgeDragMode),
}

/// # Yew Flow Workspace Store
///
/// Main state/store for `yew-flow`.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkspaceStore {
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
                        id,
                        title: format!("Node {}", id),
                        x: ((NODE_WIDTH as usize + 10) * i) as i32,
                        y: ((NODE_HEIGHT as usize + 10) * j) as i32,
                        color,
                        inputs: (0..3)
                            .into_iter()
                            .map(|input_id| NodeInput {
                                id: format!("node:{} input:{}", id, input_id),
                                reference: NodeRef::default(),
                            })
                            .collect(),
                        outputs: (0..3)
                            .into_iter()
                            .map(|output_id| NodeOutput {
                                id: format!("node:{} input:{}", id, output_id),
                                reference: NodeRef::default(),
                            })
                            .collect(),
                    }
                })
            })
            .flatten()
            .collect::<Vec<Node>>();
        Self {
            nodes,
            edges: vec![],
            interaction_mode: InteractionMode::None,
        }
    }
}

impl Reducible for WorkspaceStore {
    type Action = WorkspaceAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut nodes = self.nodes.clone();
        let mut edges = self.edges.clone();
        let mut interaction_mode = self.interaction_mode.clone();
        match action {
            WorkspaceAction::Init => Self::default().into(),
            WorkspaceAction::DragNode(DragNodeCmd { x, y }) => {
                if let InteractionMode::NodeDrag(id) = interaction_mode {
                    let active_node = nodes.iter_mut().find(|n| n.id == id);
                    if let Some(active_node) = active_node {
                        active_node.x = x;
                        active_node.y = y;
                    }
                }
                Self {
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NodeDragActivate(id) => {
                interaction_mode = InteractionMode::NodeDrag(id);
                Self {
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NodeDragDeactivate => {
                interaction_mode = InteractionMode::None;
                Self {
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NewEdgeDragActivate(NewEdgeDragActivateCmd {
                viewport,
                from_reference,
                from_connector,
            }) => {
                interaction_mode = InteractionMode::NewEdgeDrag(NewEdgeDragMode { from_connector });
                if let Some(elm) = from_reference.cast::<Element>() {
                    if viewport.dimensions.width > 0 && viewport.dimensions.height > 0 {
                        let x1 = elm.get_bounding_client_rect().x() as i32;
                        let y1 = elm.get_bounding_client_rect().y() as i32;
                        let x1 = viewport.relative_x_pos_from_abs(x1, None);
                        let y1 = viewport.relative_y_pos_from_abs(y1, None);
                        let mut new_edge = Edge {
                            x1,
                            y1,
                            x2: x1,
                            y2: y1,
                            ..Default::default()
                        };
                        if let Some(edge) = edges.last() {
                            new_edge.id = edge.id + 1;
                        }
                        edges.push(new_edge);
                    }
                }
                Self {
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
                            Connector::Input => {
                                // not sure why its vice versa (will have to figure this out)
                                // its supp to be assigned to x2,y2 and for output it should be x1,y1
                                edge.x1 = x;
                                edge.y1 = y;
                            }
                            Connector::Output => {
                                edge.x2 = x;
                                edge.y2 = y;
                            }
                        }
                    }
                }
                Self {
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
            WorkspaceAction::NewEdgeDragDeactivate(NewEdgeDragDeactivateCmd {
                viewport,
                to_reference,
            }) => {
                if let InteractionMode::NewEdgeDrag(NewEdgeDragMode { ref from_connector }) =
                    interaction_mode
                {
                    if let Some(to_reference) = to_reference {
                        if let Some(elm) = to_reference.cast::<Element>() {
                            if let Some(viewport) = viewport {
                                if viewport.dimensions.width > 0 && viewport.dimensions.height > 0 {
                                    let x = elm.get_bounding_client_rect().x() as i32;
                                    let y = elm.get_bounding_client_rect().y() as i32;
                                    let x = viewport.relative_x_pos_from_abs(x, None);
                                    let y = viewport.relative_y_pos_from_abs(y, None);
                                    if let Some(edge) = edges.last_mut() {
                                        match from_connector {
                                            Connector::Input => {
                                                // not sure why its vice versa (will have to figure this out)
                                                // its supp to be assigned to x2,y2 and for output it should be x1,y1
                                                edge.x1 = x;
                                                edge.y1 = y;
                                            }
                                            Connector::Output => {
                                                edge.x2 = x;
                                                edge.y2 = y;
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
                    nodes,
                    edges,
                    interaction_mode,
                }
                .into()
            }
        }
    }
}
