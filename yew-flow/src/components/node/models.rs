use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::types::{standard_id::StandardId, standard_unit::StandardUnit};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeInput {
    pub id: StandardId,
    #[serde(skip)]
    pub reference: NodeRef,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeOutput {
    pub id: StandardId,
    #[serde(skip)]
    pub reference: NodeRef,
}

#[derive(Clone, PartialEq, Properties, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: StandardId,
    pub title: String,
    pub x: StandardUnit,
    pub y: StandardUnit,
    pub color: String,
    pub inputs: Vec<NodeInput>,
    pub outputs: Vec<NodeOutput>,
}
