use yew::prelude::*;

use crate::components::edge::render_edge::RenderEdge;

use super::models::Edge;

#[derive(Clone, Properties, PartialEq)]
pub struct RenderEdgeListProps {
    pub edges: Vec<Edge>,
}

#[function_component(RenderEdgeList)]
pub fn render_edge_list(RenderEdgeListProps { edges }: &RenderEdgeListProps) -> Html {
    let render_edges = {
        edges
            .clone()
            .iter()
            .map(|edge| {
                html! {
                    <RenderEdge
                        edge={edge.clone()}
                    />
                }
            })
            .collect::<Html>()
    };

    html! {
        <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
            // <path
            //     d="M 100 0 C 200 0, 0 100, 100 100"
            //     stroke="blue"
            //     stroke-width="3px"
            //     fill="transparent"
            // />
            {render_edges}
        </svg>
    }
}
