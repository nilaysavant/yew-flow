use std::{cell::RefCell, rc::Rc};

use colorsys::Hsl;
use yew::prelude::*;

use crate::{
    components::{
        edge::models::Edge,
        node::models::{Node, NodeInput, NodeOutput},
    },
    constants::{NODE_HEIGHT, NODE_WIDTH},
};

pub struct NodeMoveCmd {
    pub id: usize,
    pub x: i32,
    pub y: i32,
}

pub struct ActiveNodeMoveCmd {
    pub x: i32,
    pub y: i32,
}

/// # Yew Flow Workspace Action
///
/// Actions to be dispatched to `WorkspaceStore`.
pub enum WorkspaceAction {
    /// When node needs to be moved.
    NodeMove(NodeMoveCmd),
    /// When active node needs to be moved.
    ActiveNodeMove(ActiveNodeMoveCmd),
    /// When node needs to be activated.
    NodeActivate(usize),
    /// When node needs to be deactivated.
    NodeDeactivate(usize),
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
    NodeDrag,
    /// New Edge drag mode.
    NewEdgeDrag,
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
                        is_active: false,
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
        let updated_nodes = match action {
            WorkspaceAction::NodeMove(NodeMoveCmd { id, x, y }) => {
                let node = nodes.iter_mut().find(|a| a.id == id);
                if let Some(node) = node {
                    node.x = x;
                    node.y = y;
                }
                nodes
            }
            WorkspaceAction::ActiveNodeMove(ActiveNodeMoveCmd { x, y }) => {
                let active_node = nodes.iter_mut().find(|n| n.is_active);
                if let Some(active_node) = active_node {
                    active_node.x = x;
                    active_node.y = y;
                }
                nodes
            }
            WorkspaceAction::NodeActivate(id) => {
                let node = nodes.iter_mut().find(|a| a.id == id);
                if let Some(node) = node {
                    node.is_active = true
                }
                nodes
            }
            WorkspaceAction::NodeDeactivate(id) => {
                let node = nodes.iter_mut().find(|a| a.id == id);
                if let Some(node) = node {
                    node.is_active = false
                }
                nodes
            }
        };

        Self {
            nodes: updated_nodes,
            edges: vec![],
            interaction_mode: InteractionMode::None,
        }
        .into()
    }
}
