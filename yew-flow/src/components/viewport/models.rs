use web_sys::HtmlElement;
use yew::prelude::*;

use crate::types::standard_unit::StandardUnit;

/// Used to store container dimensions like
/// **offsets**, **width**, **height** etc
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerDimensions {
    pub offset_left: StandardUnit,
    pub offset_top: StandardUnit,
    pub width: StandardUnit,
    pub height: StandardUnit,
}

impl Default for ContainerDimensions {
    fn default() -> Self {
        Self {
            offset_left: Default::default(),
            offset_top: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
    }
}

/// # Yew Flow Viewport Model
///
/// Defines `yew-flow` viewport configuration
#[derive(Debug, Clone, PartialEq)]
pub struct Viewport {
    /// Viewport element reference
    pub reference: NodeRef,
    pub dimensions: ContainerDimensions,
}

impl Viewport {
    /// Create a new Viewport from reference
    pub fn new(reference: NodeRef) -> Self {
        let mut dimensions = ContainerDimensions::default();
        if let Some(container) = reference.cast::<HtmlElement>() {
            // set proper container offset values
            dimensions.offset_left = container.offset_left().into();
            dimensions.offset_top = container.offset_top().into();
            dimensions.width = container.client_width().into();
            dimensions.height = container.client_height().into();
        }
        Self {
            reference,
            dimensions,
        }
    }

    /// Get x position relative to viewport of any element.
    pub fn relative_x_pos_from_abs(
        self: &Self,
        abs_x: StandardUnit,
        element_width: Option<StandardUnit>,
    ) -> StandardUnit {
        let element_width = element_width.unwrap_or_default();
        let x = (abs_x - self.dimensions.offset_left - element_width / 2.)
            .clamp(0., self.dimensions.width - element_width);
        x
    }

    /// Get y position relative to viewport of any element.
    pub fn relative_y_pos_from_abs(
        self: &Self,
        abs_y: StandardUnit,
        element_height: Option<StandardUnit>,
    ) -> StandardUnit {
        let element_height = element_height.unwrap_or_default();
        let y = (abs_y - self.dimensions.offset_top - element_height / 2.)
            .clamp(0., self.dimensions.height - element_height);
        y
    }
}
