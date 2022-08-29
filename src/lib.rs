mod tree;
mod context;
mod widget;
mod styles;
mod layout;
mod node;
mod render_primitive;
mod calculate_nodes;

/// The default font name used by Kayak
pub const DEFAULT_FONT: &str = "Kayak-Default";

pub mod prelude {
    pub use crate::tree::*;
    pub use crate::context::*;
    pub use crate::widget::*;
    pub use crate::styles::*;
}