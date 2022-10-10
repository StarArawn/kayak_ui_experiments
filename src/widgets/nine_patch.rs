use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, Handle, Image, In, Query};

use crate::{
    children::Children,
    context::WidgetName,
    prelude::WidgetContext,
    styles::{Edge, RenderCommand, Style, StyleProp},
    widget::Widget,
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

        children.process(&widget_context, Some(entity));

        return true;
    }
    false
}
