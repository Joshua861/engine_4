use glium::texture::TextureCreationError;
use log::warn;
use sge_camera::cameras_for_resolution;
use sge_color::Color;
use sge_math::{collision::Aabb2d, transform::Transform2D};
use sge_programs::{
    BLINN_PHONG_3D_PROGRAM, FLAT_3D_PROGRAM, GOURAUD_3D_PROGRAM, TEXTURED_3D_PROGRAM,
};
use sge_textures::TextureRef;
use sge_vectors::{Vec2, Vec3};
use sge_window::window_size;

use crate::{
    d2::{Renderer2D, Scene2D},
    dq2d, get_render_state,
    materials::{Material, MaterialRef},
    pipeline::{
        ClearColor, RenderPipeline, RenderTarget, RenderTextureRef, current_render_pipeline,
        draw_queue_2d, empty_render_texture, world_draw_queue_2d,
    },
    wdq2d,
};

pub fn draw_texture(texture: TextureRef, position: Vec2, scale: f32) {
    draw_texture_scaled(texture, position, texture.normalized_dimensions * scale);
}

pub fn draw_texture_scaled(texture: TextureRef, position: Vec2, scale: Vec2) {
    draw_queue_2d().renderer().add_sprite(
        texture,
        Transform2D::from_scale_translation(scale, position),
        Color::WHITE,
        None,
    );
}

pub fn draw_texture_world(texture: TextureRef, position: Vec2, scale: f32) {
    draw_texture_scaled_world(texture, position, texture.normalized_dimensions * scale);
}

pub fn draw_texture_scaled_world(texture: TextureRef, position: Vec2, scale: Vec2) {
    let bounds = Aabb2d::new(position - scale, position + scale);

    if !bounds.is_visible_in_world() {
        return;
    }

    world_draw_queue_2d().renderer().add_sprite(
        texture,
        Transform2D::from_scale_translation(scale, position),
        Color::WHITE,
        None,
    );
}

pub fn draw_texture_world_ex(
    texture: TextureRef,
    transform: Transform2D,
    color: Color,
    region: Option<sge_vectors::Rect>,
) {
    let bounds = Aabb2d::new(
        transform.translation() - transform.scale(),
        transform.translation() + transform.scale(),
    );

    if !bounds.is_visible_in_world() {
        return;
    }

    world_draw_queue_2d()
        .renderer()
        .add_sprite(texture, transform, color, region);
}

pub fn draw_texture_ex(
    sprite: TextureRef,
    transform: Transform2D,
    color: Color,
    region: Option<sge_vectors::Rect>,
) {
    draw_queue_2d()
        .renderer()
        .add_sprite(sprite, transform, color, region);
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

pub fn draw_scene(scene: &Scene2D) {
    dq2d().add_scene(scene);
}

pub fn draw_scene_world(scene: &Scene2D) {
    wdq2d().add_scene(scene);
}

pub fn draw_scene_to(scene: &Scene2D, mut renderer: Renderer2D) {
    renderer.add_scene(scene);
}
