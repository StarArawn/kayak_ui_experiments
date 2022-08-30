use bevy::prelude::*;

mod app;

pub use app::{KayakApp};
use app::app_update;

use crate::{context::Context, widget::Widget};

pub struct KayakWidgets;

impl Plugin for KayakWidgets {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_widget_systems);
    }
}

fn add_widget_systems(mut context: ResMut<Option<Context>>) {
    if let Some(context) = (*context).as_mut() { 
        context.register_widget_system(KayakApp::default().get_name(), app_update);
    }
}
