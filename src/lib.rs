#![allow(dead_code)]

mod calculate_nodes;
mod camera;
mod children;
mod context;
mod context_entities;
mod cursor;
mod event;
mod event_dispatcher;
mod focus_tree;
mod input;
mod input_event;
mod keyboard_event;
mod layout;
mod layout_dispatcher;
mod node;
mod on_event;
mod on_layout;
pub(crate) mod render;
mod render_primitive;
mod styles;
mod tree;
mod widget;
mod widget_context;
mod widgets;
mod window_size;
mod on_change;

pub use window_size::WindowSize;

pub use camera::*;

/// The default font name used by Kayak
pub const DEFAULT_FONT: &str = "Kayak-Default";

pub mod prelude {
    pub use crate::camera::UICameraBundle;
    pub use crate::children::Children;
    pub use crate::context::*;
    pub use crate::render::font::FontMapping;
    pub use crate::tree::*;
    pub mod widgets {
        pub use crate::widgets::*;
    }
    pub use crate::event::*;
    pub use crate::focus_tree::Focusable;
    pub use crate::event_dispatcher::EventDispatcherContext;
    pub use crate::input_event::*;
    pub use crate::keyboard_event::*;
    pub use crate::layout::*;
    pub use crate::node::DirtyNode;
    pub use crate::on_event::OnEvent;
    pub use crate::on_layout::OnLayout;
    pub use crate::on_change::OnChange;
    pub use crate::styles::*;
    pub use crate::widget::*;
    pub use crate::widget_context::*;
    pub use kayak_font::Alignment;
    pub use kayak_ui_macros::{constructor, rsx};
}

pub use focus_tree::Focusable;