use bevy::{prelude::{Component, Bundle, In, Entity, Query, Changed, Color, Commands}};
use morphorm::Units;

use crate::{styles::{Style, StyleProp, RenderCommand, Corner}, on_event::OnEvent, widget::Widget, prelude::{WidgetTree, Children}};

#[derive(Component, Default)]
pub struct Button;

#[derive(Default, Bundle)]
pub struct ButtonBundle {
    pub button: Button,
    pub styles: Style,
    pub on_event: OnEvent,
    pub children: Children,
}

impl Widget for Button {}

pub fn button_update(
    In((mut widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    mut query: Query<(&mut Style, &Children), Changed<Button>>,
) -> bool {

    if let Ok((mut style, children)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Quad);
        style.background_color = StyleProp::Value(Color::rgba(0.0781, 0.0898, 0.101, 1.0));
        style.border_radius = StyleProp::Value(Corner::all(5.0));
        style.height = StyleProp::Value(Units::Pixels(45.0));
        style.padding_left = StyleProp::Value(Units::Stretch(1.0));
        style.padding_right = StyleProp::Value(Units::Stretch(1.0));
        // style.cursor = CursorIcon::Hand.into();/

        children.build(Some(entity), &mut widget_tree, &mut commands);

        return true;
    }

    false
}