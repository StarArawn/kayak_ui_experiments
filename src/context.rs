use std::sync::{Arc, RwLock};

use bevy::{
    ecs::{event::ManualEventReader, system::CommandQueue},
    prelude::*,
    utils::HashMap,
};
use morphorm::Hierarchy;

use crate::{
    calculate_nodes::calculate_nodes,
    context_entities::ContextEntities,
    event_dispatcher::EventDispatcher,
    focus_tree::FocusTree,
    layout::{LayoutCache, Rect},
    layout_dispatcher::LayoutEventDispatcher,
    node::{DirtyNode, WrappedIndex},
    prelude::WidgetContext,
    render_primitive::RenderPrimitive,
    tree::{Change, Tree},
    WindowSize,
};

/// A tag component representing when a widget has been mounted(added to the tree).
#[derive(Component)]
pub struct Mounted;

const UPDATE_DEPTH: u32 = 0;

#[derive(Resource)]
pub struct Context {
    pub tree: Arc<RwLock<Tree>>,
    pub(crate) layout_cache: Arc<RwLock<LayoutCache>>,
    pub(crate) focus_tree: FocusTree,
    systems: HashMap<String, Box<dyn System<In = (WidgetContext, Entity), Out = bool>>>,
    pub(crate) current_z: f32,
    context_entities: ContextEntities,
}

impl Context {
    pub fn new() -> Self {
        Self {
            tree: Arc::new(RwLock::new(Tree::default())),
            layout_cache: Arc::new(RwLock::new(LayoutCache::default())),
            focus_tree: FocusTree::default(),
            systems: HashMap::default(),
            current_z: 0.0,
            context_entities: ContextEntities::new(),
        }
    }

    pub(crate) fn get_layout(&self, id: &WrappedIndex) -> Option<Rect> {
        if let Ok(cache) = self.layout_cache.try_read() {
            cache.rect.get(id).cloned()
        } else {
            None
        }
    }

    pub fn add_widget_system<Params>(
        &mut self,
        type_name: impl Into<String>,
        system: impl IntoSystem<(WidgetContext, Entity), bool, Params>,
    ) {
        let system = IntoSystem::into_system(system);
        self.systems.insert(type_name.into(), Box::new(system));
    }

    pub fn add_widget(&mut self, parent: Option<Entity>, entity: Entity) {
        if let Ok(mut tree) = self.tree.write() {
            tree.add(
                WrappedIndex(entity),
                parent.and_then(|p| Some(WrappedIndex(p))),
            );
            if let Ok(mut cache) = self.layout_cache.try_write() {
                cache.add(WrappedIndex(entity));
            }
        }
    }

    /// Creates a new context using the context entity for the given type_id + parent id.
    pub fn set_context_entity<T: Default + 'static>(
        &self,
        parent_id: Option<Entity>,
        context_entity: Entity,
    ) {
        if let Some(parent_id) = parent_id {
            self.context_entities
                .add_context_entity::<T>(parent_id, context_entity);
        }
    }

    pub fn get_child_at(&self, entity: Option<Entity>) -> Option<Entity> {
        if let Ok(tree) = self.tree.try_read() {
            if let Some(entity) = entity {
                let children = tree.child_iter(WrappedIndex(entity)).collect::<Vec<_>>();
                return children.get(0).cloned().map(|index| index.0);
            }
        }
        None
    }

    pub fn build_render_primitives(
        &self,
        nodes: &Query<&crate::node::Node>,
    ) -> Vec<RenderPrimitive> {
        let node_tree = self.tree.try_read();
        if node_tree.is_err() {
            return vec![];
        }

        let node_tree = node_tree.unwrap();

        if node_tree.root_node.is_none() {
            return vec![];
        }

        // self.node_tree.dump();

        recurse_node_tree_to_build_primitives(
            &*node_tree,
            &self.layout_cache,
            nodes,
            node_tree.root_node.unwrap(),
            0.0,
            RenderPrimitive::Empty,
        )
    }
}

fn recurse_node_tree_to_build_primitives(
    node_tree: &Tree,
    layout_cache: &Arc<RwLock<LayoutCache>>,
    nodes: &Query<&crate::node::Node>,
    current_node: WrappedIndex,
    mut main_z_index: f32,
    mut prev_clip: RenderPrimitive,
) -> Vec<RenderPrimitive> {
    let mut render_primitives = Vec::new();
    if let Ok(node) = nodes.get(current_node.0) {
        if let Ok(cache) = layout_cache.try_read() {
            if let Some(layout) = cache.rect.get(&current_node) {
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
            } else {
                println!("No layout for: {:?}", current_node.0.id());
            }
        }
    } else {
        println!("No node for: {:?}", current_node.0.id());
    }

    render_primitives
}

fn update_widgets_sys(world: &mut World) {
    let mut context = world.remove_resource::<Context>().unwrap();
    let tree_iterator = if let Ok(tree) = context.tree.read() {
        tree.down_iter().collect::<Vec<_>>()
    } else {
        panic!("Failed to acquire read lock.");
    };

    // let change_tick = world.increment_change_tick();

    // dbg!("Updating widgets!");
    update_widgets(
        world,
        &context.tree,
        &context.layout_cache,
        &mut context.systems,
        tree_iterator,
        &context.context_entities,
    );
    // dbg!("Finished updating widgets!");

    for system in context.systems.values_mut() {
        system.set_last_change_tick(world.read_change_tick());
        // system.apply_buffers(world);
    }

    // if let Ok(tree) = context.tree.try_read() {
    // tree.dump();
    // }

    world.insert_resource(context);
}

