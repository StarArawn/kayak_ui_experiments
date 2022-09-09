use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Commands, Component, Entity, In, Input, KeyCode, Query, Res,
        ResMut, Resource,
    },
    DefaultPlugins,
};
use kayak_ui::prelude::{widgets::*, Style, *};

#[derive(Component, Default)]
pub struct MyWidget {
    pub foo: u32,
}

fn my_widget_1_update(
    In((_widget_context, entity)): In<(WidgetContext, Entity)>,
    my_resource: Res<MyResource>,
    mut query: Query<(&mut MyWidget, &mut Style)>,
) -> bool {
    if my_resource.is_changed() || my_resource.is_added() {
        if let Ok((mut my_widget, mut style)) = query.get_mut(entity) {
            my_widget.foo = my_resource.0;
            dbg!(my_widget.foo);
            style.render_command = StyleProp::Value(RenderCommand::Text {
                content: format!("My number is: {}", my_widget.foo).to_string(),
                alignment: Alignment::Start,
            });
            return true;
        }
    }

    false
}

impl Widget for MyWidget {}

#[derive(Resource)]
pub struct MyResource(pub u32);

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn().insert_bundle(UICameraBundle::new());

    let mut context = Context::new();
    context.add_widget_system(MyWidget::default().get_name(), my_widget_1_update);
    let entity = commands
        .spawn()
        .insert_bundle(KayakAppBundle {
            children: Children::new(|parent_id, widget_context, commands| {
                let my_widget_entity = commands
                    .spawn()
                    .insert(MyWidget { foo: 0 })
                    .insert(Style::default())
                    .insert(WidgetName(MyWidget::default().get_name()))
                    .id();
                widget_context.add(my_widget_entity, parent_id);
            }),
            styles: Style {
                render_command: StyleProp::Value(RenderCommand::Layout),
                ..Style::new_default()
            },
            ..Default::default()
        })
        .id();
    context.add_widget(None, entity);
    commands.insert_resource(context);
}

fn update_resource(keyboard_input: Res<Input<KeyCode>>, mut my_resource: ResMut<MyResource>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        my_resource.0 += 1;
    }
}

fn main() {
    BevyApp::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ContextPlugin)
        .add_plugin(KayakWidgets)
        .insert_resource(MyResource(1))
        .add_startup_system(startup)
        .add_system(update_resource)
        .run()
}
