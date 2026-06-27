use glium::texture::TextureCreationError;
use log::warn;
use sge_camera::cameras_for_resolution;
use sge_color::Color;
use sge_math::transform::Transform2D;
use sge_programs::{
    BLINN_PHONG_3D_PROGRAM, FLAT_3D_PROGRAM, GOURAUD_3D_PROGRAM, TEXTURED_3D_PROGRAM,
};
use sge_textures::TextureRef;
use sge_types::{Area, ResizeMethod};
use sge_vectors::{Vec2, Vec3, vec2};
use sge_window::window_size;

use crate::{
    d2::{Collection2D, Renderer2D},
    dq2d, get_render_state,
    materials::{Material, MaterialRef},
    pipeline::{
        ClearColor, RenderPipeline, RenderTarget, RenderTextureRef, current_render_pipeline,
        draw_queue_2d, empty_render_texture, world_draw_queue_2d,
    },
    scissor::{pop_scissor, push_scissor},
    wdq2d,
};

pub fn draw_texture_to(texture: TextureRef, position: Vec2, scale: f32, renderer: Renderer2D) {
    draw_texture_scaled_to(
        texture,
        position,
        texture.normalized_dimensions * scale,
        renderer,
    );
}

pub fn draw_texture(texture: TextureRef, position: Vec2, scale: f32) {
    draw_texture_scaled(texture, position, texture.normalized_dimensions * scale);
}

pub fn draw_texture_scaled_to(
    texture: TextureRef,
    position: Vec2,
    scale: Vec2,
    mut renderer: Renderer2D,
) {
    renderer.add_texture(
        texture,
        Transform2D::from_scale_translation(scale, position),
        Color::WHITE,
        None,
    );
}

pub fn draw_texture_scaled(texture: TextureRef, position: Vec2, scale: Vec2) {
    draw_texture_scaled_to(texture, position, scale, dq2d());
}

pub fn draw_texture_world(texture: TextureRef, position: Vec2, scale: f32) {
    draw_texture_scaled_world(texture, position, texture.normalized_dimensions * scale);
}

pub fn draw_texture_scaled_world(texture: TextureRef, position: Vec2, scale: Vec2) {
    draw_texture_scaled_to(texture, position, scale, wdq2d());
}

pub fn draw_texture_world_ex(
    texture: TextureRef,
    transform: Transform2D,
    color: Color,
    region: Option<Area>,
) {
    world_draw_queue_2d()
        .renderer()
        .add_texture(texture, transform, color, region);
}

pub fn draw_texture_ex(
    texture: TextureRef,
    transform: Transform2D,
    color: Color,
    region: Option<Area>,
) {
    draw_queue_2d()
        .renderer()
        .add_texture(texture, transform, color, region);
}

pub fn draw_texture_to_ex(
    texture: TextureRef,
    transform: Transform2D,
    color: Color,
    region: Option<Area>,
    mut renderer: Renderer2D,
) {
    renderer.add_texture(texture, transform, color, region);
}

pub fn create_flat_material(color: Color) -> MaterialRef {
    let material = Material::new(FLAT_3D_PROGRAM).with_color("color", color);
    material.create()
}

pub fn create_gouraud_material(
    regular_color: Color,
    dark_color: Color,
    light_pos: Vec3,
) -> MaterialRef {
    let material = Material::new(GOURAUD_3D_PROGRAM)
        .with_color("regular_color", regular_color)
        .with_color("dark_color", dark_color)
        .with_vec3("light_pos", light_pos);

    material.create()
}

pub fn create_textured_material(texture: TextureRef) -> MaterialRef {
    let material = Material::new(TEXTURED_3D_PROGRAM).with_texture("tex", texture);
    material.create()
}

pub fn create_blinn_phong_material(
    ambient: Color,
    diffuse: Color,
    specular: Color,
    rim: Color,
    light_pos: Vec3,
) -> MaterialRef {
    let material = Material::new(BLINN_PHONG_3D_PROGRAM)
        .with_color("ambient_color", ambient)
        .with_color("diffuse_color", diffuse)
        .with_color("specular_color", specular)
        .with_color("rim_color", rim)
        .with_vec3("light_pos", light_pos);
    material.create()
}

pub fn start_rendering_to_texture(texture: RenderTextureRef) {
    let size = texture.dimensions();

    get_render_state().texture_pipeline = Some(RenderPipeline::new(
        RenderTarget::Texture(texture),
        Some(cameras_for_resolution(size.x, size.y)),
    ));
}

