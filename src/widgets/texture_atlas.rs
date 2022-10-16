use bevy::prelude::{Bundle, Changed, Component, Entity, Handle, Image, In, Or, Query, Vec2, With};

use crate::{
    context::{Mounted, WidgetName},
    prelude::WidgetContext,
    styles::{RenderCommand, Style, StyleProp},
    widget::Widget,
};

/// A widget that renders a texture atlas
/// Allows for the use of a partial square of an image such as in a sprite sheet
///
/// # Props
///
/// __Type:__ [`TextureAtlasProps`]
///
/// | Common Prop | Accepted |
/// | :---------: | :------: |
/// | `children`  |           |
/// | `styles`    | ✅        |
/// | `on_event`  | ✅        |
/// | `on_layout` | ✅        |
/// | `focusable` | ✅        |
///
#[derive(Component, Default, Debug)]
pub struct TextureAtlas {
    /// The handle to image
    pub handle: Handle<Image>,
    /// The position of the tile (in pixels)
    pub position: Vec2,
    /// The size of the tile (in pixels)
    pub tile_size: Vec2,
}

impl Widget for TextureAtlas {}

#[derive(Bundle)]
pub struct TextureAtlasBundle {
    pub atlas: TextureAtlas,
    pub styles: Style,
    pub widget_name: WidgetName,
}

impl Default for TextureAtlasBundle {
    fn default() -> Self {
        Self {
            atlas: Default::default(),
            styles: Default::default(),
            widget_name: WidgetName(TextureAtlas::default().get_name()),
        }
    }
}

pub fn update_texture_atlas(
    In((_widget_context, entity)): In<(WidgetContext, Entity)>,
    mut query: Query<
        (&mut Style, &TextureAtlas),
        Or<(Changed<TextureAtlas>, Changed<Style>, With<Mounted>)>,
    >,
) -> bool {
    if let Ok((mut styles, texture_atlas)) = query.get_mut(entity) {
        *styles = Style {
            render_command: StyleProp::Value(RenderCommand::TextureAtlas {
                position: texture_atlas.position,
                size: texture_atlas.tile_size,
                handle: texture_atlas.handle.clone_weak(),
            }),
            ..styles.clone()
        };

        return true;
    }

    false
}
