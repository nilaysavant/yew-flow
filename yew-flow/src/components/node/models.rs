use colorsys::Hsl;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct NodeInput {
    pub id: String,
    pub reference: NodeRef,
}

#[derive(Clone, PartialEq, Debug)]
pub struct NodeOutput {
    pub id: String,
    pub reference: NodeRef,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Node {
    pub id: usize,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub color: Hsl,
    pub inputs: Vec<NodeInput>,
    pub outputs: Vec<NodeOutput>,
}
