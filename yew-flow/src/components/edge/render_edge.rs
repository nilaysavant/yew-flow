use yew::prelude::*;

use crate::constants::{NODE_HEIGHT, NODE_WIDTH};

use super::models::Edge;

#[derive(Clone, Properties, PartialEq)]
pub struct RenderEdgeProps {
    pub edge: Edge,
}

#[function_component(RenderEdge)]
pub fn render_edge(RenderEdgeProps { edge }: &RenderEdgeProps) -> Html {
    let Edge {
        id,
        from_output,
        to_input,
        x1,
        y1,
        x2,
        y2,
        color,
    } = edge;

    let sx1 = x1 + (NODE_WIDTH / 2.);
    let sy1 = y1;
    let sx2 = (x2 - (NODE_WIDTH / 2.)).max(0.);
    let sy2 = y2;

    html! {
        <path
            d={format!(
                "M {x1} {y1} C {sx1} {sy1}, {sx2} {sy2}, {x2} {y2}",
                x1=x1,
                y1=y1,
                sx1=sx1,
                sy1=sy1,
                sx2=sx2,
                sy2=sy2,
                x2=x2,
                y2=y2,
            )}
            stroke="blue"
            stroke-width="4px"
            fill="transparent"
        />
    }
}
