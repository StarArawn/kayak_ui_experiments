use bevy::prelude::{Handle, Image};

use super::Edge;

#[derive(Debug, Clone, PartialEq)]
pub enum RenderCommand {
    Empty,
    /// Represents a node that has no renderable object but contributes to the layout.
    Layout,
    Clip,
    Quad,
    Text {
        content: String,
    },
    Image {
        handle: Handle<Image>,
    },
    TextureAtlas {
        position: (f32, f32),
        size: (f32, f32),
        handle: Handle<Image>,
    },
    NinePatch {
        border: Edge<f32>,
        handle: Handle<Image>,
    },
}

impl Default for RenderCommand {
    fn default() -> Self {
        Self::Empty
    }
}
