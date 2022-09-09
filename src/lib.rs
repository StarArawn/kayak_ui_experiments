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
mod event;
mod cursor;
mod keyboard_event;
mod on_event;
mod event_dispatcher;
mod input_event;
mod focus_tree;
mod input;
mod context_entities;
mod widget_context;

use bevy::prelude::Component;
pub use window_size::WindowSize;

pub use camera::*;

/// The default font name used by Kayak
pub const DEFAULT_FONT: &str = "Kayak-Default";

pub mod prelude {
    pub use crate::children::Children;
    pub use crate::render::font::FontMapping;
    pub use crate::camera::UICameraBundle;
    pub use crate::tree::*;
    pub use crate::context::*;
    pub mod widgets {
        pub use crate::widgets::*;
    }
    pub use crate::widget::*;
    pub use crate::styles::*;
    pub use crate::node::DirtyNode;
    pub use crate::on_event::OnEvent;
    pub use crate::input_event::*;
    pub use crate::keyboard_event::*;
    pub use crate::event::*;
    pub use crate::event_dispatcher::EventDispatcherContext;
    pub use kayak_font::Alignment;
    pub use crate::widget_context::*;
}

#[derive(Component)]
pub struct Focusable;