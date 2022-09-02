use bevy::{prelude::{Component, Commands, In, Entity, Res, Query, With, Bundle}, window::Windows};
use morphorm::Units;

use crate::{styles::{StyleProp, Style}, widget::Widget, children::Children, prelude::WidgetTree, context::WidgetName};

#[derive(Component, Default)]
pub struct KayakApp;

impl Widget for KayakApp {}

#[derive(Bundle)]
pub struct KayakAppBundle {
    pub app: KayakApp,
    pub styles: Style,
    pub children: Children,
    pub widget_name: WidgetName,
}

impl Default for KayakAppBundle {
    fn default() -> Self {
        Self { app: Default::default(), styles: Default::default(), children: Default::default(), widget_name: WidgetName(KayakApp::default().get_name()) }
    }
}

/// TODO: USE CAMERA INSTEAD OF WINDOW!!
pub fn app_update(
    In((mut widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<(&mut Style, &Children), With<KayakApp>>,
) -> bool {
    let mut has_changed = false;
    let primary_window = windows.get_primary().unwrap();
    if let Ok((mut app_style, children)) = query.get_mut(entity) {
        if app_style.width != StyleProp::Value(Units::Pixels(primary_window.width())) {
            app_style.width = StyleProp::Value(Units::Pixels(primary_window.width()));
            has_changed = true;
        }
        if app_style.height != StyleProp::Value(Units::Pixels(primary_window.height())) {
            app_style.height = StyleProp::Value(Units::Pixels(primary_window.height()));
            has_changed = true;
        }

        if has_changed {
            children.spawn(Some(entity), &mut widget_tree, &mut commands, true);
        }
    }

    has_changed
}