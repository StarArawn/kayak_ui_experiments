use bevy::{
    ecs::{event::ManualEventReader, system::CommandQueue},
    prelude::*,
    utils::HashMap,
};
use morphorm::Hierarchy;
use std::sync::Arc;

use crate::{
    calculate_nodes::calculate_nodes,
    event_dispatcher::EventDispatcher,
    focus_tree::FocusTree,
    layout::{LayoutCache, Rect},
    node::{DirtyNode, WrappedIndex},
    render_primitive::RenderPrimitive,
    tree::{Change, Tree, WidgetTree},
    widget::Widget,
    WindowSize,
};

#[derive(Resource)]
pub struct Context {
    pub(crate) tree: Tree,
    pub(crate) node_tree: Tree,
    pub(crate) layout_cache: LayoutCache,
    pub(crate) focus_tree: FocusTree,
    pub(crate) event_dispatcher: EventDispatcher,
    widget_types: HashMap<Entity, Arc<dyn Widget>>,
    systems: HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = bool>>>,
    pub(crate) current_z: f32,
}

impl Context {
    pub fn new() -> Self {
        Self {
            tree: Tree::default(),
            node_tree: Tree::default(),
            layout_cache: LayoutCache::default(),
            focus_tree: FocusTree::default(),
            event_dispatcher: EventDispatcher::new(),
            widget_types: HashMap::default(),
            systems: HashMap::default(),
            current_z: 0.0,
        }
    }

    pub(crate) fn get_layout(&self, id: &WrappedIndex) -> Option<&Rect> {
        self.layout_cache.rect.get(id)
    }

    pub fn add_widget_system<Params>(
        &mut self,
        type_name: impl Into<String>,
        system: impl IntoSystem<(WidgetTree, Entity), bool, Params>,
    ) {
        let system = IntoSystem::into_system(system);
        self.systems.insert(type_name.into(), Box::new(system));
    }

    pub fn add_widget<T: Widget + Default + 'static>(
        &mut self,
        parent: Option<WrappedIndex>,
        entity: Entity,
    ) {
        self.tree.add(WrappedIndex(entity), parent);
        self.layout_cache.add(WrappedIndex(entity));
        self.widget_types.insert(entity, Arc::new(T::default()));
    }

    pub fn build_render_primitives(
        &self,
        nodes: &Query<&crate::node::Node>,
    ) -> Vec<RenderPrimitive> {
        if self.node_tree.root_node.is_none() {
            return vec![];
        }

        recurse_node_tree_to_build_primitives(
            &self.node_tree,
            &self.layout_cache,
            nodes,
            self.node_tree.root_node.unwrap(),
            0.0,
            RenderPrimitive::Empty,
        )
    }
}

fn recurse_node_tree_to_build_primitives(
    node_tree: &Tree,
    layout_cache: &LayoutCache,
    nodes: &Query<&crate::node::Node>,
    current_node: WrappedIndex,
    mut main_z_index: f32,
    mut prev_clip: RenderPrimitive,
) -> Vec<RenderPrimitive> {
    let mut render_primitives = Vec::new();
    if let Ok(node) = nodes.get(current_node.0) {
        if let Some(layout) = layout_cache.rect.get(&current_node) {
            let mut render_primitive = node.primitive.clone();
            let mut layout = *layout;
            let new_z_index = if matches!(render_primitive, RenderPrimitive::Clip { .. }) {
                main_z_index - 0.1
            } else {
                main_z_index
            };
            layout.z_index = new_z_index;
            render_primitive.set_layout(layout);
            render_primitives.push(render_primitive.clone());

            let new_prev_clip = if matches!(render_primitive, RenderPrimitive::Clip { .. }) {
                render_primitive.clone()
            } else {
                prev_clip
            };

            prev_clip = new_prev_clip.clone();

            if node_tree.children.contains_key(&current_node) {
                for child in node_tree.children.get(&current_node).unwrap() {
                    main_z_index += 1.0;
                    render_primitives.extend(recurse_node_tree_to_build_primitives(
                        node_tree,
                        layout_cache,
                        nodes,
                        *child,
                        main_z_index,
                        new_prev_clip.clone(),
                    ));

                    main_z_index = layout.z_index;
                    // Between each child node we need to reset the clip.
                    if matches!(prev_clip, RenderPrimitive::Clip { .. }) {
                        // main_z_index = new_z_index;
                        match &mut prev_clip {
                            RenderPrimitive::Clip { layout } => {
                                layout.z_index = main_z_index + 0.1;
                            }
                            _ => {}
                        };
                        render_primitives.push(prev_clip.clone());
                    }
                }
            }
        }
    }

    render_primitives
}

fn update_widgets_sys(world: &mut World) {
    let mut context = world.remove_resource::<Context>().unwrap();
    let tree_iterator = context.tree.down_iter().collect::<Vec<_>>();

    // let change_tick = world.increment_change_tick();

    update_widgets(
        world,
        &mut context.tree,
        &mut context.layout_cache,
        &mut context.systems,
        tree_iterator,
        &mut context.widget_types,
    );

    for system in context.systems.values_mut() {
        system.set_last_change_tick(world.read_change_tick());
        // system.apply_buffers(world);
    }
    world.insert_resource(context);
}

