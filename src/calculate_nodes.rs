use bevy::{prelude::{Entity, Query, ResMut, With, Commands}, utils::HashMap};

use crate::{
    node::{DirtyNode, Node, NodeBuilder},
    prelude::{Context, Index, Style, RenderCommand, Tree},
    render_primitive::RenderPrimitive,
};

pub fn calculate_nodes(
    mut commands: Commands,
    mut context: ResMut<Option<Context>>,
    query: Query<Entity, With<DirtyNode>>,
    all_styles_query: Query<&Style>,
    node_query: Query<&Node>,
) {
    if context.is_none() {
        dbg!("Whoops no context!");
        return;
    }

    let mut context = (*context).as_mut().unwrap();

    let mut new_nodes = HashMap::<Entity, Node>::default();
    // This is the maximum recursion depth for this method.
    // Recursion involves recalculating layout which should be done sparingly.
    // const MAX_RECURSION_DEPTH: usize = 2;

    context.current_z = 0.0;

    let initial_styles = Style::initial();
    let default_styles = Style::new_default();

    for dirty_entity in query.iter() {
        if let Ok(styles) = all_styles_query.get(dirty_entity) {
            // Get the parent styles. Will be one of the following:
            // 1. Already-resolved node styles (best)
            // 2. Unresolved widget prop styles
            // 3. Unresolved default styles
            let parent_styles =
                if let Some(parent_widget_id) = context.tree.parents.get(&dirty_entity) {
                    if let Ok(parent_node) = node_query.get(*parent_widget_id) {
                        parent_node.resolved_styles.clone()
                    } else if let Some(parent_node) = new_nodes.get(parent_widget_id) {
                        parent_node.resolved_styles.clone()
                    } else if let Ok(parent_styles) = all_styles_query.get(*parent_widget_id) {
                        parent_styles.clone()
                    } else {
                        default_styles.clone()
                    }
                } else { default_styles.clone() };

            let parent_z = if let Some(parent_widget_id) = context.tree.parents.get(&dirty_entity) {
                if let Ok(parent_node) = node_query.get(*parent_widget_id) {
                    parent_node.z
                } else if let Some(parent_node) = new_nodes.get(parent_widget_id) {
                    parent_node.z
                } else {
                    -1.0
                }
            } else {
                -1.0
            };

            let current_z = {
                if parent_z > -1.0 {
                    parent_z + 1.0
                } else {
                    let z = context.current_z;
                    context.current_z += 1.0;
                    z
                }
            };

            let raw_styles = styles.clone();
            let mut styles = raw_styles.clone();
            // Fill in all `initial` values for any unset property
            styles.apply(&initial_styles);
            // Fill in all `inherited` values for any `inherit` property
            styles.inherit(&parent_styles);

            let primitive = create_primitive(dirty_entity, &mut styles);

            let children = context.tree.children.get(&dirty_entity).cloned().unwrap_or(vec![]);

            let mut node = NodeBuilder::empty()
                .with_id(dirty_entity)
                .with_styles(styles, Some(raw_styles))
                .with_children(children)
                .with_primitive(primitive)
                .build();
            node.z = current_z;
            dbg!(&node);
            new_nodes.insert(dirty_entity, node);
        }
    }
    for (entity, node) in new_nodes.drain() { 
        commands.entity(entity).insert(node).remove::<DirtyNode>();
    }


}

fn create_primitive(_id: Index, styles: &mut Style) -> RenderPrimitive {
    let render_primitive = RenderPrimitive::from(&styles.clone());
    // let mut needs_layout = false;

    // match &mut render_primitive {
    //     RenderPrimitive::Text {
    //         content,
    //         font,
    //         properties,
    //         text_layout,
    //         ..
    //     } => {
            // --- Bind to Font Asset --- //
            // let asset = assets.get_asset::<KayakFont, _>(font.clone());
            // self.bind(id, &asset);

            // if let Some(font) = asset.get() {
            //     if let Some(parent_id) = self.get_valid_parent(id) {
            //         if let Some(parent_layout) = self.get_layout(&parent_id) {
            //             properties.max_size = (parent_layout.width, parent_layout.height);

            //             // --- Calculate Text Layout --- //
            //             *text_layout = font.measure(&content, *properties);
            //             let measurement = text_layout.size();

            //             // --- Apply Layout --- //
            //             if matches!(styles.width, StyleProp::Default) {
            //                 styles.width = StyleProp::Value(Units::Pixels(measurement.0));
            //             }
            //             if matches!(styles.height, StyleProp::Default) {
            //                 styles.height = StyleProp::Value(Units::Pixels(measurement.1));
            //             }
            //         } else {
            //             needs_layout = true;
            //         }
            //     }
            // }
    //     }
    //     _ => {}
    // }

    // if needs_layout {
    //     needs_layout(id);
    // }

    render_primitive
}

pub fn build_nodes_tree(mut context: ResMut<Option<Context>>, node_query: Query<(Entity, &Node)>) {
    if context.is_none() {
        dbg!("Whoops no context!");
        return;
    }

    let mut context = (*context).as_mut().unwrap();
    
    let mut tree = Tree::default();
    tree.root_node = context.tree.root_node;
    tree.children.insert(
        tree.root_node.unwrap(),
        get_valid_node_children(&context, &node_query, tree.root_node.unwrap()),
    );

    // let old_focus = self.focus_tree.current();
    // self.focus_tree.clear();
    // self.focus_tree.add(root_node_id, &self.tree);

    for (node_id, node) in node_query.iter() {
        if let Some(widget_styles) = node.raw_styles.as_ref() {
            // Only add widgets who have renderable nodes.
            if widget_styles.render_command.resolve() != RenderCommand::Empty {
                let valid_children = get_valid_node_children(&context, &node_query, node_id);
                tree.children.insert(node_id, valid_children);
                let valid_parent = get_valid_parent(&context, &node_query, node_id);
                if let Some(valid_parent) = valid_parent {
                    tree.parents.insert(node_id, valid_parent);
                }
            }
        }

        // let focusable = self.get_focusable(widget_id).unwrap_or_default();
        // if focusable {
        //     self.focus_tree.add(widget_id, &self.tree);
        // }
    }

    // if let Some(old_focus) = old_focus {
    //     if self.focus_tree.contains(old_focus) {
    //         self.focus_tree.focus(old_focus);
    //     }
    // }

    context.node_tree = tree;
}

pub fn get_valid_node_children(context: &Context, query: &Query<(Entity, &Node)>, node_id: Index) -> Vec<Index> {
    let mut children = Vec::new();
    if let Some(node_children) = context.tree.children.get(&node_id) {
        for child_id in node_children {
            if let Ok((_, child_node)) = query.get(*child_id) {
                if child_node.resolved_styles.render_command.resolve() != RenderCommand::Empty {
                    children.push(*child_id);
                } else {
                    children.extend(get_valid_node_children(context, query, *child_id));
                }
            }
        }
    }

    children
}

pub fn get_valid_parent(context: &Context, query: &Query<(Entity, &Node)>, node_id: Index) -> Option<Index> {
    if let Some(parent_id) = context.tree.parents.get(&node_id) {
        if let Ok((_, parent_node)) = query.get(*parent_id) {
            if parent_node.resolved_styles.render_command.resolve() != RenderCommand::Empty {
                return Some(*parent_id);
            }
        }
        return get_valid_parent(context, query, *parent_id);
    }

    None
}