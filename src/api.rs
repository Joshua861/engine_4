use std::any::Any;

use crate::{
    camera::Camera3D,
    collisions::AABB2D,
    post_processing::PostProcessingEffect,
    prelude::{FontRef, avg_fps, draw_text},
    render_pipeline::{RenderTexture, RenderTextureRef},
    shapes_2d::*,
    textures::EngineTexture,
};
use bevy_math::{UVec2, Vec2};
use egui_glium::egui_winit::egui::Context;
use glium::{
    Texture2d,
    texture::DepthTexture2d,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
};
use rand::{
    Rng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
};
use tunes::engine::AudioEngine;

use crate::{camera::Camera2D, color::Color, get_state, textures::TextureRef};

// always sets color alpha to 1.0 to stop some buggyness
pub fn clear_screen(color: Color) {
    get_state().current_render_pipeline().clear_color = Some(color.with_alpha(1.0));
}

pub fn draw_tri_outline(a: Vec2, b: Vec2, c: Vec2, thickness: f32, color: Color) {
    draw_line(a, b, thickness, color);
    draw_line(b, c, thickness, color);
    draw_line(c, a, thickness, color);

    let radius = thickness / 2.0;
    draw_circle(a, radius, color);
    draw_circle(b, radius, color);
    draw_circle(c, radius, color);
}

pub fn draw_tri_outline_world(a: Vec2, b: Vec2, c: Vec2, thickness: f32, color: Color) {
    draw_line_world(a, b, thickness, color);
    draw_line_world(b, c, thickness, color);
    draw_line_world(c, a, thickness, color);

    let radius = thickness / 2.0;
    draw_circle_world(a, radius, color);
    draw_circle_world(b, radius, color);
    draw_circle_world(c, radius, color);
}

pub fn draw_rect_outline(top_left: Vec2, size: Vec2, thickness: f32, color: Color) {
    let half_thick = thickness / 2.0;
    let top_right = top_left + Vec2::new(size.x, 0.0);
    let bottom_left = top_left + Vec2::new(0.0, size.y);
    let bottom_right = top_left + size;

    draw_line(
        top_left - Vec2::new(half_thick, 0.0),
        top_right + Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line(
        top_right + Vec2::new(0.0, -half_thick),
        bottom_right + Vec2::new(0.0, half_thick),
        thickness,
        color,
    );
    draw_line(
        bottom_right + Vec2::new(half_thick, 0.0),
        bottom_left - Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line(
        bottom_left + Vec2::new(0.0, half_thick),
        top_left - Vec2::new(0.0, -half_thick),
        thickness,
        color,
    );
}

pub fn draw_rect_outline_world(top_left: Vec2, size: Vec2, thickness: f32, color: Color) {
    let half_thick = thickness / 2.0;
    let top_right = top_left + Vec2::new(size.x, 0.0);
    let bottom_left = top_left + Vec2::new(0.0, size.y);
    let bottom_right = top_left + size;

    draw_line_world(
        top_left - Vec2::new(half_thick, 0.0),
        top_right + Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line_world(
        top_right + Vec2::new(0.0, -half_thick),
        bottom_right + Vec2::new(0.0, half_thick),
        thickness,
        color,
    );
    draw_line_world(
        bottom_right + Vec2::new(half_thick, 0.0),
        bottom_left - Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line_world(
        bottom_left + Vec2::new(0.0, half_thick),
        top_left - Vec2::new(0.0, -half_thick),
        thickness,
        color,
    );
}

pub fn draw_square_outline(top_left: Vec2, size: f32, thickness: f32, color: Color) {
    draw_rect_outline(top_left, Vec2::splat(size), thickness, color);
}

pub fn draw_square_outline_world(top_left: Vec2, size: f32, thickness: f32, color: Color) {
    draw_rect_outline_world(top_left, Vec2::splat(size), thickness, color);
}

pub fn draw_poly_outline(
    center: Vec2,
    sides: usize,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
) {
    let poly = Poly {
        sides,
        radius,
        center,
        rotation,
        color,
    };
    let points = poly.gen_points();
    let half_thick = thickness / 2.0;

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        let dir = (end - start).normalize();

        draw_line(
            start - dir * half_thick,
            end + dir * half_thick,
            thickness,
            color,
        );
    }
}

pub fn draw_poly_outline_world(
    center: Vec2,
    sides: usize,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
) {
    let poly = Poly {
        sides,
        radius,
        center,
        rotation,
        color,
    };
    let points = poly.gen_points();
    let half_thick = thickness / 2.0;

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        let dir = (end - start).normalize();

        draw_line_world(
            start - dir * half_thick,
            end + dir * half_thick,
            thickness,
            color,
        );
    }
}

pub fn should_quit() -> bool {
    get_state().input.close_requested()
}

pub fn get_camera2d() -> &'static mut Camera2D {
    &mut get_state().camera_2d
}

pub fn mutate_camera_2d<T: FnOnce(&'static mut Camera2D)>(f: T) {
    f(&mut get_state().camera_2d);
    get_state().camera_2d.mark_dirty();
}

pub fn mutate_camera_3d<T: FnOnce(&'static mut Camera3D)>(f: T) {
    f(&mut get_state().camera_3d);
    get_state().camera_3d.mark_dirty();
}

pub fn get_camera3d() -> &'static mut Camera3D {
    &mut get_state().camera_3d
}

