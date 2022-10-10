use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Bundle, Commands, Component, ImageSettings, Res, ResMut,
    },
    DefaultPlugins,
};
use kayak_ui::prelude::WidgetName;
use kayak_ui::prelude::{widgets::KayakWidgets, Context, ContextPlugin, Style};
use kayak_ui::prelude::{Children, Widget, WidgetBundle};
use kayak_ui_macros::rsx;

#[derive(Component, Debug, Default)]
pub struct MyWidgetProps {
    foo: u32,
}

impl Widget for MyWidgetProps {}

#[derive(Bundle, Debug)]
struct MyWidgetBundle {
    props: MyWidgetProps,
    children: Children,
    styles: Style,
    widget_name: WidgetName,
}

impl Default for MyWidgetBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            children: Default::default(),
            styles: Default::default(),
            widget_name: WidgetName(MyWidgetProps::default().get_name()),
        }
    }
}

pub fn startup(mut commands: Commands) {
    let styles = Style::default();

    let mut widget_context = Context::new();
    let parent_id = None;
    let foo = 0;
    // rsx! {
    //     {
    //         if foo == 0 {
    //             <MyWidgetBundle props={MyWidgetProps { foo: 0 }} styles={styles}>
    //                 <MyWidgetBundle props={MyWidgetProps { foo: 1 }} />
    //                 <MyWidgetBundle props={MyWidgetProps { foo: 1 }} />
    //             </MyWidgetBundle>
    //         }
    //     }
    // }
    rsx! {
        <>
            {
                if foo == 0 {
                    rsx! {
                        <MyWidgetBundle>
                            <MyWidgetBundle />
                            <MyWidgetBundle />
                        </MyWidgetBundle>
                    }
                }
            }
        </>
    }

    commands.insert_resource(widget_context);
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
