use crate::{collisions::AABB, shapes::*, sound::SoundRef};
use bevy_math::Vec2;
use egui_glium::egui_winit::egui::Context;
use glium::Surface;
use rand::{
    Rng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
};
use winit_input_helper::WinitInputHelper;

use crate::{camera::Camera, color::Color, get_frame, get_state, textures::TextureRef};

pub fn clear_screen(color: Color) {
    get_frame().clear_color(color.r, color.g, color.b, color.a);
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

pub fn get_input() -> &'static WinitInputHelper {
    &get_state().input
}

pub fn get_camera() -> &'static Camera {
    &get_state().camera
}

pub fn mutate_camera<T: FnOnce(&'static mut Camera)>(f: T) {
    f(&mut get_state().camera);
    get_state().camera.mark_dirty();
}

pub fn camera_zoom_at(screen_pos: Vec2, zoom_factor: f32) {
    get_state().camera.zoom_at(screen_pos, zoom_factor);
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
    get_state().draw_queue.add_sprite(sprite, position, scale);
}

pub fn draw_sprite_world(sprite_ref: TextureRef, position: Vec2, scale: f32) {
    let sprite = sprite_ref.get();
    draw_sprite_scaled_world(sprite_ref, position, sprite.normalized_dimensions * scale);
}

pub fn draw_sprite_scaled_world(sprite: TextureRef, position: Vec2, scale: Vec2) {
    let bounds = AABB::new(position - scale, position + scale);

    if !bounds.is_visible_in_world() {
        return;
    }

    get_state()
        .world_draw_queue
        .add_sprite(sprite, position, scale);
}

pub fn screen_to_world(screen_pos: Vec2) -> Vec2 {
    get_state().camera.screen_to_world(screen_pos)
}

pub fn world_to_screen(world_pos: Vec2) -> Vec2 {
    get_state().camera.world_to_screen(world_pos)
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