fn update_widgets(
    world: &mut World,
    tree: &Arc<RwLock<Tree>>,
    layout_cache: &Arc<RwLock<LayoutCache>>,
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetContext, Entity), Out = bool>>>,
    widgets: Vec<WrappedIndex>,
    context_entities: &ContextEntities,
) {
    for entity in widgets.iter() {
        if let Some(entity_ref) = world.get_entity(entity.0) {
            if let Some(widget_type) = entity_ref.get::<WidgetName>() {
                let widget_context = WidgetContext::new(
                    tree.clone(),
                    context_entities.clone(),
                    layout_cache.clone(),
                );
                widget_context.copy_from_point(&tree, *entity);
                let children_before = widget_context.get_children(entity.0);
                let (widget_context, should_update_children) = update_widget(
                    systems,
                    tree,
                    world,
                    *entity,
                    widget_type.0,
                    widget_context,
                    children_before,
                );

                // Only merge tree if changes detected.
                if should_update_children {
                    if let Ok(mut tree) = tree.write() {
                        let diff = tree.diff_children(&widget_context, *entity, UPDATE_DEPTH);
                        for (_index, child, _parent, changes) in diff.changes.iter() {
                            for change in changes.iter() {
                                if matches!(change, Change::Inserted) {
                                    if let Ok(mut cache) = layout_cache.try_write() {
                                        cache.add(*child);
                                    }
                                }
                            }
                        }
                        // dbg!("Dumping widget tree:");
                        // widget_context.dump_at(*entity);
                        // dbg!(entity.0, &diff);

                        tree.merge(&widget_context, *entity, diff, UPDATE_DEPTH);
                        // dbg!(tree.dump_at(*entity));
                    }
                }

                // if should_update_children {
                let children = widget_context.child_iter(*entity).collect::<Vec<_>>();
                update_widgets(
                    world,
                    tree,
                    layout_cache,
                    systems,
                    children,
                    context_entities,
                );
                // }
            }
        }
    }
}

fn update_widget(
    systems: &mut HashMap<String, Box<dyn System<In = (WidgetContext, Entity), Out = bool>>>,
    tree: &Arc<RwLock<Tree>>,
    world: &mut World,
    entity: WrappedIndex,
    widget_type: &'static str,
    widget_context: WidgetContext,
    previous_children: Vec<Entity>,
) -> (Tree, bool) {
    let should_update_children;
    {
        // Remove children from previous render.
        widget_context.remove_children(previous_children);
        let widget_system = systems.get_mut(widget_type).unwrap();
        let old_tick = widget_system.get_last_change_tick();
        should_update_children = widget_system.run((widget_context.clone(), entity.0), world);
        widget_system.set_last_change_tick(old_tick);
        widget_system.apply_buffers(world);
    }
    let widget_context = widget_context.take();
    let mut command_queue = CommandQueue::default();
    let mut commands = Commands::new(&mut command_queue, world);

    commands.entity(entity.0).remove::<Mounted>();

    // Mark node as needing a recalculation of rendering/layout.
    if should_update_children {
        commands.entity(entity.0).insert(DirtyNode);
    }

    let diff = if let Ok(tree) = tree.read() {
        tree.diff_children(&widget_context, entity, UPDATE_DEPTH)
    } else {
        panic!("Failed to acquire read lock.");
    };
    if should_update_children {
        for (_, changed_entity, _, changes) in diff.changes.iter() {
            if changes.iter().any(|change| *change != Change::Deleted) {
                commands.entity(changed_entity.0).insert(DirtyNode);
            }

            if changes.iter().any(|change| *change == Change::Deleted) {
                // commands.entity(changed_entity.0).despawn();
                commands.entity(changed_entity.0).remove::<DirtyNode>();
            }
            if changes.iter().any(|change| *change == Change::Inserted) {
                commands.entity(changed_entity.0).insert(Mounted);
            }
        }
    }
    command_queue.apply(world);

    (widget_context, should_update_children)
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
            .add_system_to_stage(
                CoreStage::PostUpdate,
                calculate_ui.exclusive_system().at_end(),
            )
            .add_system(crate::window_size::update_window_size);
    }
}

fn calculate_ui(world: &mut World) {
    // dbg!("Calculating nodes!");
    let mut system = IntoSystem::into_system(calculate_nodes);
    system.initialize(world);

    for _ in 0..5 {
        system.run((), world);
        system.apply_buffers(world);
        world.resource_scope::<Context, _>(|world, mut context| {
            LayoutEventDispatcher::dispatch(&mut context, world);
        });
    }

    // dbg!("Finished calculating nodes!");

    // dbg!("Dispatching layout events!");
    // dbg!("Finished dispatching layout events!");
}

#[derive(Component, Debug)]
pub struct WidgetName(pub &'static str);
