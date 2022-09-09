use bevy::prelude::*;
use kayak_font::Alignment;

use crate::{styles::{Style, StyleProp, RenderCommand}, prelude::WidgetContext, widget::Widget, context::WidgetName};

#[derive(Component)]
pub struct Text {
    /// The string to display
    pub content: String,
    /// The name of the font to use
    ///
    /// The given font must already be loaded into the [`KayakContext`](kayak_core::KayakContext)
    pub font: Option<String>,
    /// The height of a line of text (currently in pixels)
    pub line_height: Option<f32>,
    /// If true, displays the default text cursor when hovered.
    ///
    /// This _will_ override the `cursor` style.
    pub show_cursor: bool,
    /// The font size (in pixels)
    ///
    /// Negative values have no effect
    pub size: f32,
    /// Text alignment.
    pub alignment: Alignment,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            content: String::new(),
            font: None,
            line_height: None,
            show_cursor: false,
            size: -1.0,
            alignment: Alignment::Start,
        }
    }
}

impl Widget for Text {}

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub styles: Style,
    pub widget_name: WidgetName,
}

impl Default for TextBundle {
    fn default() -> Self {
        Self { text: Default::default(), styles: Default::default(), widget_name: WidgetName(Text::default().get_name()) }
    }
}

pub fn text_update(
    In((_, entity)): In<(WidgetContext, Entity)>,
    mut query: Query<(&mut Style, &Text), Changed<Text>>,
) -> bool {

    if let Ok((mut style, text)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Text {
            content: text.content.clone(),
            alignment: text.alignment,
        });

        if let Some(ref font) = text.font {
            style.font = StyleProp::Value(font.clone());
        }
        // if text.show_cursor {
            // style.cursor = StyleProp::Value(CursorIcon::Text);
        // }
        if text.size >= 0.0 {
            style.font_size = StyleProp::Value(text.size);
        }
        if let Some(line_height) = text.line_height {
            style.line_height = StyleProp::Value(line_height);
        }

        // style.cursor = CursorIcon::Hand.into();

        return true;
    }

    false
}