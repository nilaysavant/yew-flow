use yew::prelude::*;

use crate::components::node::render_node_list::RenderNodeList;

#[derive(Debug, Properties, PartialEq)]
pub struct WorkspaceProps;

/// # Yew Flow Workspace
/// 
/// `yew-flow` canvas/work area where nodes
/// are rendered.
#[function_component(Workspace)]
pub fn workspace(WorkspaceProps {}: &WorkspaceProps) -> Html {
    html! {
        <>
            <RenderNodeList />
        </>
    }
}
