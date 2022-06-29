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

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodesProps {}

#[styled_component(RenderNodes)]
pub fn render_nodes(RenderNodesProps {}: &RenderNodesProps) -> Html {
    let step = use_state(|| 0);
    let nodes = use_state(|| {
        vec![
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
        ]
    });

    let on_step_btn_click = {
        let step = step.clone();
        // Callback::from(move |_| step.set(*step + 1))
    };
    let on_container_mouse_move = {
        let nodes = nodes.clone();
        Callback::from(move |e: MouseEvent| {
            let updated = nodes
                .iter()
                .map(|node| {
                    let mut tmp = node.clone();
                    if tmp.is_active {
                        tmp.x = (e.offset_x().clamp(40, 600) - 40 )as u64;
                        tmp.y = (e.offset_y().clamp(25, 400) - 25 )as u64;
                    }
                    tmp
                })
                .collect();
            nodes.set(updated);
        })
    };

    {
        let nodes = nodes.clone();
        let step = step.clone();
        let step_deps = step.clone();
        use_effect_with_deps(
            move |_| {
                let updated = nodes
                    .iter()
                    .map(|node| {
                        let mut tmp = node.clone();
                        tmp.x += *step;
                        tmp
                    })
                    .collect();
                nodes.set(updated);
                || ()
            },
            step_deps,
        );
    }

    let render_nodes = nodes
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
                let nodes = nodes.clone();
                Callback::from(move |_| {
                    let updated = nodes
                        .iter()
                    .map(|node_item| {
                            let mut tmp = node_item.clone();
                            if tmp.id == node.id {
                                tmp.is_active = !tmp.is_active;
                            }
                            tmp
                        })
                        .collect();
                    nodes.set(updated);
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
