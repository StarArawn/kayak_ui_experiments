use bevy::prelude::*;

mod app;
mod button;
mod text;
mod window;
mod background;
mod clip;
mod image;

pub use app::{KayakApp, KayakAppBundle};
pub use button::{Button, ButtonBundle};
pub use text::{Text, TextBundle};
pub use window::{Window, WindowBundle};
pub use background::{Background, BackgroundBundle};
pub use clip::{Clip, ClipBundle};
pub use image::{Image, ImageBundle};

use app::app_update;
use button::button_update;
use text::text_update;
use window::window_update;
use background::update_background;
use clip::update_clip;
use image::update_image;

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
    context.add_widget_system(Window::default().get_name(), window_update);
    context.add_widget_system(Background::default().get_name(), update_background);
    context.add_widget_system(Clip::default().get_name(), update_clip);
    context.add_widget_system(Image::default().get_name(), update_image);
}
