use std::sync::{Arc, RwLock};

use bevy::prelude::Entity;

use crate::{prelude::Tree, context_entities::ContextEntities, node::WrappedIndex};

#[derive(Clone)]
pub struct WidgetContext {
    old_tree: Arc<RwLock<Tree>>,
    new_tree: Arc<RwLock<Tree>>,
    context_entities: ContextEntities,
}

impl WidgetContext {
    pub fn new(old_tree: Arc<RwLock<Tree>>, context_entities: ContextEntities) -> Self {
        Self {
            old_tree,
            new_tree: Arc::new(RwLock::new(Tree::default())),
            context_entities,
        }
    }

    pub(crate) fn store(&self, new_tree: &Tree) {
        if let Ok(mut tree) = self.new_tree.write() {
            *tree = new_tree.clone();
        }
    }

    /// Creates a new context using the context entity for the given type_id + parent id.
    pub fn set_context_entity<T: Default + 'static>(&self, parent_id: Option<Entity>, context_entity: Entity) {
        if let Some(parent_id) = parent_id {
            self.context_entities.add_context_entity::<T>(parent_id, context_entity);
        }
    }

    /// Finds the closest matching context entity by traversing up the tree.
    pub fn get_context_entity<T: Default + 'static>(&self, current_entity: Entity) -> Option<Entity> {
        if let Ok(tree) = self.old_tree.read() {
            let mut parent = tree.get_parent(WrappedIndex(current_entity));
            while parent.is_some() {
                if let Some(entity) = self.context_entities.get_context_entity::<T>(parent.unwrap().0) {
                    return Some(entity);
                }
                parent = tree.get_parent(parent.unwrap());
            }
        }

        None
    }

    pub(crate) fn copy_from_point(&self, other_tree: &Arc<RwLock<Tree>>, entity: WrappedIndex) {
        if let Ok(other_tree) = other_tree.read() { 
            if let Ok(mut tree) = self.new_tree.write() {
                tree.copy_from_point(&other_tree, entity);
            }
        }
    }

    pub fn clear_children(&self, entity: Entity) {
        if let Ok(mut tree) = self.new_tree.write() {
            tree.children.insert(WrappedIndex(entity), vec![]);
        }
    }

    pub fn get_children(&self, entity: Entity) -> Vec<Entity> {
        let mut children = vec![];
        if let Ok(tree) = self.new_tree.read() {
            if let Some(existing_children) = tree.children.get(&WrappedIndex(entity)) {
                children = existing_children.iter().map(|index| index.0).collect::<Vec<_>>();
            }
        }

        children
    }

    pub fn remove_children(&self, children_to_remove: Vec<Entity>) {
        if let Ok(mut tree) = self.new_tree.write() {
            for child in children_to_remove.iter() {
                tree.remove(WrappedIndex(*child));
            }
        }
    }

    pub fn add(&self, entity: Entity, parent: Option<Entity>) {
        if let Ok(mut tree) = self.new_tree.write() {
            tree.add(WrappedIndex(entity), parent.map(|parent| WrappedIndex(parent)));
        }
    }

    pub fn dbg_tree(&self) {
        if let Ok(tree) = self.new_tree.read() {
            dbg!(&tree);
        }
    }

    pub fn take(self) -> Tree {
        Arc::try_unwrap(self.new_tree).unwrap().into_inner().unwrap()
    }
}
