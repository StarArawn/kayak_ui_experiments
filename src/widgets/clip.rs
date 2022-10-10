use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, In, Query, With};

use crate::{
    children::Children,
    context::WidgetName,
    prelude::WidgetContext,
    styles::{RenderCommand, Style, StyleProp, Units},
    widget::Widget,
};

#[derive(Component, Default)]
pub struct Clip;

impl Widget for Clip {}

#[derive(Bundle)]
pub struct ClipBundle {
    pub clip: Clip,
    pub styles: Style,
    pub children: Children,
    pub widget_name: WidgetName,
}

impl Default for ClipBundle {
    fn default() -> Self {
        Self {
            clip: Clip::default(),
            styles: Style {
                render_command: StyleProp::Value(RenderCommand::Clip),
                height: StyleProp::Value(Units::Stretch(1.0)),
                width: StyleProp::Value(Units::Stretch(1.0)),
                ..Style::default()
            },
            children: Children::default(),
            widget_name: WidgetName(Clip::default().get_name()),
        }
    }
}

pub fn update_clip(
    In((widget_context, entity)): In<(WidgetContext, Entity)>,
    _: Commands,
    mut query: Query<(&Style, &Children), (Changed<Style>, With<Clip>)>,
) -> bool {
    if let Ok((_, children)) = query.get_mut(entity) {
        children.process(&widget_context, Some(entity));
        return true;
    }
    false
}
