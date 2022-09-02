use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, In, Query, With};

use crate::{
    children::Children, context::WidgetName, prelude::WidgetTree, styles::Style, widget::Widget,
};

#[derive(Component, Default)]
pub struct Background;

impl Widget for Background {}

#[derive(Bundle)]
pub struct BackgroundBundle {
    pub background: Background,
    pub styles: Style,
    pub children: Children,
    pub widget_name: WidgetName,
}

impl Default for BackgroundBundle {
    fn default() -> Self {
        Self {
            background: Default::default(),
            styles: Default::default(),
            children: Default::default(),
            widget_name: WidgetName(Background::default().get_name()),
        }
    }
}

pub fn update_background(
    In((mut widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    mut query: Query<(&mut Style, &Children), (Changed<Style>, With<Background>)>,
) -> bool {
    if let Ok((_, children)) = query.get_mut(entity) {
        children.spawn(Some(entity), &mut widget_tree, &mut commands, true);
        return true;
    }
    false
}
