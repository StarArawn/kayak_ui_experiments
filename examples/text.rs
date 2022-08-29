use bevy::{
    prelude::{
        App as BevyApp, Commands, Component, Entity, In, Input, KeyCode, Query, Res, ResMut, With, AssetServer,
    },
    window::Windows,
    DefaultPlugins,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use kayak_ui::prelude::{Style, *};
use morphorm::Units;

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

#[derive(Component, Default)]
pub struct App;

impl Widget for App {}

fn app_update(
    In((widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<&mut Style, With<App>>,
) -> bool {
    let mut has_changed = false;
    let primary_window = windows.get_primary().unwrap();
    for mut app_style in query.iter_mut() {
        if app_style.width != StyleProp::Value(Units::Pixels(primary_window.width())) {
            app_style.width = StyleProp::Value(Units::Pixels(primary_window.width()));
            has_changed = true;
        }
        if app_style.height != StyleProp::Value(Units::Pixels(primary_window.height())) {
            app_style.height = StyleProp::Value(Units::Pixels(primary_window.height()));
            has_changed = true;
        }
    }

    if has_changed {
        let child_id = commands
            .spawn()
            .insert(MyWidget { foo: 0 })
            .insert(Style {
                render_command: StyleProp::Value(RenderCommand::Text {
                    content: format!("My number is: {}", 0).to_string(),
                }),
                ..Style::new_default()
            })
            .id();
        widget_tree.add::<MyWidget>(child_id, Some(entity));
    }

    has_changed
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn().insert_bundle(UICameraBundle::new());

    let mut context = Context::new();
    context.register_widget_system(MyWidget::default().get_name(), my_widget_1_update);
    context.register_widget_system(App::default().get_name(), app_update);
    let entity = commands
        .spawn()
        .insert(App)
        .insert(Style {
            render_command: StyleProp::Value(RenderCommand::Layout),
            ..Style::new_default()
        })
        .id();
    context.add_widget::<App>(None, entity);
    commands.insert_resource(Some(context));
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
        // .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(MyResource(1))
        .add_startup_system(startup)
        .add_system(update_resource)
        .run()
}
