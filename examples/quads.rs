use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Changed, Color, Commands, Component, Entity, In, Query, Res, ResMut, Vec2,
    },
    DefaultPlugins, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
};
use kayak_ui::prelude::{Style, *, widgets::*};
use morphorm::{Units, PositionType};

#[derive(Component, Default)]
pub struct MyQuad {
    pos: Vec2,
    pub size: Vec2,
    pub color: Color,
}

fn my_quad_update(
    In((_widget_context, entity)): In<(WidgetContext, Entity)>,
    mut query: Query<(&MyQuad, &mut Style), Changed<MyQuad>>,
) -> bool {
    if let Ok((quad, mut style)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Quad);
        style.position_type = StyleProp::Value(PositionType::SelfDirected);
        style.left = StyleProp::Value(Units::Pixels(quad.pos.x));
        style.top = StyleProp::Value(Units::Pixels(quad.pos.y));
        style.width = StyleProp::Value(Units::Pixels(quad.size.x));
        style.height = StyleProp::Value(Units::Pixels(quad.size.y));
        style.background_color = StyleProp::Value(quad.color);
        return true;
    }

    false
}

impl Widget for MyQuad {}


fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn().insert_bundle(UICameraBundle::new());

    let mut context = Context::new();
    context.add_widget_system(MyQuad::default().get_name(), my_quad_update);
    let entity = commands
        .spawn()
        .insert_bundle(KayakAppBundle {
            children: Children::new(|parent_id, widget_context, commands| {
                for _ in 0..1000 {
                    let pos = Vec2::new(fastrand::i32(0..1280) as f32, fastrand::i32(0..720) as f32);
                    let my_widget_entity = commands
                        .spawn()
                        .insert(MyQuad {
                            pos,
                            size: Vec2::new(
                                fastrand::i32(32..64) as f32,
                                fastrand::i32(32..64) as f32,
                            ),
                            color: Color::rgba(
                                fastrand::f32(),
                                fastrand::f32(),
                                fastrand::f32(),
                                1.0,
                            ),
                        })
                        .insert(Style::default())
                        .insert(WidgetName(MyQuad::default().get_name()))
                        .insert(DirtyNode)
                        .id();
                    widget_context.add(my_widget_entity, parent_id);
                }
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
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}
