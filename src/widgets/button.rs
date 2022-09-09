use bevy::prelude::{Bundle, Changed, Color, Commands, Component, Entity, In, Query};

use crate::{
    context::WidgetName,
    on_event::OnEvent,
    prelude::{Children, Units, WidgetContext},
    styles::{Corner, RenderCommand, Style, StyleProp},
    widget::Widget,
};

#[derive(Component, Default)]
pub struct Button;

#[derive(Bundle)]
pub struct ButtonBundle {
    pub button: Button,
    pub styles: Style,
    pub on_event: OnEvent,
    pub children: Children,
    pub widget_name: WidgetName,
}

impl Default for ButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            styles: Default::default(),
            on_event: Default::default(),
            children: Children::default(),
            widget_name: WidgetName(Button::default().get_name()),
        }
    }
}

impl Widget for Button {}

pub fn button_update(
    In((mut widget_context, entity)): In<(WidgetContext, Entity)>,
    mut commands: Commands,
    mut query: Query<(&mut Style, &Children), Changed<Button>>,
) -> bool {
    if let Ok((mut style, children)) = query.get_mut(entity) {
        *style = Style::default()
            .with_style(Style {
                render_command: StyleProp::Value(RenderCommand::Quad),
                ..Default::default()
            })
            .with_style(style.clone())
            .with_style(Style {
            render_command: StyleProp::Value(RenderCommand::Quad),
                background_color: StyleProp::Value(Color::rgba(0.0781, 0.0898, 0.101, 1.0)),
                border_radius: StyleProp::Value(Corner::all(5.0)),
                height: StyleProp::Value(Units::Pixels(45.0)),
                padding_left: StyleProp::Value(Units::Stretch(1.0)),
                padding_right: StyleProp::Value(Units::Stretch(1.0)),
                padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
                padding_top: StyleProp::Value(Units::Stretch(1.0)),
                ..Default::default()
            });
        // style.cursor = CursorIcon::Hand.into();/

        children.spawn(Some(entity), &mut widget_context, &mut commands);

        return true;
    }

    false
}