pub fn camera2d_zoom_at(screen_pos: Vec2, zoom_factor: f32) {
    get_state().camera_2d.zoom_at(screen_pos, zoom_factor);
}

pub fn run_ui(mut f: impl FnMut(&Context)) {
    let state = get_state();
    state.gui_initialized = true;
    state.gui.run(&state.window, |ctx| {
        state.debug_info.draw_debug_info(ctx);

        f(ctx);
    });
}

pub fn draw_sprite(sprite_ref: TextureRef, position: Vec2, scale: f32) {
    let sprite = sprite_ref.get();
    draw_sprite_scaled(sprite_ref, position, sprite.normalized_dimensions * scale);
}

pub fn draw_sprite_scaled(sprite: TextureRef, position: Vec2, scale: Vec2) {
    get_state().draw_queue_2d().add_sprite(
        sprite,
        Transform2D::from_scale_translation(scale, position),
        Color::WHITE,
        None,
    );
}

pub fn draw_sprite_world(sprite_ref: TextureRef, position: Vec2, scale: f32) {
    let sprite = sprite_ref.get();
    draw_sprite_scaled_world(sprite_ref, position, sprite.normalized_dimensions * scale);
}

pub fn draw_sprite_scaled_world(sprite: TextureRef, position: Vec2, scale: Vec2) {
    let bounds = AABB2D::new(position - scale, position + scale);

    if !bounds.is_visible_in_world() {
        return;
    }

    get_state().world_draw_queue_2d().add_sprite(
        sprite,
        Transform2D::from_scale_translation(scale, position),
        Color::WHITE,
        None,
    );
}

pub fn draw_sprite_world_ex(sprite: TextureRef, transform: Transform2D, color: Color) {
    let bounds = AABB2D::new(
        transform.translation() - transform.scale(),
        transform.translation() + transform.scale(),
    );

    if !bounds.is_visible_in_world() {
        return;
    }

    get_state()
        .world_draw_queue_2d()
        .add_sprite(sprite, transform, color, None);
}

pub fn draw_sprite_ex(sprite: TextureRef, transform: Transform2D, color: Color) {
    get_state()
        .draw_queue_2d()
        .add_sprite(sprite, transform, color, None);
}

pub fn screen_to_world(screen_pos: Vec2) -> Vec2 {
    get_state().camera_2d.screen_to_world(screen_pos)
}

pub fn world_to_screen(world_pos: Vec2) -> Vec2 {
    get_state().camera_2d.world_to_screen(world_pos)
}

