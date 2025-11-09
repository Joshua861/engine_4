use egui_glium::egui_winit::egui::Context;
use glam::Vec2;
use glium::Surface;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    camera::Camera,
    color::Color,
    get_frame, get_state,
    shapes::{
        AABB, CustomShape, Line, Poly, Rect, Triangle, draw_circle_internal,
        draw_circle_world_internal, draw_shape, draw_shape_world, is_world_shape_visible,
    },
    textures::TextureRef,
};

pub fn clear_screen(color: Color) {
    get_frame().clear_color(color.r, color.g, color.b, color.a);
}

pub fn draw_circle(center: Vec2, radius: f32, color: Color) {
    draw_circle_internal(center, radius, color);
}

pub fn draw_circle_world(center: Vec2, radius: f32, color: Color) {
    draw_circle_world_internal(center, radius, color);
}

pub fn draw_custom_shape(points: Vec<Vec2>, color: Color) {
    let shape = CustomShape { points, color };
    draw_shape(shape);
}

pub fn draw_custom_shape_world(points: Vec<Vec2>, color: Color) {
    if points.is_empty() {
        return;
    }

    // Calculate bounding box for culling
    let mut min = points[0];
    let mut max = points[0];

    for point in &points[1..] {
        min = min.min(*point);
        max = max.max(*point);
    }

    let bounds = AABB::new(min, max);
    if !is_world_shape_visible(bounds) {
        return;
    }

    let shape = CustomShape { points, color };
    draw_shape_world(shape);
}

pub fn draw_line(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    let line = Line {
        start,
        end,
        thickness,
        color,
    };

    draw_shape(line);
}

pub fn draw_line_world(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    let half_thick = thickness * 0.5;
    let bounds = AABB::new(
        start.min(end) - Vec2::splat(half_thick),
        start.max(end) + Vec2::splat(half_thick),
    );
    if !is_world_shape_visible(bounds) {
        return;
    }

    let line = Line {
        start,
        end,
        thickness,
        color,
    };
    draw_shape_world(line);
}

pub fn draw_rect(top_left: Vec2, size: Vec2, color: Color) {
    let rect = Rect {
        top_left,
        size,
        color,
    };

    draw_shape(rect);
}

pub fn draw_rect_world(top_left: Vec2, size: Vec2, color: Color) {
    let bounds = AABB::new(top_left, top_left + size);
    if !is_world_shape_visible(bounds) {
        return;
    }

    let rect = Rect {
        top_left,
        size,
        color,
    };
    draw_shape_world(rect);
}

pub fn draw_square(top_left: Vec2, size: f32, color: Color) {
    draw_rect(top_left, Vec2::splat(size), color);
}

pub fn draw_square_world(top_left: Vec2, size: f32, color: Color) {
    draw_rect_world(top_left, Vec2::splat(size), color);
}

pub fn draw_tri(a: Vec2, b: Vec2, c: Vec2, color: Color) {
    let tri = Triangle {
        points: [a, b, c],
        color,
    };

    draw_shape(tri);
}

pub fn draw_tri_world(a: Vec2, b: Vec2, c: Vec2, color: Color) {
    let min = a.min(b).min(c);
    let max = a.max(b).max(c);
    let bounds = AABB::new(min, max);
    if !is_world_shape_visible(bounds) {
        return;
    }

    let tri = Triangle {
        points: [a, b, c],
        color,
    };
    draw_shape_world(tri);
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
    let min = a.min(b).min(c);
    let max = a.max(b).max(c);
    let bounds = AABB::new(min, max).expand(thickness * 0.5);
    if !is_world_shape_visible(bounds) {
        return;
    }

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
    let bounds = AABB::new(top_left, top_left + size).expand(thickness * 0.5);
    if !is_world_shape_visible(bounds) {
        return;
    }

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

pub fn draw_poly(center: Vec2, sides: usize, radius: f32, rotation: f32, color: Color) {
    let poly = Poly {
        sides,
        radius,
        center,
        rotation,
        color,
    };

    draw_shape(poly);
}

pub fn draw_poly_world(center: Vec2, sides: usize, radius: f32, rotation: f32, color: Color) {
    let bounds = AABB::from_center_size(center, Vec2::splat(radius * 2.0));
    if !is_world_shape_visible(bounds) {
        return;
    }

    let poly = Poly {
        sides,
        radius,
        center,
        rotation,
        color,
    };
    draw_shape_world(poly);
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
    let bounds = AABB::from_center_size(center, Vec2::splat(radius * 2.0)).expand(thickness * 0.5);
    if !is_world_shape_visible(bounds) {
        return;
    }

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

pub fn run_ui(mut f: impl FnMut(&Context)) {
    let state = get_state();
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

    if !is_world_shape_visible(bounds) {
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
