use stylist::yew::styled_component;
use web_sys::HtmlElement;
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

pub struct NodesState {
    pub nodes: Vec<Node>,
}

impl Default for NodesState {
    fn default() -> Self {
        // Generate a grid of nodes
        let nodes = (0..5)
            .into_iter()
            .map(|i| {
                (0..5).into_iter().map(move |j| {
                    let id = i * 10 + j;
                    Node {
                        id,
                        title: format!("Node {}", id),
                        x: (400 / 4 * i) as u64,
                        y: (400 / 5 * j) as u64,
                        color: "red".to_string(),
                        is_active: false,
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

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodesProps {}

#[styled_component(RenderNodes)]
pub fn render_nodes(RenderNodesProps {}: &RenderNodesProps) -> Html {
    let container_ref = use_node_ref();
    let nodes_store = use_reducer(NodesState::default);

    let on_container_mouse_move = {
        let container_ref = container_ref.clone();
        let nodes_store = nodes_store.clone();
        // set init offset value to 0 by default
        let mut offset_left = 0;
        let mut offset_top = 0;
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            // set proper container offset values
            offset_left = container.offset_left();
            offset_top = container.offset_top();
        }
        Callback::from(move |e: MouseEvent| {
            let x = (e.page_x() - offset_left - 40) as u64;
            let y = (e.page_y() - offset_top - 25) as u64;
            nodes_store.dispatch(NodesAction::MoveActive(MoveActiveCmd { x, y }))
        })
    };

    let render_nodes = nodes_store.nodes
        .iter()
        .map(|node| {
            let node = node.clone();

            let on_node_mouse_down = {
                let nodes_store = nodes_store.clone();
                Callback::from(move |_| {
                    nodes_store.dispatch(NodesAction::Activate(node.id))
                })
            };
            let on_node_mouse_up = {
                let nodes_store = nodes_store.clone();
                Callback::from(move |_| {
                    nodes_store.dispatch(NodesAction::Deactivate(node.id))
                })
            };
            // let on_node_click = {
            //     let node = node.clone();
            //     let nodes_store  = nodes_store.clone();
            //     Callback::from( move |_| {
            //         if node.is_active {
            //             nodes_store.dispatch(NodesAction::Deactivate(node.id));
            //         } else {
            //             nodes_store.dispatch(NodesAction::Activate(node.id));
            //         }
            //     })
            // };
            html! {
                <div
                    onmousedown={on_node_mouse_down}
                    onmouseup={on_node_mouse_up}
                    // onclick={on_node_click}
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
            <div ref={container_ref} class={css!("background: gray; width: 600px; height: 400px; position: relative; font-size: 14px; margin: 50px;")} onmousemove={on_container_mouse_move}>
                {render_nodes}
            </div>
        </>
    }
}
