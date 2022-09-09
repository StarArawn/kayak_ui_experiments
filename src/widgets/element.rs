use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, In, Query, With};

use crate::{
    children::Children, context::WidgetName, prelude::WidgetContext, styles::{Style, StyleProp, RenderCommand}, widget::Widget,
};

#[derive(Component, Default)]
pub struct Element;

impl Widget for Element {}

#[derive(Bundle)]
pub struct ElementBundle {
    pub element: Element,
    pub styles: Style,
    pub children: Children,
    pub widget_name: WidgetName,
}

impl Default for ElementBundle {
    fn default() -> Self {
        Self {
            element: Default::default(),
            styles: Default::default(),
            children: Default::default(),
            widget_name: WidgetName(Element::default().get_name()),
        }
    }
}

pub fn update_element(
    In((mut widget_context, entity)): In<(WidgetContext, Entity)>,
    mut commands: Commands,
    mut query: Query<(&mut Style, &Children), (Changed<Style>, With<Element>)>,
) -> bool {
    if let Ok((mut style, children)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Layout);
        children.spawn(Some(entity), &mut widget_context, &mut commands);
        return true;
    }
    false
}