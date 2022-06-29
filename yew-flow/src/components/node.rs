use std::{borrow::BorrowMut, cell::RefCell};

use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Node {
    pub id: usize,
    pub title: String,
    pub x: u64,
    pub y: u64,
    pub color: String,
    pub is_active: bool,
}

pub struct MoveCmd {
    id: usize,
    x: u64,
    y: u64,
}

pub enum NodesAction {
    Move(MoveCmd),
    Activate(usize),
    Deactivate(usize),
}

pub struct NodesState {
    pub nodes: Vec<Node>,
}

impl Default for NodesState {
    fn default() -> Self {
        Self {
            nodes: vec![
                Node {
                    id: 0,
                    title: "Node 0".to_string(),
                    x: 0,
                    y: 0,
                    color: "red".to_string(),
                    is_active: false,
                },
                Node {
                    id: 1,
                    title: "Node 0".to_string(),
                    x: 0,
                    y: 100,
                    color: "blue".to_string(),
                    is_active: false,
                },
            ],
        }
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

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodesProps {}

#[styled_component(RenderNodes)]
pub fn render_nodes(RenderNodesProps {}: &RenderNodesProps) -> Html {
    let nodes_store = use_reducer(NodesState::default);

    let on_container_mouse_move = {
        let nodes_store = nodes_store.clone();
        let nodes = nodes_store.nodes.clone();
        Callback::from(move |e: MouseEvent| {
            let active_node = nodes.iter().find(|n| n.is_active);
            if let Some(Node {
                id,
                is_active: _,
                color: _,
                title: _,
                x: _,
                y: _,
            }) = active_node
            {
                nodes_store.dispatch(NodesAction::Move(MoveCmd {
                    id: *id,
                    x: (e.offset_x().clamp(40, 600) - 40) as u64,
                    y: (e.offset_y().clamp(25, 400) - 25) as u64,
                }))
            }
        })
    };

    let render_nodes = nodes_store.nodes
        .iter()
        .map(|node| {
            let node = node.clone();
            // let on_node_mouse_down = {
            //     let nodes = nodes.clone();
            //     Callback::from(move |_| {
            //         let updated = nodes
            //             .iter()
            //             .map(|node_item| {
            //                 let mut tmp = node_item.clone();
            //                 if tmp.id == node.id {
            //                     tmp.is_active = true;
            //                 }
            //                 tmp
            //             })
            //             .collect();
            //         nodes.set(updated);
            //     })
            // };
            // let on_node_mouse_up = {
            //     let nodes = nodes.clone();
            //     Callback::from(move |_| {
            //         let updated = nodes
            //             .iter()
            //         .map(|node_item| {
            //                 let mut tmp = node_item.clone();
            //                 if tmp.id == node.id {
            //                     tmp.is_active = false;
            //                 }
            //                 tmp
            //             })
            //             .collect();
            //         nodes.set(updated);
            //     })
            // };
            let on_node_click = {
                let node = node.clone();
                let nodes_store  = nodes_store.clone();
                Callback::from( move |_| {
                    let nodes = nodes_store.nodes.clone();
                    let curr_node = nodes.iter().find(|n| n.id == node.id);
                    if let Some(curr_node) = curr_node {
                        if curr_node.is_active {
                            nodes_store.dispatch(NodesAction::Deactivate(curr_node.id));
                        } else {
                            nodes_store.dispatch(NodesAction::Activate(curr_node.id));
                        }
                    }
                })
            };
            html! {
                <div
                    // onmousedown={on_node_mouse_down} 
                    // onmouseup={on_node_mouse_up}  
                    onclick={on_node_click}
                    style={format!("user-select: none; width: 80px; height: 50px; position: absolute; left: {}px; top: {}px; border: 1px solid {};", node.x, node.y, node.color)}>
                    {format!("node: {} x:{} y:{}", node.title, node.x, node.y)}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <>
            // <button onclick={on_step_btn_click.clone()}>{"increment"}</button>
            // <p>{format!("step: {} ", *step)}</p>
            <div class={css!("background: gray; width: 600px; height: 400px; position: relative; font-size: 14px;")} onmouseover={on_container_mouse_move}>
                {render_nodes}
            </div>
        </>
    }
}
