use colorsys::Hsl;
use yew::prelude::*;

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
