use bevy::prelude::{Bundle, Component, In, Entity, Query, Changed, Image, Handle, Commands};

use crate::{
    context::WidgetName,
    styles::{Edge, Style, StyleProp, RenderCommand},
    widget::Widget, prelude::WidgetContext, children::Children,
};

#[derive(Component, Default, Debug)]
pub struct NinePatch {
    /// The handle to image
    pub handle: Handle<Image>,
    /// The size of each edge (in pixels)
    pub border: Edge<f32>,
}

impl Widget for NinePatch {}

#[derive(Bundle)]
pub struct NinePatchBundle {
    pub nine_patch: NinePatch,
    pub styles: Style,
    pub children: Children,
    pub widget_name: WidgetName,
}

impl Default for NinePatchBundle {
    fn default() -> Self {
        Self {
            nine_patch: Default::default(),
            styles: Default::default(),
            children: Children::default(),
            widget_name: WidgetName(NinePatch::default().get_name()),
        }
    }
}

pub fn update_nine_patch(
    In((widget_context, entity)): In<(WidgetContext, Entity)>,
    mut commands: Commands,
    mut query: Query<(&mut Style, &NinePatch, &Children), (Changed<NinePatch>, Changed<Style>)>,
) -> bool {
    if let Ok((mut style, nine_patch, children)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::NinePatch {
            border: nine_patch.border,
            handle: nine_patch.handle.clone_weak(),
        });

        children.spawn(Some(entity), &widget_context, &mut commands);

        return true;
    }
    false
}
