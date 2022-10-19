use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Commands, Res,
        ResMut, Vec2,
    },
    DefaultPlugins,
};
use kayak_ui::prelude::{widgets::*, *};

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn(UICameraBundle::new());

    let mut widget_context = Context::new();
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <WindowBundle
                window={Window {
                    title: "Todo App".into(),
                    draggable: true,
                    position: Vec2::new(10.0, 10.0),
                    size: Vec2::new(300.0, 250.0),
                    ..Window::default()
                }}
            >
            </WindowBundle>
        </KayakAppBundle>
    }
    commands.insert_resource(widget_context);
}

fn main() {
    BevyApp::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}
