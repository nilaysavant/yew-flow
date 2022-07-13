use yew::prelude::*;

use crate::store::{
    Connector, InteractionMode, NewEdgeDragActivateCmd, NewEdgeDragDeactivateCmd, WorkspaceAction,
    WorkspaceStore,
};

use super::{
    models::{Node, NodeInput, NodeOutput},
    render_node::RenderNode,
};

#[derive(Clone, Properties, PartialEq)]
pub struct RenderNodeListProps {
    pub nodes: Vec<Node>,
    pub interaction_mode: InteractionMode,
    pub dispatcher: UseReducerDispatcher<WorkspaceStore>,
}

#[function_component(RenderNodeList)]
pub fn render_node_list(
    RenderNodeListProps {
        nodes,
        interaction_mode,
        dispatcher,
    }: &RenderNodeListProps,
) -> Html {
    let on_node_mouse_down = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |node: Node, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NodeDragActivate(node.id))
            },
            dispatcher,
        )
    };
    let on_node_mouse_up = {
        let dispatcher = dispatcher.clone();
        let interaction_mode = interaction_mode.clone();
        use_callback(
            move |node: Node, (dispatcher, interaction_mode)| {
                match interaction_mode {
                    InteractionMode::None => {
                        // dispatch(WorkspaceAction::DragNode(DragNodeCmd { x, y }))
                    }
                    InteractionMode::NodeDrag(_) => {
                        dispatcher.dispatch(WorkspaceAction::NodeDragDeactivate)
                    }
                    InteractionMode::NewEdgeDrag(_) => dispatcher.dispatch(
                        WorkspaceAction::NewEdgeDragDeactivate(NewEdgeDragDeactivateCmd {
                            to_reference: None,
                            to_connector: None,
                        }),
                    ),
                }
            },
            (dispatcher, interaction_mode),
        )
    };
    let on_node_click = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |node: Node, _| {
                // if node.is_active {
                //     store.dispatch(NodesAction::Deactivate(node.id));
                // } else {
                //     store.dispatch(NodesAction::Activate(node.id));
                // }
            },
            (),
        )
    };
    let on_node_input_mouse_down = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |input: NodeInput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragActivate(
                    NewEdgeDragActivateCmd {
                        from_reference: input.reference,
                        from_connector: Connector::Input(input.id),
                    },
                ))
            },
            dispatcher,
        )
    };
    let on_node_input_mouse_up = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |input: NodeInput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragDeactivate(
                    NewEdgeDragDeactivateCmd {
                        to_reference: Some(input.reference),
                        to_connector: Some(Connector::Input(input.id)),
                    },
                ))
            },
            dispatcher,
        )
    };
    let on_node_output_mouse_down = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |output: NodeOutput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragActivate(
                    NewEdgeDragActivateCmd {
                        from_reference: output.reference,
                        from_connector: Connector::Output(output.id),
                    },
                ))
            },
            dispatcher,
        )
    };
    let on_node_output_mouse_up = {
        let dispatcher = dispatcher.clone();
        use_callback(
            move |output: NodeOutput, dispatcher| {
                dispatcher.dispatch(WorkspaceAction::NewEdgeDragDeactivate(
                    NewEdgeDragDeactivateCmd {
                        to_reference: Some(output.reference),
                        to_connector: Some(Connector::Output(output.id)),
                    },
                ))
            },
            dispatcher,
        )
    };

    let render_nodes = {
        nodes
            .iter()
            .map(|node| {
                html! {
                    <RenderNode
                        node={node.clone()}
                        on_mouse_down={on_node_mouse_down.clone()}
                        on_mouse_up={on_node_mouse_up.clone()}
                        on_click={on_node_click.clone()}
                        on_input_mouse_down={on_node_input_mouse_down.clone()}
                        on_input_mouse_up={on_node_input_mouse_up.clone()}
                        on_output_mouse_down={on_node_output_mouse_down.clone()}
                        on_output_mouse_up={on_node_output_mouse_up.clone()}
                    />
                }
            })
            .collect::<Html>()
    };

    html! {
        <>
            {render_nodes}
        </>
    }
}
