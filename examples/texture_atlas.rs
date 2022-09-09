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

    let image_handle = asset_server.load("texture_atlas.png");

    //texture_atlas.png uses 16 pixel sprites and is 272x128 pixels
    let tile_size = 16;
    let columns = 272 / tile_size;
    let rows = 128 / tile_size;
    let atlas = bevy::sprite::TextureAtlas::from_grid(
        image_handle.clone(),
        bevy::prelude::Vec2::splat(tile_size as f32),
        columns,
        rows,
    );

    //The sign in the top right of the image would be index 16
    let sign_index = 16;
    //The flower is in the 6(-1) row and 15 collumn
    let flower_index = columns * 5 + 15;

    let mut context = Context::new();
    let entity = commands
        .spawn()
        .insert_bundle(KayakAppBundle {
            children: Children::new(move |parent_id, widget_context, commands| {
                let atlas_styles = Style {
                    position_type: StyleProp::Value(PositionType::ParentDirected),
                    width: StyleProp::Value(Units::Pixels(200.0)),
                    height: StyleProp::Value(Units::Pixels(200.0)),
                    ..Style::default()
                };
        
                let rect = atlas.textures[sign_index];
                let sign_position = rect.min;
                let sign_size = rect.max - rect.min;
        
                let rect = atlas.textures[flower_index];
                let flower_position = rect.min;
                let flower_size = rect.max - rect.min;

                let image_entity = commands.spawn().insert_bundle(TextureAtlasBundle {
                    atlas: TextureAtlas {
                        handle: image_handle.clone(),
                        position: sign_position,
                        tile_size: sign_size,
                    },
                    styles: atlas_styles.clone(),
                    ..Default::default()
                }).id();
                widget_context.add(image_entity, parent_id);

                let image_entity = commands.spawn().insert_bundle(TextureAtlasBundle {
                    atlas: TextureAtlas {
                        handle: image_handle.clone(),
                        position: flower_position,
                        tile_size: flower_size,
                    },
                    styles: atlas_styles.clone(),
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
