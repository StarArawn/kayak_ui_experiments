use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{
        App as BevyApp, AssetServer, Changed, Commands, Component, Entity, In, Query, Res, ResMut,
    },
    DefaultPlugins,
};
use kayak_ui::prelude::{widgets::*, Style, *};
use morphorm::Units;

#[derive(Component, Default)]
struct CurrentCount(pub u32);

impl Widget for CurrentCount {}

fn current_count_update(
    In((widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    query: Query<&CurrentCount, Changed<CurrentCount>>,
) -> bool {
    if let Ok(current_count) = query.get(entity) {
        let text_entity = commands
            .spawn()
            .insert_bundle(kayak_ui::prelude::widgets::TextBundle {
                text: kayak_ui::prelude::widgets::Text {
                    content: format!("Current Count: {}", current_count.0).into(),
                    size: 16.0,
                    line_height: Some(40.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        widget_tree.add::<kayak_ui::prelude::widgets::Text>(text_entity, Some(entity));

        return true;
    }

    false
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn().insert_bundle(UICameraBundle::new());

    let mut context = Context::new();
    context.add_widget_system(CurrentCount::default().get_name(), current_count_update);

    let entity = commands
        .spawn()
        .insert(KayakApp)
        .insert(Children::new(|app_id, widget_tree, commands| {
            let current_count_entity = commands
                .spawn()
                .insert(CurrentCount(0))
                .insert(Style::default())
                .insert(DirtyNode)
                .id();
            widget_tree.add::<CurrentCount>(current_count_entity, app_id);

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
                            .id();
                        widget_tree.add::<kayak_ui::prelude::widgets::Text>(text_entity, button_id);
                    }),
                    on_event: OnEvent::new(
                        move |In((event, _entity)): In<(Event, Entity)>,
                              mut query: Query<&mut CurrentCount>| {
                            match event.event_type {
                                EventType::Click(..) => {
                                    if let Ok(mut current_count) =
                                        query.get_mut(current_count_entity)
                                    {
                                        current_count.0 += 1;
                                    }
                                }
                                _ => {}
                            }
                            event
                        },
                    ),
                    ..Default::default()
                })
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
