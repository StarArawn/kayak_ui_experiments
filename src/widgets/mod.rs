use bevy::prelude::*;

mod app;
mod background;
mod button;
mod clip;
mod element;
mod image;
mod nine_patch;
mod text;
mod texture_atlas;
mod window;

pub use app::{KayakApp, KayakAppBundle};
pub use background::{Background, BackgroundBundle};
pub use button::{Button, ButtonBundle};
pub use clip::{Clip, ClipBundle};
pub use element::{Element, ElementBundle};
pub use image::{Image, ImageBundle};
pub use nine_patch::{NinePatch, NinePatchBundle};
pub use text::{Text, TextBundle};
pub use texture_atlas::{TextureAtlas, TextureAtlasBundle};
pub use window::{Window, WindowBundle};

use app::app_update;
use background::update_background;
use button::button_update;
use clip::update_clip;
use element::update_element;
use image::update_image;
use nine_patch::update_nine_patch;
use text::text_update;
use texture_atlas::update_texture_atlas;
use window::window_update;

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
    context.add_widget_system(TextureAtlas::default().get_name(), update_texture_atlas);
    context.add_widget_system(NinePatch::default().get_name(), update_nine_patch);
    context.add_widget_system(Element::default().get_name(), update_element);
}