pub fn end_rendering_to_texture() {
    let state = get_render_state();

    match &mut state.texture_pipeline {
        Some(pipeline) => pipeline.draw(),
        None => warn!(
            "Called `end_rendering_to_texture` without any texture pipeline loaded. Create one with `start_rendering_to_texture`."
        ),
    }

    state.texture_pipeline = None;
}

pub fn create_empty_render_texture(
    width: u32,
    height: u32,
) -> Result<RenderTextureRef, TextureCreationError> {
    Ok(empty_render_texture(width, height)?.create())
}

pub fn clear_screen(color: Color) {
    current_render_pipeline().clear_color = ClearColor::Clear(color);
}

pub fn dont_clear_screen() {
    current_render_pipeline().clear_color = ClearColor::DontClear;
}

pub fn draw_fullscreen_texture(texture: TextureRef) {
    draw_texture_scaled(texture, Vec2::ZERO, window_size());
}

pub fn draw_collection(collection: &Collection2D) {
    dq2d().add_collection(collection);
}

pub fn draw_collection_world(collection: &Collection2D) {
    wdq2d().add_collection(collection);
}

pub fn draw_collection_to(collection: &Collection2D, mut renderer: Renderer2D) {
    renderer.add_collection(collection);
}

pub fn draw_nine_slice_to(
    texture: TextureRef,
    position: Vec2,
    size: Vec2,
    scale: Vec2,
    corner_size: u32,
    resize_method: ResizeMethod,
    renderer: Renderer2D,
) {
    let dim = texture.dimensions.as_vec2();
    let unit = Vec2::splat(corner_size as f32);
    let tl = Vec2::ZERO;
    let br = dim - unit;
    let tr = vec2(dim.x - unit.x, 0.0);
    let bl = vec2(tl.x, br.y);

    let unit_scaled = unit * scale;
    let br_scaled = position + size - unit * scale;

    // top left
    draw_texture_to_ex(
        texture,
        Transform2D::from_scale_translation(unit_scaled, position),
        Color::WHITE,
        Some(Area::new(tl, unit)),
        renderer,
    );

    // top right
    draw_texture_to_ex(
        texture,
        Transform2D::from_scale_translation(unit_scaled, vec2(br_scaled.x, position.y)),
        Color::WHITE,
        Some(Area::new(tr, unit)),
        renderer,
    );

    // bottom left
    draw_texture_to_ex(
        texture,
        Transform2D::from_scale_translation(unit_scaled, vec2(position.x, br_scaled.y)),
        Color::WHITE,
        Some(Area::new(bl, unit)),
        renderer,
    );

    // bottom right
    draw_texture_to_ex(
        texture,
        Transform2D::from_scale_translation(unit_scaled, br_scaled),
        Color::WHITE,
        Some(Area::new(br, unit)),
        renderer,
    );

    let inner_size = dim - unit - unit;
    let target_inner_size = size - unit_scaled - unit_scaled;

    match resize_method {
        ResizeMethod::Stretch => {
            // top border
            draw_texture_to_ex(
                texture,
                Transform2D::from_scale_translation(
                    vec2(target_inner_size.x, unit_scaled.y),
                    position + unit_scaled.with_y(0.0),
                ),
                Color::WHITE,
                Some(Area::from_corners(unit, tr)),
                renderer,
            );

            // bottom border
            draw_texture_to_ex(
                texture,
                Transform2D::from_scale_translation(
                    vec2(target_inner_size.x, unit_scaled.y),
                    position + vec2(unit_scaled.x, size.y - unit_scaled.y),
                ),
                Color::WHITE,
                Some(Area::from_corners(bl + unit, br)),
                renderer,
            );

            // left border
            draw_texture_to_ex(
                texture,
                Transform2D::from_scale_translation(
                    vec2(unit_scaled.x, target_inner_size.y),
                    position + unit_scaled.with_x(0.0),
                ),
                Color::WHITE,
                Some(Area::from_corners(unit, bl)),
                renderer,
            );

            // right border
            draw_texture_to_ex(
                texture,
                Transform2D::from_scale_translation(
                    vec2(unit_scaled.x, target_inner_size.y),
                    position + vec2(size.x - unit_scaled.x, unit_scaled.y),
                ),
                Color::WHITE,
                Some(Area::from_corners(br, vec2(dim.x, unit.y))),
                renderer,
            );

            // center
            draw_texture_to_ex(
                texture,
                Transform2D::from_scale_translation(target_inner_size, position + unit_scaled),
                Color::WHITE,
                Some(Area::from_corners(unit, br)),
                renderer,
            );
        }
        ResizeMethod::Tile => {
            // horizontal borders
            {
                let draw_width = inner_size.x * scale.x;
                let target_draw_width = target_inner_size.x;

                let mut cursor = 0.0;
                loop {
                    let final_segment = cursor + draw_width > target_draw_width;

                    let ratio = if final_segment {
                        (target_draw_width - cursor) / draw_width
                    } else {
                        1.0
                    };

                    let width = draw_width * ratio;

                    // top
                    draw_texture_to_ex(
                        texture,
                        Transform2D::from_scale_translation(
                            vec2(width, unit_scaled.y),
                            position + vec2(unit_scaled.x + cursor, 0.0),
                        ),
                        Color::WHITE,
                        Some(Area::new(
                            unit.with_y(0.0),
                            vec2(inner_size.x * ratio, unit.y),
                        )),
                        renderer,
                    );

                    // bottom
                    draw_texture_to_ex(
                        texture,
                        Transform2D::from_scale_translation(
                            vec2(width, unit_scaled.y),
                            position + vec2(unit_scaled.x + cursor, size.y - unit_scaled.y),
                        ),
                        Color::WHITE,
                        Some(Area::new(
                            vec2(unit.x, dim.y - unit.y),
                            vec2(inner_size.x * ratio, unit.y),
                        )),
                        renderer,
                    );

                    if final_segment {
                        break;
                    } else {
                        cursor += draw_width;
                    }
                }
            }

            // vertical borders
            {
                let draw_height = inner_size.y * scale.y;
                let target_draw_height = target_inner_size.y;

                let mut cursor = 0.0;
                loop {
                    let final_segment = cursor + draw_height > target_draw_height;

                    let ratio = if final_segment {
                        (target_draw_height - cursor) / draw_height
                    } else {
                        1.0
                    };

                    let height = draw_height * ratio;

                    // left
                    draw_texture_to_ex(
                        texture,
                        Transform2D::from_scale_translation(
                            vec2(unit_scaled.x, height),
                            position + vec2(0.0, unit_scaled.y + cursor),
                        ),
                        Color::WHITE,
                        Some(Area::new(
                            unit.with_x(0.0),
                            vec2(unit.x, inner_size.y * ratio),
                        )),
                        renderer,
                    );

                    // right
                    draw_texture_to_ex(
                        texture,
                        Transform2D::from_scale_translation(
                            vec2(unit_scaled.x, height),
                            position + vec2(size.x - unit_scaled.x, unit_scaled.y + cursor),
                        ),
                        Color::WHITE,
                        Some(Area::new(
                            vec2(dim.x - unit.x, unit.y),
                            vec2(unit.x, inner_size.y * ratio),
                        )),
                        renderer,
                    );

                    if final_segment {
                        break;
                    } else {
                        cursor += draw_height;
                    }
                }
            }

            // center
            {
                push_scissor(Area::new(
                    position + unit_scaled - vec2(1.0, 0.0),
                    target_inner_size + vec2(1.0, 0.0),
                ));

                let draw_size = inner_size * scale;
                let scale_factor = target_inner_size / draw_size;
                let grid_size = scale_factor.ceil().as_uvec2();

                for x in 0..grid_size.x {
                    for y in 0..grid_size.y {
                        draw_texture_to_ex(
                            texture,
                            Transform2D::from_scale_translation(
                                draw_size,
                                vec2(x as f32 * draw_size.x, y as f32 * draw_size.y)
                                    + position
                                    + unit_scaled,
                            ),
                            Color::WHITE,
                            Some(Area::new(unit, inner_size)),
                            renderer,
                        );
                    }
                }

                pop_scissor();
            }
        }
    }
}

pub fn draw_nine_slice(
    texture: TextureRef,
    position: Vec2,
    size: Vec2,
    scale: Vec2,
    corner_size: u32,
    resize_method: ResizeMethod,
) {
    draw_nine_slice_to(
        texture,
        position,
        size,
        scale,
        corner_size,
        resize_method,
        dq2d(),
    );
}

pub fn draw_nine_slice_world(
    texture: TextureRef,
    position: Vec2,
    size: Vec2,
    scale: Vec2,
    corner_size: u32,
    resize_method: ResizeMethod,
) {
    draw_nine_slice_to(
        texture,
        position,
        size,
        scale,
        corner_size,
        resize_method,
        wdq2d(),
    );
}
