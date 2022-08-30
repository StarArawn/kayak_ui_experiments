use bevy::prelude::*;

mod app;
mod button;
mod text;

pub use app::KayakApp;
pub use button::{Button, ButtonBundle};
pub use text::{Text, TextBundle};

use app::app_update;
use button::button_update;
use text::text_update;

use crate::{context::Context, widget::Widget};

pub struct KayakWidgets;

impl Plugin for KayakWidgets {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_widget_systems);
    }
}

fn add_widget_systems(mut context: ResMut<Context>) {
    context.add_widget_system(KayakApp::default().get_name(), app_update);
    context.add_widget_system(Button::default().get_name(), button_update);
    context.add_widget_system(Text::default().get_name(), text_update);
}
