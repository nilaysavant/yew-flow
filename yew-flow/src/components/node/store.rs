use std::{cell::RefCell, rc::Rc};

use colorsys::Hsl;
use yew::prelude::*;

use crate::constants::{NODE_HEIGHT, NODE_WIDTH};

use super::models::{Node, NodeInput, NodeOutput};

pub struct MoveCmd {
    pub id: usize,
    pub x: u64,
    pub y: u64,
}

pub struct MoveActiveCmd {
    pub x: u64,
    pub y: u64,
}

pub enum NodesAction {
    Move(MoveCmd),
    MoveActive(MoveActiveCmd),
    Activate(usize),
    Deactivate(usize),
}

#[derive(Clone, PartialEq, Debug)]
pub struct NodesState {
    pub nodes: Vec<Node>,
}

impl Default for NodesState {
    fn default() -> Self {
        // Generate a grid of nodes
        let auto_incr_id = Rc::new(RefCell::new(0..));
        let nodes = (0..5)
            .into_iter()
            .map(move |i| {
                let auto_incr_id = auto_incr_id.clone();
                (0..5).into_iter().map(move |j| {
                    let id = auto_incr_id.clone().borrow_mut().next().unwrap();
                    let mut color = Hsl::new(0., 100., 50., Some(0.8));
                    color.set_hue(360. / 15. * ((i * j) as f64));
                    Node {
                        id,
                        title: format!("Node {}", id),
                        x: ((NODE_WIDTH as usize + 10) * i) as u64,
                        y: ((NODE_HEIGHT as usize + 10) * j) as u64,
                        color,
                        is_active: false,
                        inputs: (0..3)
                            .into_iter()
                            .map(|input_id| NodeInput {
                                id: format!("node:{} input:{}", id, input_id),
                            })
                            .collect(),
                        outputs: (0..3)
                            .into_iter()
                            .map(|output_id| NodeOutput {
                                id: format!("node:{} input:{}", id, output_id),
                            })
                            .collect(),
                    }
                })
            })
            .flatten()
            .collect::<Vec<Node>>();
        Self { nodes }
    }
}

impl Reducible for NodesState {
    type Action = NodesAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut nodes = self.nodes.clone();
        let updated_nodes = match action {
            NodesAction::Move(MoveCmd { id, x, y }) => {
                let node = nodes.iter_mut().find(|a| a.id == id);
                if let Some(node) = node {
                    node.x = x;
                    node.y = y;
                }
                nodes
            }
            NodesAction::MoveActive(MoveActiveCmd { x, y }) => {
                let active_node = nodes.iter_mut().find(|n| n.is_active);
                if let Some(active_node) = active_node {
                    active_node.x = x;
                    active_node.y = y;
                }
                nodes
            }
            NodesAction::Activate(id) => {
                let node = nodes.iter_mut().find(|a| a.id == id);
                if let Some(node) = node {
                    node.is_active = true
                }
                nodes
            }
            NodesAction::Deactivate(id) => {
                let node = nodes.iter_mut().find(|a| a.id == id);
                if let Some(node) = node {
                    node.is_active = false
                }
                nodes
            }
        };

        Self {
            nodes: updated_nodes,
        }
        .into()
    }
}
