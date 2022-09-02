use bevy::prelude::{Handle, In, Entity, Query, Changed, Bundle, Component};

use crate::{widget::Widget, prelude::WidgetTree, styles::{Style, RenderCommand, StyleProp}, context::WidgetName};

#[derive(Component, Default)]
pub struct Image(pub Handle<bevy::prelude::Image>);

impl Widget for Image {}

#[derive(Bundle)]
pub struct ImageBundle {
    pub image: Image,
    pub style: Style,
    pub widget_name: WidgetName,
}

impl Default for ImageBundle {
    fn default() -> Self {
        Self { image: Default::default(), style: Default::default(), widget_name: WidgetName(Image::default().get_name()) }
    }
}

pub fn update_image(
    In((_widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut query: Query<(&mut Style, &Image), (Changed<Image>, Changed<Style>)>,
) -> bool {
    if let Ok((mut style, image)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Image { handle: image.0.clone_weak() });
        return true;
    }
    false
}
