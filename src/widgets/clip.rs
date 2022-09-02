use bevy::{prelude::{Bundle, Changed, Component, Entity, In, Query, With, Commands}};

use crate::{prelude::WidgetTree, styles::{Style, RenderCommand, StyleProp, Units}, widget::Widget, children::Children, context::WidgetName};

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
    In((mut widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    mut query: Query<(&Style, &Children), (Changed<Style>, With<Clip>)>,
) -> bool {
    if let Ok((_, children)) = query.get_mut(entity) {
        children.spawn(Some(entity), &mut widget_tree, &mut commands);
        return true;
    }
    false
}
