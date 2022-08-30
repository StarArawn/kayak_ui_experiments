use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Commands, Res, ResMut,
    },
    DefaultPlugins, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
};
use kayak_ui::prelude::{Style, *, widgets::*};

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn().insert_bundle(UICameraBundle::new());

    let mut context = Context::new();
    let entity = commands
        .spawn()
        .insert(KayakApp)
        .insert(Children::new(|app_id, widget_tree, commands| {
            let current_count = Binding::new(0);
            let text_entity = commands.spawn()
                .insert_bundle(kayak_ui::prelude::widgets::TextBundle {
                    text: kayak_ui::prelude::widgets::Text {
                        content: format!("Current Count: {}", current_count.get()).into(),
                        size: 16.0,
                        line_height: Some(40.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(DirtyNode)
                .id();
            widget_tree.add::<kayak_ui::prelude::widgets::Text>(text_entity, app_id);

            let button_entity = commands
                .spawn()
                .insert_bundle(ButtonBundle {
                    children: Children::new(|button_id, widget_tree, commands| {
                        let text_entity = commands
                            .spawn()
                            .insert_bundle(kayak_ui::prelude::widgets::TextBundle {
                                text: kayak_ui::prelude::widgets::Text {
                                    content: "Click me!".into(),
                                    size: 16.0,
                                    line_height: Some(40.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(DirtyNode)
                            .id();
                        widget_tree.add::<kayak_ui::prelude::widgets::Text>(text_entity, button_id);
                    }),
                    ..Default::default()
                })
                .insert(DirtyNode)
                .id();
            widget_tree.add::<Button>(button_entity, app_id);
        }))
        .insert(Style {
            render_command: StyleProp::Value(RenderCommand::Layout),
            ..Style::new_default()
        })
        .id();
    context.add_widget::<KayakApp>(None, entity);
    commands.insert_resource(context);
}

fn main() {
    BevyApp::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}
