use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Commands, Res,
        ResMut, ImageSettings,
    },
    DefaultPlugins,
};
use kayak_ui::prelude::{widgets::*, Style, *};

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn().insert_bundle(UICameraBundle::new());

    let image = asset_server.load("generic-rpg-vendor.png");

    let mut context = Context::new();
    let entity = commands
        .spawn()
        .insert_bundle(KayakAppBundle {
            children: Children::new(move |parent_id, widget_context, commands| {
                let image_entity = commands.spawn().insert_bundle(ImageBundle {
                    image: Image(image.clone()),
                    style: Style {
                        position_type: StyleProp::Value(PositionType::SelfDirected),
                        left: StyleProp::Value(Units::Pixels(10.0)),
                        top: StyleProp::Value(Units::Pixels(10.0)),
                        border_radius: StyleProp::Value(Corner::all(500.0)),
                        width: StyleProp::Value(Units::Pixels(200.0)),
                        height: StyleProp::Value(Units::Pixels(182.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                }).id();
                widget_context.add(image_entity, parent_id);
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

fn main() {
    BevyApp::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(ContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}
