use web_sys::HtmlElement;
use yew::prelude::*;

/// Used to store container dimensions like
/// **offsets**, **width**, **height** etc
#[derive(Debug, Clone, Copy)]
pub struct ContainerDimensions {
    pub offset_left: i32,
    pub offset_top: i32,
    pub width: i32,
    pub height: i32,
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
#[derive(Debug, Clone)]
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
            dimensions.offset_left = container.offset_left();
            dimensions.offset_top = container.offset_top();
            dimensions.width = container.client_width();
            dimensions.height = container.client_height();
        }
        Self {
            reference,
            dimensions,
        }
    }

    /// Get x position relative to viewport of any element.
    pub fn relative_x_pos_from_abs(self: &Self, abs_x: i32, element_width: Option<i32>) -> i32 {
        let element_width = element_width.unwrap_or_default();
        let x = (abs_x - self.dimensions.offset_left - element_width / 2)
            .clamp(0, self.dimensions.width - element_width);
        x
    }

    /// Get y position relative to viewport of any element.
    pub fn relative_y_pos_from_abs(self: &Self, abs_y: i32, element_height: Option<i32>) -> i32 {
        let element_height = element_height.unwrap_or_default();
        let y = (abs_y - self.dimensions.offset_top - element_height / 2)
            .clamp(0, self.dimensions.height - element_height);
        y
    }
}
