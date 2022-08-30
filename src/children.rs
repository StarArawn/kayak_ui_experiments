use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use bevy::prelude::{Commands, Entity};

use crate::prelude::WidgetTree;

/// A container for a function that generates child widgets
#[derive(Clone)]
pub struct Children(Arc<dyn Fn(Option<Entity>, &mut WidgetTree, &mut Commands) + Send + Sync>);

impl Default for Children {
    fn default() -> Self {
        Children::new(|_e, _w, _c| {})
    }
}

impl Children {
    pub fn new<F: Fn(Option<Entity>, &mut WidgetTree, &mut Commands) + Send + Sync + 'static>(
        builder: F,
    ) -> Self {
        Self(Arc::new(builder))
    }
    pub fn build(&self, id: Option<Entity>, widget_tree: &mut WidgetTree, commands: &mut Commands) {
        self.0(id, widget_tree, commands);
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
