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

/// Edge
#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Edge {
    pub id: usize,
    /// From (x1, y1) coordinate
    pub x1: u64,
    /// From (x1, y1) coordinate
    pub y1: u64,
    /// To (x2, y2) coordinate
    pub x2: u64,
    /// To (x2, y2) coordinate
    pub y2: u64,
    pub color: Hsl,
    pub is_active: bool,
}