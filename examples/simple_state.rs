use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Changed, Commands, Component, Entity, In, Query, Res, ResMut, Vec2,
    },
    DefaultPlugins
};
use kayak_ui::prelude::{widgets::*, Style, *};

#[derive(Component, Default)]
struct CurrentCount(pub u32);

impl Widget for CurrentCount {}

fn current_count_update(
    In((widget_context, entity)): In<(WidgetContext, Entity)>,
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
        widget_context.add(text_entity, Some(entity));

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
        .insert_bundle(KayakAppBundle {
            children: Children::new(|app_id, widget_context, commands| {
                let window_entity = commands.spawn().insert_bundle(WindowBundle {
                    window: Window {
                        title: "State Example Window".into(),
                        draggable: true,
                        position: Vec2::new(10.0, 10.0),
                        size: Vec2::new(300.0, 250.0),
                        ..Window::default()
                    },
                    children: Children::new(|window_id, widget_context, commands| {
                        let current_count_entity = commands
                            .spawn()
                            .insert(CurrentCount(0))
                            .insert(Style::default())
                            .insert(WidgetName(CurrentCount::default().get_name()))
                            .id();
                        widget_context.add(current_count_entity, window_id);

                        let button_entity = commands
                            .spawn()
                            .insert_bundle(ButtonBundle {
                                children: Children::new(|button_id, widget_context, commands| {
                                    let text_entity = commands
                                        .spawn()
                                        .insert_bundle(kayak_ui::prelude::widgets::TextBundle {
                                            text: kayak_ui::prelude::widgets::Text {
                                                content: "Click me!".into(),
                                                size: 16.0,
                                                alignment: Alignment::Start,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .id();
                                    widget_context.add(text_entity, button_id);
                                }),
                                on_event: OnEvent::new(
                                    move |In((event_dispatcher_context, event, _entity)): In<(EventDispatcherContext, Event, Entity)>,
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
                                        (event_dispatcher_context, event)
                                    },
                                ),
                                ..Default::default()
                            })
                            .id();
                        widget_context.add(button_entity, window_id);
                    }),
                    ..WindowBundle::default()
                }).id();
                widget_context.add(window_entity, app_id);
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
        .add_plugin(ContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}
