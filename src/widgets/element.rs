use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, In, Or, Query, With};

use crate::{
    children::Children,
    context::{Mounted, WidgetName},
    on_event::OnEvent,
    prelude::WidgetContext,
    styles::{RenderCommand, Style, StyleProp},
    widget::Widget,
};

#[derive(Component, Default)]
pub struct Element;

impl Widget for Element {}

#[derive(Bundle)]
pub struct ElementBundle {
    pub element: Element,
    pub styles: Style,
    pub on_event: OnEvent,
    pub children: Children,
    pub widget_name: WidgetName,
}

impl Default for ElementBundle {
    fn default() -> Self {
        Self {
            element: Default::default(),
            styles: Default::default(),
            children: Default::default(),
            on_event: OnEvent::default(),
            widget_name: WidgetName(Element::default().get_name()),
        }
    }
}

pub fn update_element(
    In((mut widget_context, entity)): In<(WidgetContext, Entity)>,
    _: Commands,
    mut query: Query<(&mut Style, &Children), Or<((Changed<Style>, With<Element>), With<Mounted>)>>,
) -> bool {
    if let Ok((mut style, children)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Layout);
        children.process(&mut widget_context, Some(entity));
        return true;
    }
    false
}
