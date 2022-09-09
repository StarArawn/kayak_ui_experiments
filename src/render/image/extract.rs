use bevy::{math::Vec2, render::color::Color, prelude::Rect};
use crate::{render_primitive::RenderPrimitive, render::unified::pipeline::{ExtractQuadBundle, ExtractedQuad, UIQuadType}, styles::Corner};

pub fn extract_images(
    render_command: &RenderPrimitive,
    dpi: f32,
) -> Vec<ExtractQuadBundle> {
    let (border_radius, layout, handle) = match render_command {
        RenderPrimitive::Image {
            border_radius,
            layout,
            handle,
        } => (*border_radius, layout, handle),
        _ => panic!(""),
    };

    vec![ExtractQuadBundle {
        extracted_quad: ExtractedQuad {
            rect: Rect {
                min: Vec2::new(layout.posx, layout.posy) * dpi,
                max: Vec2::new(layout.posx + layout.width, layout.posy + layout.height) * dpi,
            },
            color: Color::WHITE,
            vertex_index: 0,
            char_id: 0,
            z_index: layout.z_index,
            font_handle: None,
            quad_type: UIQuadType::Image,
            type_index: 0,
            border_radius: Corner {
                top_left: border_radius.top_left,
                top_right: border_radius.top_right,
                bottom_left: border_radius.bottom_left,
                bottom_right: border_radius.bottom_right,
            },
            image: Some(handle.clone_weak()),
            uv_max: None,
            uv_min: None,
        },
    }]
}
