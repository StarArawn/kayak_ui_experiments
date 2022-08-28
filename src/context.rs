use std::sync::Arc;
use bevy::{ecs::system::CommandQueue, prelude::*, utils::HashMap};

use crate::{widget::Widget, tree::{Index, WidgetTree, Tree, ChildChanges, Change, Hierarchy}};

pub struct Context {
    tree: Tree,
    widget_types: HashMap<Index, Arc<dyn Widget>>,
    systems: HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = bool>>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            tree: Tree::default(),
            widget_types: HashMap::default(),
            systems: HashMap::default(),
        }
    }

    pub fn register_widget_system<Params>(
        &mut self,
        type_name: impl Into<String>,
        system: impl IntoSystem<(WidgetTree, Entity), bool, Params>,
    ) {
        self.systems
            .insert(type_name.into(), Box::new(IntoSystem::into_system(system)));
    }

    pub fn add_widget<T: Widget + Default + 'static>(
        &mut self,
        parent: Option<Index>,
        entity: Entity,
    ) {
        self.tree.add(entity, parent);
        self.widget_types.insert(entity, Arc::new(T::default()));
    }
}


fn update_widgets_sys(world: &mut World) {
    let mut context = world
        .get_resource_mut::<Option<Context>>()
        .unwrap()
        .take()
        .unwrap();
    let tree_iterator = context.tree.down_iter().collect::<Vec<_>>();
    update_widgets(
        world,
        &mut context.tree,
        &mut context.systems,
        tree_iterator,
        &mut context.widget_types,
    );
    *world.get_resource_mut::<Option<Context>>().unwrap() = Some(context);
}

fn update_widgets(
    world: &mut World,
    tree: &mut Tree,
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = bool>>>,
    widgets: Vec<Entity>,
    parent_widget_types: &mut HashMap<Entity, Arc<dyn Widget>>,
) {
    for entity in widgets.iter() {
        let widget_type = parent_widget_types.get(entity).cloned();
        if let Some(widget_type) = widget_type {
            let (mut widget_tree, mut widget_types, diff, should_update_children) = update_widget(
                systems,
                tree,
                parent_widget_types,
                world,
                *entity,
                &widget_type,
            );

            if should_update_children {
                let children = widget_tree.child_iter(*entity).collect::<Vec<_>>();
                update_widgets(
                    world,
                    &mut widget_tree,
                    systems,
                    children,
                    &mut widget_types,
                );
            }

            tree.merge(&widget_tree, *entity, diff);
            parent_widget_types.extend(widget_types);
        }
    }
}

fn update_widget(
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = bool>>>,
    tree: &mut Tree,
    parent_widget_types: &mut HashMap<Entity, Arc<dyn Widget>>,
    world: &mut World,
    entity: Entity,
    widget_type: &Arc<dyn Widget>,
) -> (
    Tree,
    HashMap<Entity, Arc<dyn Widget>>,
    ChildChanges,
    bool,
) {
    let widget_tree = WidgetTree::new();
    let should_update_children;
    {
        let widget_system = systems.get_mut(widget_type.get_name()).unwrap();
        should_update_children = widget_system.run((widget_tree.clone(), entity), world);
        widget_system.apply_buffers(world);
    }
    let (widget_tree, widget_types) = widget_tree.take();
    let diff = tree.diff_children(&widget_tree, entity);
    let mut command_queue = CommandQueue::default();
    let mut commands = Commands::new(&mut command_queue, world);
    for (_, changed_entity, _, changes) in diff.changes.iter() {
        if changes
            .iter()
            .any(|change| *change == Change::Deleted)
        {
            commands.entity(*changed_entity).despawn();
            parent_widget_types.remove(changed_entity);
        }
    }
    command_queue.apply(world);

    (widget_tree, widget_types, diff, should_update_children)
}

fn init_systems(world: &mut World) {
    let mut context = world
        .get_resource_mut::<Option<Context>>()
        .unwrap()
        .take()
        .unwrap();
    for system in context.systems.values_mut() {
        system.initialize(world);
    }

    *world.get_resource_mut::<Option<Context>>().unwrap() = Some(context);
}

pub struct ContextPlugin;

impl Plugin for ContextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_systems.exclusive_system().at_end())
            .add_system(update_widgets_sys.exclusive_system());
    }
}