pub fn rand<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    get_state().rng.random()
}

/// Return a bool with a probability `p` of being true.
pub fn random_bool(p: f64) -> bool {
    get_state().rng.random_bool(p)
}

pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    get_state().rng.random_range(range)
}

/// Return a bool with a probability of `numerator/denominator` of being
/// true.
pub fn random_ratio(numerator: u32, denominator: u32) -> bool {
    get_state().rng.random_ratio(numerator, denominator)
}

// applies when loading a texture, not drawing
//
// setting this to true will make textures look better (less horrible and pixelated) from afer
//
// setting this to false will sometimes make images look crisper
pub fn use_mipmaps(use_mipmaps: bool) {
    get_state().config.use_mipmaps = use_mipmaps;
}

pub fn use_linear_filtering() {
    get_state().config.default_magnify_filter = MagnifySamplerFilter::Linear;
    get_state().config.default_minify_filter = MinifySamplerFilter::Linear;
}

pub fn use_default_filtering() {
    get_state().config.default_magnify_filter = MagnifySamplerFilter::Linear;
    get_state().config.default_minify_filter = MinifySamplerFilter::LinearMipmapLinear;
}

pub fn use_nearest_filtering() {
    get_state().config.default_magnify_filter = MagnifySamplerFilter::Nearest;
    get_state().config.default_minify_filter = MinifySamplerFilter::Nearest;
}

pub fn set_minify_filter(filtering: MinifySamplerFilter) {
    get_state().config.default_minify_filter = filtering;
}

pub fn set_magnify_filter(filtering: MagnifySamplerFilter) {
    get_state().config.default_magnify_filter = filtering;
}

#[cfg(feature = "debugging")]
#[inline]
pub(crate) fn debugger_add_vertices(vertices: usize) {
    use crate::prelude::get_debug_info_mut;
    let debug = get_debug_info_mut();
    debug.current_frame_mut().vertex_count += vertices;
}

#[cfg(not(feature = "debugging"))]
#[inline]
pub(crate) fn debugger_add_vertices(vertices: usize) {}

#[cfg(feature = "debugging")]
#[inline]
pub(crate) fn debugger_add_indices(indices: usize) {
    use crate::prelude::get_debug_info_mut;
    let debug = get_debug_info_mut();
    debug.current_frame_mut().index_count += indices;
}

#[cfg(not(feature = "debugging"))]
#[inline_always]
pub(crate) fn debugger_add_indices(indices: usize) {}

#[cfg(feature = "debugging")]
#[inline]
pub(crate) fn debugger_add_draw_calls(count: usize) {
    use crate::prelude::get_debug_info_mut;
    let debug = get_debug_info_mut();
    debug.current_frame_mut().draw_calls += count;
}

#[cfg(not(feature = "debugging"))]
#[inline_always]
pub(crate) fn debugger_add_draw_calls(count: usize) {}

#[cfg(feature = "debugging")]
#[inline]
pub(crate) fn debugger_add_drawn_objects(count: usize) {
    use crate::prelude::get_debug_info_mut;
    let debug = get_debug_info_mut();
    debug.current_frame_mut().drawn_objects += count;
}

#[cfg(not(feature = "debugging"))]
#[inline_always]
pub(crate) fn debugger_add_drawn_object(count: usize) {}

pub fn time() -> f32 {
    get_state().time
}

pub fn delta_time() -> f32 {
    get_state().delta_time
}

pub fn start_rendering_to_texture(texture: RenderTextureRef) {
    get_state().start_rendering_to_texture(texture);
}

pub fn end_rendering_to_texture() {
    get_state().end_rendering_to_texture();
}

pub(crate) fn empty_render_texture(width: u32, height: u32) -> anyhow::Result<RenderTexture> {
    let state = get_state();
    let facade = &state.display;
    let texture = Texture2d::empty(facade, width, height)?;
    let texture = EngineTexture::new(texture).create();
    Ok(RenderTexture {
        dimensions: UVec2::new(width, height),
        depth_texture: DepthTexture2d::empty(facade, width, height)?,
        color_texture: texture,
    })
}

