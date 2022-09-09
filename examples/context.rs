//! This example demonstrates how to use the provider/consumer pattern for passing props down
//! to multiple descendants.
//!
//! The problem we'll be solving here is adding support for theming.
//!
//! One reason the provider/consumer pattern might be favored over a global state is that it allows
//! for better specificity and makes local contexts much easier to manage. In the case of theming,
//! this allows us to have multiple active themes, even if they are nested within each other!

use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Bundle, Changed, Color, Commands, Component, Entity,
        ImageSettings, In, ParamSet, Query, Res, ResMut, Vec2,
    },
    DefaultPlugins,
};
use kayak_ui::prelude::{widgets::*, Style, *};

/// The color theme struct we will be using across our demo widgets
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct Theme {
    name: String,
    primary: Color,
    secondary: Color,
    background: Color,
}

impl Theme {
    fn vampire() -> Self {
        Self {
            name: "Vampire".to_string(),
            primary: Color::rgba(1.0, 0.475, 0.776, 1.0),
            secondary: Color::rgba(0.641, 0.476, 0.876, 1.0),
            background: Color::rgba(0.157, 0.165, 0.212, 1.0),
        }
    }
    fn solar() -> Self {
        Self {
            name: "Solar".to_string(),
            primary: Color::rgba(0.514, 0.580, 0.588, 1.0),
            secondary: Color::rgba(0.149, 0.545, 0.824, 1.0),
            background: Color::rgba(0.026, 0.212, 0.259, 1.0),
        }
    }
    fn vector() -> Self {
        Self {
            name: "Vector".to_string(),
            primary: Color::rgba(0.533, 1.0, 0.533, 1.0),
            secondary: Color::rgba(0.098, 0.451, 0.098, 1.0),
            background: Color::rgba(0.004, 0.059, 0.004, 1.0),
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
struct ThemeButton {
    pub theme: Theme,
}
impl Widget for ThemeButton {}

#[derive(Bundle)]
pub struct ThemeButtonBundle {
    theme_button: ThemeButton,
    widget_name: WidgetName,
}

impl Default for ThemeButtonBundle {
    fn default() -> Self {
        Self {
            theme_button: Default::default(),
            widget_name: WidgetName(ThemeButton::default().get_name()),
        }
    }
}

fn update_theme_button(
    In((widget_context, theme_button_entity)): In<(WidgetContext, Entity)>,
    mut commands: Commands,
    query: Query<&ThemeButton>,
    changed_query: Query<&ThemeButton, Changed<ThemeButton>>,
    mut context_query: ParamSet<(Query<&mut Theme>, Query<&Theme, Changed<Theme>>)>,
) -> bool {
    if !context_query.p1().is_empty() || !changed_query.is_empty() {
        if let Ok(theme_button) = query.get(theme_button_entity) {
            if let Some(theme_context_entity) =
                widget_context.get_context_entity::<Theme>(theme_button_entity)
            {
                if let Ok(theme) = context_query.p0().get_mut(theme_context_entity) {
                    let mut box_style = Style {
                        width: StyleProp::Value(Units::Pixels(30.0)),
                        height: StyleProp::Value(Units::Pixels(30.0)),
                        background_color: StyleProp::Value(theme_button.theme.primary),
                        ..Default::default()
                    };

                    if theme_button.theme.name == theme.name {
                        box_style.top = StyleProp::Value(Units::Pixels(3.0));
                        box_style.left = StyleProp::Value(Units::Pixels(3.0));
                        box_style.bottom = StyleProp::Value(Units::Pixels(3.0));
                        box_style.right = StyleProp::Value(Units::Pixels(3.0));
                        box_style.width = StyleProp::Value(Units::Pixels(24.0));
                        box_style.height = StyleProp::Value(Units::Pixels(24.0));
                    }
                    let background_entity = commands
                        .spawn()
                        .insert_bundle(BackgroundBundle {
                            styles: box_style,
                            on_event: OnEvent::new(
                                move |In((event_dispatcher_context, event, _entity)): In<(
                                    EventDispatcherContext,
                                    Event,
                                    Entity,
                                )>,
                                query: Query<&ThemeButton>,
                                mut context_query: Query<&mut Theme>,
                                | {
                                    match event.event_type {
                                        EventType::Click(..) => {
                                            if let Ok(button) = query.get(theme_button_entity) {
                                                if let Ok(mut context_theme) = context_query.get_mut(theme_context_entity) {
                                                    *context_theme = button.theme.clone();
                                                }
                                            }
                                        },
                                        _ => {}
                                    }
                                    (event_dispatcher_context, event)
                                },
                            ),
                            ..Default::default()
                        })
                        .id();
                    widget_context.add(background_entity, Some(theme_button_entity));

                    return true;
                }
            }
        }
    }

    false
}

#[derive(Component, Debug, Default, Clone)]
struct ThemeSelector;
impl Widget for ThemeSelector {}

#[derive(Bundle)]
pub struct ThemeSelectorBundle {
    theme_selector: ThemeSelector,
    widget_name: WidgetName,
}

impl Default for ThemeSelectorBundle {
    fn default() -> Self {
        Self {
            theme_selector: Default::default(),
            widget_name: WidgetName(ThemeSelector::default().get_name()),
        }
    }
}

fn update_theme_selector(
    In((widget_context, entity)): In<(WidgetContext, Entity)>,
    mut commands: Commands,
    query: Query<&ThemeSelector, Changed<ThemeSelector>>,
) -> bool {
    if let Ok(_) = query.get(entity) {
        let button_container_style = Style {
            layout_type: StyleProp::Value(LayoutType::Row),
            width: StyleProp::Value(Units::Stretch(1.0)),
            height: StyleProp::Value(Units::Auto),
            top: StyleProp::Value(Units::Pixels(5.0)),
            ..Default::default()
        };
        let element_entity = commands
            .spawn()
            .insert_bundle(ElementBundle {
                styles: button_container_style,
                children: Children::new(|parent_id, widget_context, commands| {
                    let vampire_theme = Theme::vampire();
                    let solar_theme = Theme::solar();
                    let vector_theme = Theme::vector();
                    let vampire_button_entity = commands
                        .spawn()
                        .insert_bundle(ThemeButtonBundle {
                            theme_button: ThemeButton {
                                theme: vampire_theme,
                            },
                            ..Default::default()
                        })
                        .id();
                    widget_context.add(vampire_button_entity, parent_id);
                    let solar_button_entity = commands
                        .spawn()
                        .insert_bundle(ThemeButtonBundle {
                            theme_button: ThemeButton { theme: solar_theme },
                            ..Default::default()
                        })
                        .id();
                    widget_context.add(solar_button_entity, parent_id);
                    let vector_button_entity = commands
                        .spawn()
                        .insert_bundle(ThemeButtonBundle {
                            theme_button: ThemeButton {
                                theme: vector_theme,
                            },
                            ..Default::default()
                        })
                        .id();
                    widget_context.add(vector_button_entity, parent_id);
                }),
                ..Default::default()
            })
            .id();
        widget_context.add(element_entity, Some(entity));
        return true;
    }

    false
}

#[derive(Component, Debug, Default, Clone)]
pub struct ThemeDemo {
    is_root: bool,
    context_entity: Option<Entity>,
}
impl Widget for ThemeDemo {}

#[derive(Bundle)]
pub struct ThemeDemoBundle {
    theme_demo: ThemeDemo,
    widget_name: WidgetName,
}

impl Default for ThemeDemoBundle {
    fn default() -> Self {
        Self {
            theme_demo: Default::default(),
            widget_name: WidgetName(ThemeDemo::default().get_name()),
        }
    }
}

fn update_theme_demo(
    In((widget_context, entity)): In<(WidgetContext, Entity)>,
    mut commands: Commands,
    mut query_set: ParamSet<(Query<&mut ThemeDemo>, Query<&ThemeDemo, Changed<ThemeDemo>>)>,
    theme_context: Query<&Theme>,
    detect_changes_query: Query<&Theme, Changed<Theme>>,
) -> bool {
    if !detect_changes_query.is_empty() || !query_set.p1().is_empty() {
        if let Ok(mut theme_demo) = query_set.p0().get_mut(entity) {
            if let Some(theme_context_entity) = widget_context.get_context_entity::<Theme>(entity) {
                if let Ok(theme) = theme_context.get(theme_context_entity) {
                    let select_lbl = if theme_demo.is_root {
                        format!("Select Theme (Current: {})", theme.name)
                    } else {
                        format!("Select A Different Theme (Current: {})", theme.name)
                    };

                    let select_text_entity = commands
                        .spawn()
                        .insert_bundle(TextBundle {
                            text: Text {
                                content: select_lbl,
                                size: 14.0,
                                line_height: Some(28.0),
                                ..Default::default()
                            },
                            styles: Style {
                                height: StyleProp::Value(Units::Pixels(28.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .id();
                    widget_context.add(select_text_entity, Some(entity));

                    let theme_selector_entity = commands
                        .spawn()
                        .insert_bundle(ThemeSelectorBundle::default())
                        .id();
                    widget_context.add(theme_selector_entity, Some(entity));

                    if theme_demo.is_root {
                        if theme_demo.context_entity.is_none() {
                            let theme_entity = commands.spawn().insert(Theme::vector()).id();
                            theme_demo.context_entity = Some(theme_entity);
                        }
                    }

                    let context_entity = if let Some(entity) = theme_demo.context_entity {
                        entity
                    } else {
                        Entity::from_raw(1000000)
                    };
                    let text_styles = Style {
                        color: StyleProp::Value(theme.primary),
                        height: StyleProp::Value(Units::Pixels(28.0)),
                        ..Default::default()
                    };
                    let btn_style = Style {
                        background_color: StyleProp::Value(theme.secondary),
                        width: StyleProp::Value(Units::Stretch(0.75)),
                        height: StyleProp::Value(Units::Pixels(32.0)),
                        top: StyleProp::Value(Units::Pixels(5.0)),
                        left: StyleProp::Value(Units::Stretch(1.0)),
                        right: StyleProp::Value(Units::Stretch(1.0)),
                        ..Default::default()
                    };

                    let is_root = theme_demo.is_root;
                    let background_children =
                        Children::new(move |parent_id, widget_context, commands| {
                            let text_entity = commands
                                .spawn()
                                .insert_bundle(TextBundle {
                                    text: Text {
                                        content: "Lorem ipsum dolor...".into(),
                                        size: 12.0,
                                        ..Default::default()
                                    },
                                    styles: text_styles.clone(),
                                    ..Default::default()
                                })
                                .id();
                            widget_context.add(text_entity, parent_id);

                            let button_entity = commands
                                .spawn()
                                .insert_bundle(ButtonBundle {
                                    styles: btn_style.clone(),
                                    children: Children::new(|parent_id, widget_context, commands| {
                                        let text_entity = commands.spawn().insert_bundle(TextBundle {
                                            text: Text {
                                                content: "BUTTON".into(),
                                                size: 14.0,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        }).id();
                                        widget_context.add(text_entity, parent_id);
                                    }),
                                    ..Default::default()
                                })
                                .id();
                            widget_context.add(button_entity, parent_id);

                            if is_root {
                                let element_entity = commands
                                    .spawn()
                                    .insert_bundle(ElementBundle {
                                        styles: Style {
                                            top: StyleProp::Value(Units::Pixels(10.0)),
                                            left: StyleProp::Value(Units::Pixels(10.0)),
                                            bottom: StyleProp::Value(Units::Pixels(10.0)),
                                            right: StyleProp::Value(Units::Pixels(10.0)),
                                            ..Default::default()
                                        },
                                        children: Children::new(
                                            move |parent_id, widget_context, commands| {
                                                widget_context.set_context_entity::<Theme>(
                                                    parent_id,
                                                    context_entity,
                                                );
                                                let theme_demo_child = commands
                                                    .spawn()
                                                    .insert_bundle(ThemeDemoBundle::default())
                                                    .id();
                                                widget_context.add(theme_demo_child, parent_id);
                                            },
                                        ),
                                        ..Default::default()
                                    })
                                    .id();
                                widget_context.add(element_entity, parent_id);
                            }
                        });

                    let background_widget_entity = commands
                        .spawn()
                        .insert_bundle(BackgroundBundle {
                            styles: Style {
                                background_color: StyleProp::Value(theme.background),
                                top: StyleProp::Value(Units::Pixels(15.0)),
                                width: StyleProp::Value(Units::Stretch(1.0)),
                                height: StyleProp::Value(Units::Stretch(1.0)),
                                ..Default::default()
                            },
                            children: background_children,
                            ..Default::default()
                        })
                        .id();
                    widget_context.add(background_widget_entity, Some(entity));

                    return true;
                }
            }
        }
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
    context.add_widget_system(ThemeDemo::default().get_name(), update_theme_demo);
    context.add_widget_system(ThemeButton::default().get_name(), update_theme_button);
    context.add_widget_system(ThemeSelector::default().get_name(), update_theme_selector);
    let entity = commands
        .spawn()
        .insert_bundle(KayakAppBundle {
            children: Children::new(move |parent_id, widget_context, commands| {
                let theme_entity = commands.spawn().insert(Theme::vampire()).id();
                widget_context.set_context_entity::<Theme>(parent_id, theme_entity);

                let window_entity = commands
                    .spawn()
                    .insert_bundle(WindowBundle {
                        window: Window {
                            title: "Context Example".into(),
                            draggable: true,
                            position: Vec2::ZERO,
                            size: Vec2::new(350.0, 400.0),
                            ..Default::default()
                        },
                        children: Children::new(|parent_id, widget_context, commands| {
                            let theme_demo_entity = commands
                                .spawn()
                                .insert_bundle(ThemeDemoBundle {
                                    theme_demo: ThemeDemo {
                                        is_root: true,
                                        context_entity: None,
                                    },
                                    ..Default::default()
                                })
                                .id();
                            widget_context.add(theme_demo_entity, parent_id);
                        }),
                        ..Default::default()
                    })
                    .id();
                widget_context.add(window_entity, parent_id);
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
