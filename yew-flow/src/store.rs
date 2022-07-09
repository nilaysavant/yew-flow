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
    /// Init/Re-init store
    Init,
    /// When active node needs to be moved.
    ActiveNodeMove(ActiveNodeMoveCmd),
    /// When node drag needs to be activated.
    NodeDragActivate(usize),
    /// When node drag needs to be deactivated.
    NodeDragDeactivate,
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
        let mut interaction_mode = self.interaction_mode.clone();
        match action {
            WorkspaceAction::Init => Self::default().into(),
            WorkspaceAction::ActiveNodeMove(ActiveNodeMoveCmd { x, y }) => {
                if let InteractionMode::NodeDrag(id) = interaction_mode {
                    let active_node = nodes.iter_mut().find(|n| n.id == id);
                    if let Some(active_node) = active_node {
                        active_node.x = x;
                        active_node.y = y;
                    }
                }
                Self {
                    nodes,
                    interaction_mode,
                    edges: self.edges.clone(),
                }
                .into()
            }
            WorkspaceAction::NodeDragActivate(id) => {
                interaction_mode = InteractionMode::NodeDrag(id);
                Self {
                    nodes,
                    interaction_mode,
                    edges: self.edges.clone(),
                }
                .into()
            }
            WorkspaceAction::NodeDragDeactivate => {
                interaction_mode = InteractionMode::None;
                Self {
                    nodes,
                    interaction_mode,
                    edges: self.edges.clone(),
                }
                .into()
            }
        }
    }
}