pub fn create_empty_render_texture(width: u32, height: u32) -> anyhow::Result<RenderTextureRef> {
    Ok(empty_render_texture(width, height)?.create())
}

pub fn add_post_processing_effect(effect: PostProcessingEffect) {
    get_state().current_render_pipeline().add_effect(effect);
}

pub fn blur_screen(sigma: f32) {
    add_post_processing_effect(PostProcessingEffect::GaussianBlur { sigma });
}

/// does not render less textures. if you care about efficency, draw to a smaller render texture and then draw that to the screen
pub fn pixelate_screen(pixel_size: f32) {
    add_post_processing_effect(PostProcessingEffect::Pixelate { pixel_size });
}

pub fn saturate_screen(amount: f32) {
    add_post_processing_effect(PostProcessingEffect::Saturate(amount));
}

pub fn hue_rotate_screen(degrees: f32) {
    add_post_processing_effect(PostProcessingEffect::HueRotate(degrees));
}

pub fn brighten_screen(amount: f32) {
    add_post_processing_effect(PostProcessingEffect::Brighten(amount));
}

pub fn vignette_screen(color: Color, intensity: f32) {
    add_post_processing_effect(PostProcessingEffect::Vignette { color, intensity });
}

pub fn bloom_screen(threshold: f32, intensity: f32, radius: f32) {
    add_post_processing_effect(PostProcessingEffect::Bloom {
        threshold,
        intensity,
        blur_radius: radius,
    });
}

pub fn contrast_screen(amount: f32) {
    add_post_processing_effect(PostProcessingEffect::Contrast(amount));
}

pub fn greyscale_screen() {
    add_post_processing_effect(PostProcessingEffect::Grayscale);
}

pub fn invert_screen() {
    add_post_processing_effect(PostProcessingEffect::Invert);
}

pub fn chromatic_abberation_screen(strength: f32) {
    add_post_processing_effect(PostProcessingEffect::ChromaticAberration { strength });
}

pub fn window_size() -> Vec2 {
    get_state().window_size()
}

pub fn window_height() -> f32 {
    get_state().window_size().y
}

pub fn window_width() -> f32 {
    get_state().window_size().x
}

pub fn draw_fullscreen_texture(texture: TextureRef) {
    draw_sprite_scaled(texture, Vec2::ZERO, window_size());
}

pub fn audio() -> &'static mut AudioEngine {
    &mut get_state().audio_engine
}

pub fn cursor_pos() -> Vec2 {
    get_state().cursor_position
}

pub fn dpi_scaling() -> f32 {
    get_state().dpi_scaling()
}

pub fn default_font() -> FontRef {
    FontRef(0)
}

pub fn frame_count() -> usize {
    get_state().frame_count
}

pub fn physics_time() -> f32 {
    get_state().physics_time
}

pub fn pause_physics_timer() {
    get_state().is_physics_time_paused = true;
}

pub fn play_physics_timer() {
    get_state().is_physics_time_paused = false;
}

pub fn toggle_physics_timer() {
    let state = get_state();
    state.is_physics_time_paused = !state.is_physics_time_paused;
}

pub fn draw_fps() {
    draw_text(format!("{:.1}", avg_fps()), Vec2::new(10.0, 5.0));
}

pub fn storage_store_state<T: Any>(state: T) {
    get_state().user_storage.store(state);
}

pub fn storage_get_state<T: Any>() -> &'static T {
    get_state().user_storage.get()
}

pub fn storage_try_get_state<T: Any>() -> Option<&'static T> {
    get_state().user_storage.try_get()
}

pub fn storage_get_state_mut<T: Any>() -> &'static mut T {
    get_state().user_storage.get_mut()
}

pub fn storage_try_get_state_mut<T: Any>() -> Option<&'static mut T> {
    get_state().user_storage.try_get_mut()
}
