use bevy::prelude::{Bundle, Changed, Component, Entity, In, Or, ParamSet, Query, With};

use crate::{
    children::Children,
    context::{Mounted, WidgetName},
    layout::GeometryChanged,
    layout::LayoutEvent,
    on_layout::OnLayout,
    prelude::WidgetContext,
    styles::{LayoutType, RenderCommand, Style, Units},
    widget::Widget,
};

use super::scroll_context::ScrollContext;

#[derive(Component, Default)]
pub struct ScrollContentProps;

impl Widget for ScrollContentProps {}

#[derive(Bundle)]
pub struct ScrollContentBundle {
    pub scroll_content_props: ScrollContentProps,
    pub styles: Style,
    pub children: Children,
    pub on_layout: OnLayout,
    pub widget_name: WidgetName,
}

impl Default for ScrollContentBundle {
    fn default() -> Self {
        Self {
            scroll_content_props: Default::default(),
            styles: Default::default(),
            children: Default::default(),
            on_layout: Default::default(),
            widget_name: ScrollContentProps::default().get_name(),
        }
    }
}

pub fn update_scroll_content(
    In((widget_context, entity)): In<(WidgetContext, Entity)>,
    mut query: ParamSet<(
        Query<
            Entity,
            Or<(
                Changed<ScrollContentProps>,
                Changed<Children>,
                With<Mounted>,
            )>,
        >,
        Query<(&mut Style, &Children, &mut OnLayout), With<ScrollContentProps>>,
    )>,
    mut context_query: ParamSet<(Query<Entity, Changed<ScrollContext>>, Query<&ScrollContext>)>,
) -> bool {
    if !context_query.p0().is_empty() || !query.p0().is_empty() {
        if let Ok((mut styles, children, mut on_layout)) = query.p1().get_mut(entity) {
            if let Some(context_entity) = widget_context.get_context_entity::<ScrollContext>(entity)
            {
                if let Ok(scroll_context) = context_query.p1().get(context_entity) {
                    // === OnLayout === //
                    *on_layout = OnLayout::new(
                        move |In((event, _entity)): In<(LayoutEvent, Entity)>,
                              mut query: Query<&mut ScrollContext>| {
                            if event.flags.intersects(
                                GeometryChanged::WIDTH_CHANGED | GeometryChanged::HEIGHT_CHANGED,
                            ) {
                                if let Ok(mut scroll) = query.get_mut(context_entity) {
                                    scroll.content_width = event.layout.width;
                                    scroll.content_height = event.layout.height;
                                }
                            }

                            event
                        },
                    );

                    // === Styles === //
                    *styles = Style::default()
                        .with_style(Style {
                            render_command: RenderCommand::Layout.into(),
                            layout_type: LayoutType::Column.into(),
                            min_width: Units::Pixels(
                                scroll_context.scrollbox_width - scroll_context.pad_x,
                            )
                            .into(),
                            min_height: Units::Stretch(
                                scroll_context.scrollbox_height - scroll_context.pad_y,
                            )
                            .into(),
                            width: Units::Auto.into(),
                            height: Units::Auto.into(),
                            ..Default::default()
                        })
                        .with_style(styles.clone());

                    children.process(&widget_context, Some(entity));

                    return true;
                }
            }
        }
    }
    false
}
