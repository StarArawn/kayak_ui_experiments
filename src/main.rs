mod tree;

use std::sync::Arc;

use bevy::{ecs::system::CommandQueue, prelude::*, utils::HashMap};
use tree::{Hierarchy, WidgetTree};

#[derive(Component, Default)]
pub struct MyWidget2 {
    bar: u32,
}

fn my_widget_2_update(
    In((_widget_tree, entity)): In<(WidgetTree, Entity)>,
    query: Query<&MyWidget2, Or<(Added<MyWidget2>, Changed<MyWidget2>)>>,
) -> bool  {
    if let Ok(my_widget2) = query.get(entity) {
        dbg!(my_widget2.bar);
    }

    true
}

impl Widget for MyWidget2 {}

#[derive(Component, Default)]
pub struct MyWidget {
    pub foo: u32,
}

fn my_widget_1_update(
    In((widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    my_resource: Res<MyResource>,
    mut query: Query<&mut MyWidget>,
) -> bool {
    if my_resource.is_changed() {
        if let Ok(mut my_widget) = query.get_mut(entity) {
            my_widget.foo = my_resource.0;
            dbg!(my_widget.foo);

            let my_child = MyWidget2 {
                bar: my_widget.foo,
            };
            let should_update = my_widget.foo == my_child.bar;
            let child_id = commands.spawn().insert(my_child).id();
            widget_tree.add::<MyWidget2>(child_id, Some(entity));

            return should_update;
        }
    }

    false
}

impl Widget for MyWidget {}

pub trait Widget: Send + Sync {
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub struct MyResource(pub u32);

pub struct Context {
    tree: tree::Tree,
    widget_types: HashMap<tree::Index, Arc<dyn Widget>>,
    systems: HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = (bool)>>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            tree: tree::Tree::default(),
            widget_types: HashMap::default(),
            systems: HashMap::default(),
        }
    }

    pub fn register_widget_system<Params>(
        &mut self,
        type_name: impl Into<String>,
        system: impl IntoSystem<(WidgetTree, Entity), (bool), Params>,
    ) {
        self.systems
            .insert(type_name.into(), Box::new(IntoSystem::into_system(system)));
    }

    pub fn add_widget<T: Widget + Default + 'static>(
        &mut self,
        parent: Option<tree::Index>,
        entity: Entity,
    ) {
        self.tree.add(entity, parent);
        self.widget_types.insert(entity, Arc::new(T::default()));
    }
}

fn startup(mut commands: Commands) {
    let mut context = Context::new();
    context.register_widget_system(MyWidget::default().get_name(), my_widget_1_update);
    context.register_widget_system(MyWidget2::default().get_name(), my_widget_2_update);
    let entity = commands.spawn().insert(MyWidget { foo: 0 }).id();
    context.add_widget::<MyWidget>(None, entity);
    commands.insert_resource(Some(context));
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
    tree: &mut tree::Tree,
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = (bool)>>>,
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
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = (bool)>>>,
    tree: &mut tree::Tree,
    parent_widget_types: &mut HashMap<Entity, Arc<dyn Widget>>,
    world: &mut World,
    entity: Entity,
    widget_type: &Arc<dyn Widget>,
) -> (
    tree::Tree,
    HashMap<Entity, Arc<dyn Widget>>,
    tree::ChildChanges,
    bool,
) {
    let widget_tree = WidgetTree::new();
    let mut should_update_children = false;
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
            .any(|change| *change == tree::Change::Deleted)
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

fn update_resource(keyboard_input: Res<Input<KeyCode>>, mut my_resource: ResMut<MyResource>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        my_resource.0 += 1;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MyResource(1))
        .add_startup_system(startup.label("startup"))
        .add_startup_system(init_systems.exclusive_system().at_end())
        .add_system(update_widgets_sys.exclusive_system())
        .add_system(update_resource)
        .run()
}
