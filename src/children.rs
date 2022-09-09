use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use bevy::prelude::{Commands, Entity, Component};

use crate::prelude::WidgetContext;

/// A container for a function that generates child widgets
#[derive(Component, Clone)]
pub struct Children(Arc<dyn Fn(Option<Entity>, &WidgetContext, &mut Commands) + Send + Sync>);

impl Default for Children {
    fn default() -> Self {
        Children::new(|_e, _w, _c| {})
    }
}

impl Children {
    pub fn new<F: Fn(Option<Entity>, &WidgetContext, &mut Commands) + Send + Sync + 'static>(
        builder: F,
    ) -> Self {
        Self(Arc::new(builder))
    }
    pub fn spawn(&self, id: Option<Entity>, widget_context: &WidgetContext, commands: &mut Commands) {
        self.0(id, widget_context, commands);
    }
}

impl Debug for Children {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Children").finish()
    }
}

impl PartialEq for Children {
    fn eq(&self, _: &Self) -> bool {
        // Never prevent "==" for being true because of this struct
        true
    }
}
