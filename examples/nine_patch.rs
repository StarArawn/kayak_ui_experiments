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

    let image = asset_server.load("panel.png");

    let mut context = Context::new();
    let entity = commands
        .spawn()
        .insert_bundle(KayakAppBundle {
            children: Children::new(move |parent_id, widget_context, commands| {
                // The border prop splits up the image into 9 quadrants like so:
                // 1----2----3
                // |         |
                // 4    9    5
                // |         |
                // 6----7----8
                // The sizes of sprites for a 15 pixel border are as follows:
                // TopLeft = (15, 15)
                // TopRight = (15, 15)
                // LeftCenter = (15, image_height)
                // RightCenter = (15, image_height)
                // TopCenter = (image_width, 15)
                // BottomCenter = (image_width, 15)
                // BottomRight = (15, 15)
                // BottomLeft = (15, 15)
                // Middle = (
                // 30 being left border + right border
                //   image_width - 30
                // 30 being top border + bottom border
                //   image_height - 30
                // )
                //
                let nine_patch_styles = Style {
                    width: StyleProp::Value(Units::Pixels(512.0)),
                    height: StyleProp::Value(Units::Pixels(512.0)),
                    ..Style::default()
                };
                let image_entity = commands.spawn().insert_bundle(NinePatchBundle {
                    nine_patch: NinePatch { handle: image.clone(), border: Edge::all(15.0) },
                    styles: nine_patch_styles,
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
