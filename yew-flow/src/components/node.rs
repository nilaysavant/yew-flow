use colorsys::Hsl;
use stylist::yew::styled_component;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::constants::{NODE_HEIGHT, NODE_WIDTH};

#[derive(Clone, PartialEq, Properties)]
pub struct Node {
    pub id: usize,
    pub title: String,
    pub x: u64,
    pub y: u64,
    pub color: Hsl,
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
                    let mut color = Hsl::new(0., 100., 50., Some(0.8));
                    color.set_hue(360. / 15. * ((i * j) as f64));
                    Node {
                        id,
                        title: format!("Node {}", id),
                        x: (400 / 4 * i) as u64,
                        y: (400 / 5 * j) as u64,
                        color,
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

/// Used to store container dimensions like
/// **offsets**, **width**, **height** etc
#[derive(Debug, Clone, Copy)]
struct ContainerDimensions {
    offset_left: i32,
    offset_top: i32,
    width: i32,
    height: i32,
}

impl Default for ContainerDimensions {
    fn default() -> Self {
        Self {
            offset_left: Default::default(),
            offset_top: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
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
        let mut container_dimensions = ContainerDimensions::default();
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            // set proper container offset values
            container_dimensions.offset_left = container.offset_left();
            container_dimensions.offset_top = container.offset_top();
            container_dimensions.width = container.offset_width();
            container_dimensions.height = container.offset_height();
        }
        Callback::from(move |e: MouseEvent| {
            let x = (e.page_x() - container_dimensions.offset_left - 40) as u64;
            let y = (e.page_y() - container_dimensions.offset_top - 25) as u64;
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

            let mut bg_color = node.color.clone();
            bg_color.set_lightness(25.);
            bg_color.set_saturation(50.);
            html! {
                <div
                    onmousedown={on_node_mouse_down}
                    onmouseup={on_node_mouse_up}
                    // onclick={on_node_click}
                    style={format!("display: flex; align-items: center; justify-content: center; border-radius: 50px; user-select: none; width: {width}px; height: {height}px; position: absolute; left: {left}px; top: {top}px; border: 3px solid {border_color}; background: {background};", 
                        width = NODE_WIDTH,
                        height = NODE_HEIGHT,
                        left = node.x,
                        top = node.y,
                        border_color = node.color.to_css_string(),
                        background = bg_color.to_css_string(),
                    )}>
                    {format!("{}", node.title)}
                    <br />
                    {format!("({},{})", node.x, node.y)}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <>
            // <button onclick={on_step_btn_click.clone()}>{"increment"}</button>
            // <p>{format!("step: {} ", *step)}</p>
            <div ref={container_ref} class={css!("color: #e0e0e0; background: #171717; border: 2px solid #949494; border-radius: 5px; width: 600px; height: 400px; position: relative; font-size: 12px; margin: 50px;")} onmousemove={on_container_mouse_move}>
                {render_nodes}
            </div>
        </>
    }
}
