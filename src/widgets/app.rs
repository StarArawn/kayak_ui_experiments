use bevy::{prelude::{Component, Commands, In, Entity, Res, Query, With}, window::Windows};
use morphorm::Units;

use crate::{styles::{StyleProp, Style}, widget::Widget, children::Children, prelude::WidgetTree};

#[derive(Component, Default)]
pub struct KayakApp {
    pub children: Children,
}

impl Widget for KayakApp {}

/// TODO: USE CAMERA INSTEAD OF WINDOW!!
pub fn app_update(
    In((mut widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<(&mut Style, &KayakApp)>,
) -> bool {
    let mut has_changed = false;
    let primary_window = windows.get_primary().unwrap();
    if let Ok((mut app_style, app)) = query.get_mut(entity) {
        if app_style.width != StyleProp::Value(Units::Pixels(primary_window.width())) {
            app_style.width = StyleProp::Value(Units::Pixels(primary_window.width()));
            has_changed = true;
        }
        if app_style.height != StyleProp::Value(Units::Pixels(primary_window.height())) {
            app_style.height = StyleProp::Value(Units::Pixels(primary_window.height()));
            has_changed = true;
        }

        if has_changed {
            app.children.build(Some(entity), &mut widget_tree, &mut commands);
        }
    }

    has_changed
}