fn update_widgets(
    world: &mut World,
    tree: &mut Tree,
    layout_cache: &mut LayoutCache,
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = bool>>>,
    widgets: Vec<WrappedIndex>,
    parent_widget_types: &mut HashMap<Entity, Arc<dyn Widget>>,
) {
    for entity in widgets.iter() {
        let widget_type = parent_widget_types.get(&entity.0).cloned();
        if let Some(widget_type) = widget_type {
            let widget_tree = WidgetTree::new();
            widget_tree.copy_from_point(&tree, *entity);
            let children_before = widget_tree.get_children(entity.0);
            let (widget_tree, mut widget_types, should_update_children) = update_widget(
                systems,
                tree,
                world,
                *entity,
                &widget_type,
                widget_tree,
                children_before,
            );

            // Only merge tree if changes detected.
            if should_update_children {
                let diff = tree.diff_children(&widget_tree, *entity);
                for (_index, child, _parent, changes) in diff.changes.iter() {
                    for change in changes.iter() {
                        if matches!(change, Change::Inserted) {
                            layout_cache.add(*child);
                        }
                    }
                }
                tree.merge(&widget_tree, *entity, diff);
            }

            // if should_update_children {
            let children = widget_tree.child_iter(*entity).collect::<Vec<_>>();
            update_widgets(
                world,
                tree,
                layout_cache,
                systems,
                children,
                &mut widget_types,
            );
            // }
            parent_widget_types.extend(widget_types);
        }
    }
}

fn update_widget(
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetTree, Entity), Out = bool>>>,
    tree: &mut Tree,
    world: &mut World,
    entity: WrappedIndex,
    widget_type: &Arc<dyn Widget>,
    widget_tree: WidgetTree,
    previous_children: Vec<Entity>,
) -> (Tree, HashMap<Entity, Arc<dyn Widget>>, bool) {
    let should_update_children;
    {
        let widget_system = systems.get_mut(widget_type.get_name()).unwrap();
        let old_tick = widget_system.get_last_change_tick();
        should_update_children = widget_system.run((widget_tree.clone(), entity.0), world);
        widget_system.set_last_change_tick(old_tick);
        widget_system.apply_buffers(world);
    }
    let (mut widget_tree, widget_types) = widget_tree.take();
    let mut command_queue = CommandQueue::default();
    let mut commands = Commands::new(&mut command_queue, world);

    // Mark node as needing a recalculation of rendering/layout.
    if should_update_children {
        // Remove children from previous render.
        widget_tree.remove_children(
            previous_children
                .iter()
                .map(|entity| WrappedIndex(*entity))
                .collect::<Vec<_>>(),
        );
        commands.entity(entity.0).insert(DirtyNode);
    }

    let diff = tree.diff_children(&widget_tree, entity);

    for (_, changed_entity, _, changes) in diff.changes.iter() {
        if changes.iter().any(|change| *change == Change::Deleted) && should_update_children {
            commands.entity(changed_entity.0).despawn();
        } else if should_update_children {
            commands.entity(changed_entity.0).insert(DirtyNode);
        }
    }
    command_queue.apply(world);

    (widget_tree, widget_types, should_update_children)
}

fn init_systems(world: &mut World) {
    let mut context = world.remove_resource::<Context>().unwrap();
    for system in context.systems.values_mut() {
        system.initialize(world);
    }

    world.insert_resource(context);
}

pub struct ContextPlugin;

#[derive(Resource)]
pub struct CustomEventReader<T: bevy::ecs::event::Event>(pub ManualEventReader<T>);

impl Plugin for ContextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowSize::default())
            .insert_resource(EventDispatcher::new())
            .insert_resource(CustomEventReader(ManualEventReader::<
                bevy::window::CursorMoved,
            >::default()))
            .insert_resource(CustomEventReader(ManualEventReader::<
                bevy::input::mouse::MouseButtonInput,
            >::default()))
            .insert_resource(CustomEventReader(ManualEventReader::<
                bevy::input::mouse::MouseWheel,
            >::default()))
            .insert_resource(CustomEventReader(ManualEventReader::<
                bevy::window::ReceivedCharacter,
            >::default()))
            .insert_resource(CustomEventReader(ManualEventReader::<
                bevy::input::keyboard::KeyboardInput,
            >::default()))
            .add_plugin(crate::camera::KayakUICameraPlugin)
            .add_plugin(crate::render::BevyKayakUIRenderPlugin)
            .register_type::<Node>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                init_systems.exclusive_system().at_end(),
            )
            .add_system_to_stage(
                CoreStage::Update,
                crate::input::process_events.exclusive_system(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_widgets_sys.exclusive_system().at_start(),
            )
            .add_system_to_stage(CoreStage::PostUpdate, calculate_ui.exclusive_system())
            .add_system(crate::window_size::update_window_size);
    }
}

fn calculate_ui(world: &mut World) {
    let mut system = IntoSystem::into_system(calculate_nodes);
    system.initialize(world);

    for _ in 0..5 {
        system.run((), world);
        system.apply_buffers(world);
    }
}
