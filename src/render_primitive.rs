use crate::{
    layout::Rect,
    styles::{Corner, Edge, Style, RenderCommand},
};
use bevy::prelude::{Handle, Image, Color};
use kayak_font::{TextLayout, TextProperties};

#[derive(Debug, Clone, PartialEq)]
pub enum RenderPrimitive {
    Empty,
    Clip {
        layout: Rect,
    },
    Quad {
        layout: Rect,
        background_color: Color,
        border_color: Color,
        border: Edge<f32>,
        border_radius: Corner<f32>,
    },
    Text {
        color: Color,
        content: String,
        font: String,
        text_layout: TextLayout,
        layout: Rect,
        properties: TextProperties,
    },
    Image {
        border_radius: Corner<f32>,
        layout: Rect,
        handle: Handle<Image>,
    },
    TextureAtlas {
        size: (f32, f32),
        position: (f32, f32),
        layout: Rect,
        handle: Handle<Image>,
    },
    NinePatch {
        border: Edge<f32>,
        layout: Rect,
        handle: Handle<Image>,
    },
}

impl RenderPrimitive {
    pub fn set_layout(&mut self, new_layout: Rect) {
        match self {
            RenderPrimitive::Clip { layout, .. } => *layout = new_layout,
            RenderPrimitive::Quad { layout, .. } => *layout = new_layout,
            RenderPrimitive::Text { layout, .. } => *layout = new_layout,
            RenderPrimitive::Image { layout, .. } => *layout = new_layout,
            RenderPrimitive::NinePatch { layout, .. } => *layout = new_layout,
            RenderPrimitive::TextureAtlas { layout, .. } => *layout = new_layout,
            _ => (),
        }
    }
}

impl From<&Style> for RenderPrimitive {
    fn from(style: &Style) -> Self {
        let render_command = style.render_command.resolve();

        let background_color = style.background_color.resolve_or(Color::rgba(1.0, 1.0, 1.0, 0.0));

        let border_color = style.border_color.resolve_or(Color::rgba(1.0, 1.0, 1.0, 0.0));

        let font = style
            .font
            .resolve_or_else(|| String::from(crate::DEFAULT_FONT));

        let font_size = style.font_size.resolve_or(14.0);

        let line_height = style.line_height.resolve_or(font_size * 1.2);

        match render_command {
            RenderCommand::Empty => Self::Empty,
            RenderCommand::Layout => Self::Empty,
            RenderCommand::Clip => Self::Clip {
                layout: Rect::default(),
            },
            RenderCommand::Quad => Self::Quad {
                background_color,
                border_color,
                border_radius: style.border_radius.resolve(),
                border: style.border.resolve(),
                layout: Rect::default(),
            },
            RenderCommand::Text { content } => Self::Text {
                color: style.color.resolve(),
                content,
                font,
                text_layout: TextLayout::default(),
                layout: Rect::default(),
                properties: TextProperties {
                    font_size,
                    line_height,
                    ..Default::default()
                },
            },
            RenderCommand::Image { handle } => Self::Image {
                border_radius: style.border_radius.resolve(),
                layout: Rect::default(),
                handle,
            },
            RenderCommand::TextureAtlas {
                handle,
                size,
                position,
            } => Self::TextureAtlas {
                handle,
                layout: Rect::default(),
                size,
                position,
            },
            RenderCommand::NinePatch { handle, border } => Self::NinePatch {
                border,
                layout: Rect::default(),
                handle,
            },
        }
    }
}
