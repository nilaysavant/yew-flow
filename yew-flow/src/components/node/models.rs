use colorsys::Hsl;
use yew::prelude::*;

use crate::types::{standard_id::StandardId, standard_unit::StandardUnit};

#[derive(Clone, PartialEq, Debug)]
pub struct NodeInput {
    pub id: StandardId,
    pub reference: NodeRef,
}

#[derive(Clone, PartialEq, Debug)]
pub struct NodeOutput {
    pub id: StandardId,
    pub reference: NodeRef,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Node {
    pub id: StandardId,
    pub title: String,
    pub x: StandardUnit,
    pub y: StandardUnit,
    pub color: Hsl,
    pub inputs: Vec<NodeInput>,
    pub outputs: Vec<NodeOutput>,
}
