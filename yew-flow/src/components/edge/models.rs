use colorsys::Hsl;
use yew::prelude::*;

use crate::types::{
    standard_id::{IdentifierExt, StandardId},
    standard_unit::StandardUnit,
};

/// Edge
#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Edge {
    pub id: StandardId,
    /// From input id.
    pub from_input: Option<String>,
    /// To output id.
    pub to_output: Option<String>,
    /// From (x1, y1) coordinate
    pub x1: StandardUnit,
    /// From (x1, y1) coordinate
    pub y1: StandardUnit,
    /// To (x2, y2) coordinate
    pub x2: StandardUnit,
    /// To (x2, y2) coordinate
    pub y2: StandardUnit,
    pub color: Hsl,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            id: StandardId::generate(),
            from_input: None,
            to_output: None,
            x1: Default::default(),
            y1: Default::default(),
            x2: Default::default(),
            y2: Default::default(),
            color: Hsl::new(0., 100., 100., Some(0.8)),
        }
    }
}
