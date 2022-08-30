#![allow(dead_code)]

mod tree;
mod context;
mod widget;
mod styles;
mod layout;
mod node;
mod render_primitive;
mod calculate_nodes;
mod camera;
pub(crate) mod render;
mod window_size;
mod widgets;
mod children;
// mod ui_system;

pub use window_size::WindowSize;

pub use camera::*;

/// The default font name used by Kayak
pub const DEFAULT_FONT: &str = "Kayak-Default";

pub mod prelude {
    pub use crate::children::Children;
    pub use crate::widgets::*;
    pub use crate::render::font::FontMapping;
    pub use crate::camera::UICameraBundle;
    pub use crate::tree::*;
    pub use crate::context::*;
    pub use crate::widget::*;
    pub use crate::styles::*;
    pub use crate::node::DirtyNode;
}
