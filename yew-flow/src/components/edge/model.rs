use colorsys::Hsl;
use yew::prelude::*;

/// Edge
#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Edge {
    pub id: usize,
    /// From (x1, y1) coordinate
    pub x1: i32,
    /// From (x1, y1) coordinate
    pub y1: i32,
    /// To (x2, y2) coordinate
    pub x2: i32,
    /// To (x2, y2) coordinate
    pub y2: i32,
    pub color: Hsl,
    pub is_active: bool,
}
