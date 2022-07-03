use std::cell::RefCell;

use colorsys::Hsl;
use stylist::yew::styled_component;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::constants::{NODE_HEIGHT, NODE_WIDTH};

#[derive(Clone, PartialEq, Debug)]
pub struct NodeInput {
    pub id: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct NodeOutput {
    pub id: String,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Node {
    pub id: usize,
    pub title: String,
    pub x: u64,
    pub y: u64,
    pub color: Hsl,
    pub is_active: bool,
    pub inputs: Vec<NodeInput>,
    pub outputs: Vec<NodeOutput>,
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

#[derive(Clone, PartialEq, Debug)]
pub struct NodesState {
    pub nodes: Vec<Node>,
}

impl Default for NodesState {
    fn default() -> Self {
        // Generate a grid of nodes
        let auto_incr_id = std::rc::Rc::new(RefCell::new(0..));
        let nodes = (0..5)
            .into_iter()
            .map(move |i| {
                let auto_incr_id= auto_incr_id.clone();
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
pub struct RenderSingleNodeProps {
    node: Node,
    on_mouse_down: std::rc::Rc<Callback<Node>>,
    on_mouse_up: std::rc::Rc<Callback<Node>>,
    on_click: std::rc::Rc<Callback<Node>>,
}

#[function_component(RenderSingleNode)]
pub fn render_single_node(
    RenderSingleNodeProps {
        node,
        on_mouse_down,
        on_mouse_up,
        on_click,
    }: &RenderSingleNodeProps,
) -> Html {
    log::info!("render_single_node: {}", node.id);
    let render_inputs = node
        .inputs
        .iter()
        .map(|input| {
            html! {
                <span
                    key={input.id.clone()}
                    class={classes!(
                        "bg-neutral-600",
                        "border-2",
                        "border-neutral-100",
                        "w-3",
                        "h-3",
                        "rounded-full",
                        "my-1",
                        )}
                />
            }
        })
        .collect::<Html>();
    let render_outputs = node
        .outputs
        .iter()
        .map(|output| {
            html! {
                <span
                    key={output.id.clone()}
                    class={classes!(
                        "bg-neutral-600",
                        "border-2",
                        "border-neutral-100",
                        "w-3",
                        "h-3",
                        "rounded-full",
                        "my-1",
                        )}
                />
            }
        })
        .collect::<Html>();

    let handle_mouse_down = {
        let on_mouse_down = on_mouse_down.clone();
        let node = node.clone();
        Callback::from(move |_| on_mouse_down.emit(node.clone()))
    };
    let handle_mouse_up = {
        let on_mouse_up = on_mouse_up.clone();
        let node = node.clone();
        Callback::from(move |_| on_mouse_up.emit(node.clone()))
    };
    let handle_click = {
        let on_click = on_click.clone();
        let node = node.clone();
        Callback::from(move |_| on_click.emit(node.clone()))
    };

    let mut bg_color = node.color.clone();
    bg_color.set_lightness(25.);
    bg_color.set_saturation(50.);
    html! {
        <div
            key={node.id}
            onmousedown={handle_mouse_down}
            onmouseup={handle_mouse_up}
            onclick={handle_click}
            style={format!("width: {width}px; height: {height}px; left: {left}px; top: {top}px; border-color: {border_color}; background: {background};",
                width = NODE_WIDTH,
                height = NODE_HEIGHT,
                left = node.x,
                top = node.y,
                border_color = node.color.to_css_string(),
                background = bg_color.to_css_string(),
            )}
            class={classes!(
                "absolute",
                "border-2",
                "rounded-lg",
                "flex",
            )}
        >
            <div class={classes!(
                "w-full",
                "flex",
                "items-center",
                "justify-center",
                "select-none",
                "relative",
            )}
            >
                // inputs
                <span class={classes!(
                    "absolute",
                    "flex",
                    "flex-col",
                    "justify-center",
                    "h-full",
                    "-left-2",
                    )}
                >
                   {render_inputs}
                </span>
                // outputs
                <span class={classes!(
                    "absolute",
                    "flex",
                    "flex-col",
                    "justify-center",
                    "h-full",
                    "-right-2",
                    )}
                >
                   {render_outputs}
                </span>
                {format!("{}", node.title)}
                <br />
                {format!("({},{})", node.x, node.y)}
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodesProps {}

#[function_component(RenderNodes)]
pub fn render_nodes(RenderNodesProps {}: &RenderNodesProps) -> Html {
    // log::info!("render_nodes");
    let container_ref = use_node_ref();
    let nodes_store = use_reducer(NodesState::default);
    let dispatcher = nodes_store.dispatcher();

    let on_container_mouse_move = {
        let container_ref = container_ref.clone();
        let nodes_store = nodes_store.clone();
        let mut container_dimensions = ContainerDimensions::default();
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            // set proper container offset values
            container_dimensions.offset_left = container.offset_left();
            container_dimensions.offset_top = container.offset_top();
            container_dimensions.width = container.client_width();
            container_dimensions.height = container.client_height();
        }
        Callback::from(move |e: MouseEvent| {
            if container_dimensions.width > 0 && container_dimensions.height > 0 {
                let x = (e.page_x() - container_dimensions.offset_left - NODE_WIDTH / 2)
                    .clamp(0, container_dimensions.width - NODE_WIDTH)
                    as u64;
                let y = (e.page_y() - container_dimensions.offset_top - NODE_HEIGHT / 2)
                    .clamp(0, container_dimensions.height - NODE_HEIGHT)
                    as u64;
                nodes_store.dispatch(NodesAction::MoveActive(MoveActiveCmd { x, y }))
            }
        })
    };

    let on_node_mouse_down = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| dispatcher.dispatch(NodesAction::Activate(node.id)))
    });
    let on_node_mouse_up = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| dispatcher.dispatch(NodesAction::Deactivate(node.id)))
    });
    let on_node_click = use_ref(|| {
        let dispatcher = dispatcher.clone();
        Callback::from(move |node: Node| {
            // if node.is_active {
            //     nodes_store.dispatch(NodesAction::Deactivate(node.id));
            // } else {
            //     nodes_store.dispatch(NodesAction::Activate(node.id));
            // }
        })
    });

    let render_nodes = {
        nodes_store
            .nodes
            .iter()
            .map(|node| {
                html! {
                    <RenderSingleNode
                        node={node.clone()}
                        on_mouse_down={on_node_mouse_down.clone()}
                        on_mouse_up={on_node_mouse_up.clone()}
                        on_click={on_node_click.clone()}
                    />
                }
            })
            .collect::<Html>()
    };

    html! {
        <div
            class={classes!(
                "flex",
                "flex-col",
                "min-h-0",
                "p-4")}
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
            >
                {render_nodes}
            </div>
        </div>
    }
}
