use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use bevy::prelude::{Commands, Entity, Component};

use crate::prelude::WidgetTree;

/// A container for a function that generates child widgets
#[derive(Component, Clone)]
pub struct Children(Arc<dyn Fn(Option<Entity>, &WidgetTree, &mut Commands, bool) + Send + Sync>);

impl Default for Children {
    fn default() -> Self {
        Children::new(|_e, _w, _c, _b| {})
    }
}

impl Children {
    pub fn new<F: Fn(Option<Entity>, &WidgetTree, &mut Commands, bool) + Send + Sync + 'static>(
        builder: F,
    ) -> Self {
        Self(Arc::new(builder))
    }
    pub fn spawn(&self, id: Option<Entity>, widget_tree: &WidgetTree, commands: &mut Commands, should_spawn: bool) {
        self.0(id, widget_tree, commands, should_spawn);
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
