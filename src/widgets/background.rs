use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, In, Query, With};

use crate::{
    children::Children, context::WidgetName, prelude::WidgetContext, styles::{Style, RenderCommand, StyleProp}, widget::Widget, on_event::OnEvent,
};

#[derive(Component, Default)]
pub struct Background;

impl Widget for Background {}

#[derive(Bundle)]
pub struct BackgroundBundle {
    pub background: Background,
    pub styles: Style,
    pub children: Children,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for BackgroundBundle {
    fn default() -> Self {
        Self {
            background: Default::default(),
            styles: Default::default(),
            children: Default::default(),
            on_event: Default::default(),
            widget_name: WidgetName(Background::default().get_name()),
        }
    }
}

pub fn update_background(
    In((mut widget_context, entity)): In<(WidgetContext, Entity)>,
    mut commands: Commands,
    mut query: Query<(&mut Style, &Children), (Changed<Style>, With<Background>)>,
) -> bool {
    if let Ok((mut style, children)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Quad);
        children.spawn(Some(entity), &mut widget_context, &mut commands);
        return true;
    }
    false
}
