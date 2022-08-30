use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock};
use bevy::prelude::Component;

use crate::event::Event;

/// A container for a function that handles events
///
/// This differs from a standard [`Handler`](crate::Handler) in that it's sent directly
/// from the [`KayakContext`](crate::KayakContext) and gives the [`KayakContextRef`]
/// as a parameter.
#[derive(Component, Clone)]
pub struct OnEvent(
    Arc<RwLock<dyn FnMut(&mut Event) + Send + Sync + 'static>>,
);

impl Default for OnEvent {
    fn default() -> Self {
        Self::new(|_e| {})
    }
} 

impl OnEvent {
    /// Create a new event handler
    ///
    /// The handler should be a closure that takes the following arguments:
    /// 1. The current context
    /// 2. The event
    pub fn new<F: FnMut(&mut Event) + Send + Sync + 'static>(
        f: F,
    ) -> OnEvent {
        OnEvent(Arc::new(RwLock::new(f)))
    }

    /// Call the event handler
    ///
    /// Returns true if the handler was successfully invoked.
    pub fn try_call(&self, event: &mut Event) -> bool {
        if let Ok(mut on_event) = self.0.write() {
            on_event(event);
            true
        } else {
            false
        }
    }
}

impl Debug for OnEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnEvent").finish()
    }
}

impl PartialEq for OnEvent {
    fn eq(&self, _: &Self) -> bool {
        // Never prevent "==" for being true because of this struct
        true
    }
}
