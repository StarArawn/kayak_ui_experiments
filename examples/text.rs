#![allow(dead_code)]

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use kayak_ui::prelude::{Style, *};

#[derive(Component, Default)]
pub struct MyWidget {
    pub foo: u32,
}

fn my_widget_1_update(
    In((_widget_tree, entity)): In<(WidgetTree, Entity)>,
    my_resource: Res<MyResource>,
    mut query: Query<(&mut MyWidget, &mut Style)>,
) -> bool {
    if my_resource.is_changed() {
        if let Ok((mut my_widget, mut style)) = query.get_mut(entity) {
            my_widget.foo = my_resource.0;
            dbg!(my_widget.foo);
            style.render_command = StyleProp::Value(RenderCommand::Text {
                content: format!("My number is: {}", my_widget.foo).to_string(),
            });
            return true;
        }
    }

    false
}

impl Widget for MyWidget {}

pub struct MyResource(pub u32);

fn startup(mut commands: Commands) {
    let mut context = Context::new();
    context.register_widget_system(MyWidget::default().get_name(), my_widget_1_update);
    let entity = commands
        .spawn()
        .insert(MyWidget { foo: 0 })
        .insert(Style {
            render_command: StyleProp::Value(RenderCommand::Text {
                content: format!("My number is: {}", 0).to_string(),
            }),
            ..Style::new_default()
        })
        .id();
    context.add_widget::<MyWidget>(None, entity);
    commands.insert_resource(Some(context));
}

fn update_resource(keyboard_input: Res<Input<KeyCode>>, mut my_resource: ResMut<MyResource>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        my_resource.0 += 1;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ContextPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(MyResource(1))
        .add_startup_system(startup)
        .add_system(update_resource)
        .run()
}
