use bevy::prelude::{Component, Entity, Query};

use crate::{
    render_primitive::RenderPrimitive,
    styles::{Style, StyleProp},
};

#[derive(Component, Debug, Clone, Copy)]
pub struct DirtyNode;

/// A widget node used for building the layout tree
#[derive(Debug, Clone, PartialEq, Component)]
pub struct Node {
    /// The list of children directly under this node
    pub children: Vec<WrappedIndex>,
    /// The ID of this node's widget
    pub id: WrappedIndex,
    /// The fully resolved styles for this node
    pub resolved_styles: Style,
    /// The raw styles for this node, before style resolution
    pub raw_styles: Option<Style>,
    /// The generated [`RenderPrimitive`] of this node
    pub primitive: RenderPrimitive,
    /// The z-index of this node, used for controlling layering
    pub z: f32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: Default::default(),
            id: WrappedIndex(Entity::from_raw(0)),
            resolved_styles: Default::default(),
            raw_styles: Default::default(),
            primitive: RenderPrimitive::Empty,
            z: Default::default(),
        }
    }
}

/// A struct used for building a [`Node`]
pub struct NodeBuilder {
    node: Node,
}

impl NodeBuilder {
    /// Defines a basic node without children, styles, etc.
    pub fn empty() -> Self {
        Self {
            node: Node {
                children: Vec::new(),
                id: WrappedIndex(Entity::from_raw(0)),
                resolved_styles: Style::default(),
                raw_styles: None,
                primitive: RenderPrimitive::Empty,
                z: 0.0,
            },
        }
    }

    /// Defines a node with the given id and styles
    pub fn new(id: WrappedIndex, styles: Style) -> Self {
        Self {
            node: Node {
                children: Vec::new(),
                id,
                resolved_styles: styles,
                raw_styles: None,
                primitive: RenderPrimitive::Empty,
                z: 0.0,
            },
        }
    }

    /// Sets the ID of the node being built
    pub fn with_id(mut self, id: WrappedIndex) -> Self {
        self.node.id = id;
        self
    }

    /// Sets the children of the node being built
    pub fn with_children(mut self, children: Vec<WrappedIndex>) -> Self {
        self.node.children.extend(children);
        self
    }

    /// Sets the resolved and raw styles, respectively, of the node being built
    pub fn with_styles(mut self, resolved_styles: Style, raw_styles: Option<Style>) -> Self {
        self.node.resolved_styles = resolved_styles;
        self.node.raw_styles = raw_styles;
        self
    }

    /// Sets the [`RenderPrimitive`] of the node being built
    pub fn with_primitive(mut self, primitive: RenderPrimitive) -> Self {
        self.node.primitive = primitive;
        self
    }

    /// Completes and builds the actual [`Node`]
    pub fn build(self) -> Node {
        self.node
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct WrappedIndex(pub Entity);

impl<'a> morphorm::Node<'a> for WrappedIndex {
    type Data = Query<'a, 'a, &'static Node>;

    fn layout_type(&self, store: &'_ Self::Data) -> Option<morphorm::LayoutType> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.layout_type {
                StyleProp::Default => Some(morphorm::LayoutType::default()),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::LayoutType::default()),
            };
        }
        return Some(morphorm::LayoutType::default());
    }

    fn position_type(&self, store: &'_ Self::Data) -> Option<morphorm::PositionType> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.position_type {
                StyleProp::Default => Some(morphorm::PositionType::default()),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::PositionType::default()),
            };
        }
        return Some(morphorm::PositionType::default());
    }

    fn width(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.width {
                StyleProp::Default => Some(morphorm::Units::Stretch(1.0)),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Stretch(1.0)),
            };
        }
        return Some(morphorm::Units::Stretch(1.0));
    }

    fn height(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.height {
                StyleProp::Default => Some(morphorm::Units::Stretch(1.0)),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Stretch(1.0)),
            };
        }
        return Some(morphorm::Units::Stretch(1.0));
    }

    fn min_width(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.min_width {
                StyleProp::Default => Some(morphorm::Units::Pixels(0.0)),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn min_height(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.min_height {
                StyleProp::Default => Some(morphorm::Units::Pixels(0.0)),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn max_width(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.max_width {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn max_height(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.max_height {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.left {
                StyleProp::Default => match node.resolved_styles.offset {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.left),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.right {
                StyleProp::Default => match node.resolved_styles.offset {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.right),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.top {
                StyleProp::Default => match node.resolved_styles.offset {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.top),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.bottom {
                StyleProp::Default => match node.resolved_styles.offset {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.bottom),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn min_left(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_left(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn min_right(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_right(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn min_top(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_top(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn min_bottom(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_bottom(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn child_left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.padding_left {
                StyleProp::Default => match node.resolved_styles.padding {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.left),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn child_right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.padding_right {
                StyleProp::Default => match node.resolved_styles.padding {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.right),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn child_top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.padding_top {
                StyleProp::Default => match node.resolved_styles.padding {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.top),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn child_bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.padding_bottom {
                StyleProp::Default => match node.resolved_styles.padding {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop.bottom),
                    _ => Some(morphorm::Units::Auto),
                },
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        return Some(morphorm::Units::Auto);
    }

    fn row_between(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.row_between {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn col_between(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.col_between {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(prop),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn grid_rows(&self, _store: &'_ Self::Data) -> Option<Vec<morphorm::Units>> {
        Some(vec![])
    }

    fn grid_cols(&self, _store: &'_ Self::Data) -> Option<Vec<morphorm::Units>> {
        Some(vec![])
    }

    fn row_index(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(0)
    }

    fn col_index(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(0)
    }

    fn row_span(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(1)
    }

    fn col_span(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(1)
    }

    fn border_left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.border {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(morphorm::Units::Pixels(prop.left)),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn border_right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.border {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(morphorm::Units::Pixels(prop.right)),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn border_top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.border {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(morphorm::Units::Pixels(prop.top)),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }

    fn border_bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Ok(node) = store.get(self.0) {
            return match node.resolved_styles.border {
                StyleProp::Default => Some(morphorm::Units::Auto),
                StyleProp::Value(prop) => Some(morphorm::Units::Pixels(prop.bottom)),
                _ => Some(morphorm::Units::Auto),
            };
        }
        Some(morphorm::Units::Auto)
    }
}
