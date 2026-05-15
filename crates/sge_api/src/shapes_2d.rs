#![allow(clippy::too_many_arguments)]

use sge_color::Color;
use sge_macros::draw_shape_variants;
use sge_math::collision::{self, HasBounds2D, Polygon};
use sge_rendering::{
    d2::Renderer2D,
    dq2d,
    pipeline::{draw_queue_2d, world_draw_queue_2d},
    wdq2d,
};
use sge_shapes::d2::*;
pub use sge_types::Orientation;
use sge_types::{ColorVertex2D, SdfInstance, SdfStroke};
use sge_vectors::{Vec2, vec2};

use crate::draw_to;

pub trait Shape2DExt: Shape2D + Sized {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_shape(self);
    }

    fn draw(&self) {
        self.draw_to(draw_queue_2d().renderer())
    }

    fn draw_world(&self) {
        if self.is_visible_in_world() {
            self.draw_to(world_draw_queue_2d().renderer())
        }
    }

    fn draw_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_sdf(self.sdf().with_stroke(thickness, color, SdfStroke::Inside));
    }

    fn draw_with_outline_to(&self, renderer: Renderer2D, thickness: f32, color: Color) {
        self.draw_to(renderer);
        self.draw_outline_to(renderer, thickness, color);
    }

    fn draw_outline(&self, thickness: f32, color: Color) {
        self.draw_outline_to(draw_queue_2d().renderer(), thickness, color);
    }

    fn draw_outline_world(&self, thickness: f32, color: Color) {
        if self.is_visible_in_world() {
            self.draw_outline_to(world_draw_queue_2d().renderer(), thickness, color);
        }
    }

    fn draw_with_outline(&self, thickness: f32, color: Color) {
        self.draw_with_outline_to(draw_queue_2d().renderer(), thickness, color);
    }

    fn draw_with_outline_world(&self, thickness: f32, color: Color) {
        if self.is_visible_in_world() {
            self.draw_with_outline_to(world_draw_queue_2d().renderer(), thickness, color);
        }
    }
}

impl<T: Shape2D> Shape2DExt for T {}

macro_rules! draw_variants {
    (
        fn $name:ident ( $($param:ident : $ptype:ty),* $(,)? ) {
            screen($r:ident) { $($sbody:tt)* }
            world($r2:ident)  { $($wbody:tt)* }
        }
    ) => {
        paste::paste! {
            #[allow(unused_mut)]
            pub fn [<draw_ $name _to>]($($param: $ptype,)* mut $r: Renderer2D) {
                $($sbody)*
            }
            pub fn [<draw_ $name>]($($param: $ptype),*) {
                [<draw_ $name _to>]($($param,)* dq2d());
            }
            #[allow(unused_mut)]
            pub fn [<draw_ $name _world>]($($param: $ptype),*) {
                let mut $r2 = wdq2d();
                $($wbody)*
            }
        }
    };

    (
        fn $name:ident ( $($param:ident : $ptype:ty),* $(,)? ) [$r:ident] { $($body:tt)* }
    ) => {
        paste::paste! {
            pub fn [<draw_ $name _to>]($($param: $ptype,)* $r: Renderer2D) {
                #[allow(unused_mut)]
                $($body)*
            }
            pub fn [<draw_ $name>]($($param: $ptype),*) {
                [<draw_ $name _to>]($($param,)* dq2d());
            }
            pub fn [<draw_ $name _world>]($($param: $ptype),*) {
                [<draw_ $name _to>]($($param,)* wdq2d());
            }
        }
    };
}

draw_shape_variants! {
    rect [rotation, outline, with_outline]:
        top_left: Vec2, size: Vec2, color: Color
        => Rect { top_left, size, color, rot },

    square [rotation, outline, with_outline]:
        top_left: Vec2, size: f32, color: Color
        => Rect { top_left, size: Vec2::splat(size), color, rot },

    tri [rotation, outline, with_outline]:
        a: Vec2, b: Vec2, c: Vec2, color: Color
        => Triangle { points: [a, b, c], color, rot },

    line [rotation]:
        start: Vec2, end: Vec2, thickness: f32, color: Color
        => Line2D { start, end, thickness, color },

    poly [outline, with_outline]:
        center: Vec2, sides: usize, radius: f32, rotation: f32, color: Color
        => Poly { center, sides, radius, rotation, color },

    hexagon [outline, with_outline]:
        center: Vec2, radius: f32, color: Color
        => Poly { center, sides: 6, radius, rotation: 0.0, color },

    hexagon_pointy [outline, with_outline]:
        center: Vec2, radius: f32, color: Color
        => Poly { center, sides: 6, radius, rotation: std::f32::consts::FRAC_PI_6, color },
}

draw_variants! {
    fn circle(center: Vec2, radius: f32, color: Color) {
        screen(renderer) { Circle::new(center, Vec2::splat(radius), color).draw_to(renderer); }
        world(renderer)  {
            let shape = Circle { center, radius: Vec2::splat(radius), color };
            if shape.bounds().is_visible_in_world() {
                Circle::new(center, Vec2::splat(radius), color).draw_to(renderer);
            }
        }
    }
}

draw_variants! {
    fn ellipse(center: Vec2, radius: Vec2, color: Color) {
        screen(renderer) { renderer.add_shape(&Circle::new(center, radius, color)); }
        world(renderer)  {
            let shape = Circle { center, radius, color };
            if shape.bounds().is_visible_in_world() {
                renderer.add_shape(&Circle::new(center, radius, color));
            }
        }
    }
}

draw_variants! {
    fn circle_outline(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
        screen(renderer) {
            CircleWithOutline::new(center, Vec2::splat(radius), outline_color, thickness, Color::TRANSPARENT).draw_to(renderer);
        }
        world(renderer) {
            let shape = Circle { center, radius: Vec2::splat(radius), color: Color::TRANSPARENT };
            if shape.bounds().is_visible_in_world() {
                CircleWithOutline::new(center, Vec2::splat(radius), outline_color, thickness, Color::TRANSPARENT).draw_to(renderer);
            }
        }
    }
}

draw_variants! {
    fn ellipse_outline(center: Vec2, radius: Vec2, outline_color: Color, thickness: f32) {
        screen(renderer) {
            CircleWithOutline::new(center, radius, outline_color, thickness, Color::TRANSPARENT).draw_to(renderer);
        }
        world(renderer) {
            let shape = Circle { center, radius: radius + Vec2::splat(thickness), color: outline_color };
            if shape.bounds().is_visible_in_world() {
                CircleWithOutline::new(center, radius, outline_color, thickness, Color::TRANSPARENT).draw_to(renderer);
            }
        }
    }
}

draw_variants! {
    fn circle_with_outline(center: Vec2, radius: f32, fill: Color, outline: Color, thickness: f32) {
        screen(renderer) {
            CircleWithOutline::new(center, Vec2::splat(radius), outline, thickness, fill).draw_to(renderer);
        }
        world(renderer) {
            let shape = Circle { center, radius: Vec2::splat(radius + thickness), color: fill };
            if shape.bounds().is_visible_in_world() {
                CircleWithOutline::new(center, Vec2::splat(radius), outline, thickness, fill).draw_to(renderer);
            }
        }
    }
}

draw_variants! {
    fn ellipse_with_outline(center: Vec2, radius: Vec2, fill: Color, outline: Color, thickness: f32) {
        screen(renderer) {
            CircleWithOutline::new(center, radius, outline, thickness, fill).draw_to(renderer);
        }
        world(renderer) {
            let shape = Circle { center, radius: radius + Vec2::splat(thickness), color: fill };
            if shape.bounds().is_visible_in_world() {
                CircleWithOutline::new(center, radius, outline, thickness, fill).draw_to(renderer);
            }
        }
    }
}

draw_variants! {
    fn sector(
        center: Vec2, radius: f32, start_angle: f32, end_angle: f32, color: Color
    ) {
        screen(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color: color, start_angle, end_angle, outline_color: Color::TRANSPARENT, outline_thickness: 0.0 };
            renderer.add_shape(&shape);
        }
        world(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color: color, start_angle, end_angle, outline_color: Color::TRANSPARENT, outline_thickness: 0.0 };
            if shape.bounds().is_visible_in_world() {
                renderer.add_shape(&shape);
            }
        }
    }
}

draw_variants! {
    fn sector_outline(
        center: Vec2, radius: f32, start_angle: f32, end_angle: f32,
        outline_color: Color, thickness: f32,
    ) {
        screen(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color: Color::TRANSPARENT, start_angle, end_angle, outline_color, outline_thickness: thickness };
            renderer.add_shape(&shape);
        }
        world(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color: Color::TRANSPARENT, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_shape(&shape);
            }
        }
    }
}

draw_variants! {
    fn sector_with_outline(
        center: Vec2, radius: f32, start_angle: f32, end_angle: f32,
        fill_color: Color, outline_color: Color, thickness: f32,
    ) {
        screen(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color, start_angle, end_angle, outline_color, outline_thickness: thickness };
            renderer.add_shape(&shape);
        }
        world(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_shape(&shape);
            }
        }
    }
}

draw_variants! {
    fn ellipse_sector(
        center: Vec2, radius: Vec2, start_angle: f32, end_angle: f32, color: Color
    ) {
        screen(renderer) {
            let shape = Sector { center, radius, fill_color: color, start_angle, end_angle, outline_color: Color::TRANSPARENT, outline_thickness: 0.0 };
            renderer.add_shape(&shape);
        }
        world(renderer) {
            let shape = Sector { center, radius, fill_color: color, start_angle, end_angle, outline_color: Color::TRANSPARENT, outline_thickness: 0.0 };
            if shape.bounds().is_visible_in_world() {
                renderer.add_shape(&shape);
            }
        }
    }
}

draw_variants! {
    fn ellipse_sector_outline(
        center: Vec2, radius: Vec2, start_angle: f32, end_angle: f32,
        outline_color: Color, thickness: f32,
    ) {
        screen(renderer) {
            let shape = Sector { center, radius, fill_color: Color::TRANSPARENT, start_angle, end_angle, outline_color, outline_thickness: thickness };
            renderer.add_shape(&shape);
        }
        world(renderer) {
            let shape = Sector { center, radius, fill_color: Color::TRANSPARENT, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_shape(&shape);
            }
        }
    }
}

draw_variants! {
    fn ellipse_sector_with_outline(
        center: Vec2, radius: Vec2, start_angle: f32, end_angle: f32,
        fill_color: Color, outline_color: Color, thickness: f32,
    ) {
        screen(renderer) {
            let shape = Sector { center, radius, fill_color, start_angle, end_angle, outline_color, outline_thickness: thickness };
            renderer.add_shape(&shape);
        }
        world(renderer) {
            let shape = Sector { center, radius, fill_color, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_shape(&shape);
            }
        }
    }
}

draw_variants! {
    fn capped_line(start: Vec2, end: Vec2, thickness: f32, color: Color) {
        screen(renderer) {
            draw_to(&Line2D::new(start, end, thickness, color).with_caps(), renderer);
        }
        world(renderer) {
            let line = Line2D::new(start, end, thickness, color).with_caps();
            if line.is_visible_in_world() {
                draw_to(&line, renderer);
            }
        }
    }
}

draw_variants! {
    fn half_capped_line(start: Vec2, end: Vec2, thickness: f32, color: Color) {
        screen(renderer) {
            draw_to(&Line2D::new(start, end, thickness, color).with_half_caps(), renderer);
        }
        world(renderer) {
            let line = Line2D::new(start, end, thickness, color).with_half_caps();
            if line.is_visible_in_world() {
                draw_to(&line, renderer);
            }
        }
    }
}

draw_variants! {
    fn path(points: &[Vec2], thickness: f32, color: Color) [renderer] {
        points.windows(2).for_each(|p| draw_line_to(p[0], p[1], thickness, color, renderer));
    }
}

draw_variants! {
    fn connected_path(points: &[Vec2], thickness: f32, color: Color) [renderer] {
        points.windows(2).for_each(|p| draw_capped_line_to(p[0], p[1], thickness, color, renderer));
    }
}

draw_variants! {
    fn circle_path(points: &[Vec2], thickness: f32, color: Color) [renderer] {
        for point in points {
            draw_circle_to(*point, thickness / 2.0, color, renderer);
        }
        draw_path_to(points, thickness, color, renderer);
    }
}

draw_variants! {
    fn arrow(start: Vec2, end: Vec2, thickness: f32, color: Color) [renderer] {
        let dir = (end - start).normalize();
        let perp = Vec2::new(-dir.y, dir.x);
        let w = thickness * 2.0;
        let d = thickness * 4.0;
        let mult = ((d / w) * 2.0).sqrt();

        let tip = end;
        let out_left = tip + perp * w - dir * d;
        let out_right = tip - perp * w - dir * d;
        let in_left = out_left - perp * thickness;
        let in_right = out_right + perp * thickness;
        let notch = end - dir * thickness * mult;

        draw_line_to(start, notch, thickness, color, renderer);

        draw_tri_to(out_left, in_left, notch, color, renderer);
        draw_tri_to(out_left, notch, tip, color, renderer);
        draw_tri_to(out_right, in_right, notch, color, renderer);
        draw_tri_to(out_right, notch, tip, color, renderer);
    }
}

fn draw_right_angled_arrow_internal(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    color: Color,
    renderer: Renderer2D,
    f: impl Fn(Vec2, Vec2, f32, Color, Renderer2D),
) {
    let delta = end - start;
    let horizontal = delta.x.abs() > delta.y.abs();

    let (half_main, cross) = if horizontal {
        (vec2(delta.x / 2.0, 0.0), vec2(0.0, delta.y))
    } else {
        (vec2(0.0, delta.y / 2.0), vec2(delta.x, 0.0))
    };

    let mut cursor = start;

    draw_line_to(cursor, cursor + half_main, thickness, color, renderer);
    cursor += half_main;

    draw_capped_line_to(cursor, cursor + cross, thickness, color, renderer);
    cursor += cross;

    f(cursor, cursor + half_main, thickness, color, renderer);
    cursor += half_main;
}

draw_variants! {
    fn right_angled_arrow(
        start: Vec2,
        end: Vec2,
        thickness: f32,
        color: Color,
    ) [renderer] {
        draw_right_angled_arrow_internal(start, end, thickness, color, renderer, draw_arrow_to);
    }
}

draw_variants! {
    fn right_angled_solid_arrow(
        start: Vec2,
        end: Vec2,
        thickness: f32,
        color: Color,
    ) [renderer] {
        draw_right_angled_arrow_internal(start, end, thickness, color, renderer, draw_solid_arrow_to);
    }
}

draw_variants! {
    fn right_angled_sharp_arrow(start: Vec2, end: Vec2, thickness: f32, color: Color) [renderer] {
        draw_right_angled_arrow_internal(start, end, thickness, color, renderer, draw_sharp_arrow_to);
    }
}

draw_variants! {
    fn solid_arrow(
        start: Vec2,
        end: Vec2,
        thickness: f32,
        color: Color,
    ) [renderer] {
        let dir = (end - start).normalize();
        let perp = Vec2::new(-dir.y, dir.x);
        let h = thickness * 4.0;
        let points = [
            end,
            end - dir * h + perp * h / 2.0,
            end - dir * h - perp * h / 2.0,
        ];
        draw_line_to(start, end - dir * h, thickness, color, renderer);
        draw_tri_to(points[0], points[1], points[2], color, renderer);
    }
}

draw_variants! {
    fn sharp_arrow(
        start: Vec2,
        end: Vec2,
        thickness: f32,
        color: Color,
    ) [renderer] {
        let dir = (end - start).normalize();
        let perp = Vec2::new(-dir.y, dir.x);
        let w = thickness * 2.0;
        let d = thickness * 4.0;
        let mult = ((d / w) * 2.0).sqrt();

        let tip = end;
        let out_left = tip + perp * w - dir * d;
        let out_right = tip - perp * w - dir * d;
        let notch = end - dir * thickness * mult;

        draw_line_to(start, notch, thickness, color, renderer);

        draw_tri_to(out_left, notch, tip, color, renderer);
        draw_tri_to(out_right, notch, tip, color, renderer);
    }
}

draw_variants! {
    fn zig_zag_ex(
        start: Vec2,
        end: Vec2,
        thickness: f32,
        color: Color,
        width: f32,
        num_segments: usize,
    ) [renderer] {
        let delta = end - start;
        let dir = delta.normalize();
        let perp = Vec2::new(-dir.y, dir.x);
        let step = delta / num_segments as f32;
        let thick = perp * thickness * 0.5;

        let mut spine = Vec::with_capacity(num_segments + 1);
        for i in 0..=num_segments {
            let base = start + step * i as f32;
            let side = if i % 2 == 0 {
                -width * 0.5
            } else {
                width * 0.5
            };
            spine.push(base + perp * side);
        }

        for w in spine.windows(2) {
            let (a, b) = (w[0], w[1]);
            draw_tri_to(a - thick, a + thick, b + thick, color, renderer);
            draw_tri_to(a - thick, b + thick, b - thick, color, renderer);
        }
    }
}

draw_variants! {
    fn zig_zag(start: Vec2, end: Vec2, thickness: f32, color: Color) [renderer] {
        draw_zig_zag_ex_to(start, end, thickness, color, 5.0, 10, renderer);
    }
}

draw_variants! {
    fn rounded_rect(top_left: Vec2, size: Vec2, color: Color, corner_radius: f32) [renderer] {
        draw_to(&RoundedRectangle::new(top_left, size, color, corner_radius), renderer);
    }
}

draw_variants! {
    fn rounded_square(top_left: Vec2, size: f32, color: Color, corner_radius: f32) [renderer] {
        draw_to(&RoundedRectangle::new(top_left, Vec2::splat(size), color, corner_radius), renderer);
    }
}

draw_variants! {
    fn rounded_rect_with_outline(
        top_left: Vec2, size: Vec2, color: Color, corner_radius: f32,
        outline_thickness: f32, outline_color: Color,
    ) {
        screen(renderer) {
            RoundedRectangle { top_left, size, fill_color: color, corner_radius,
                outline_thickness, outline_color }.draw_to(renderer);
        }
        world(renderer) {
            draw_to(&RoundedRectangle { top_left, size, fill_color: color, corner_radius,
                outline_thickness, outline_color }, renderer);
        }
    }
}

fn dashed_line_internal(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    color: Color,
    segment_length: f32,
    renderer: Renderer2D,
) {
    let delta = end - start;
    let (dir, length) = delta.normalize_and_length();
    if length == 0.0 || segment_length <= 0.0 {
        return;
    }

    let ideal_count = (length / segment_length).round() as u32;

    let count = if ideal_count < 1 {
        1
    } else if ideal_count % 2 == 0 {
        ideal_count + 1
    } else {
        ideal_count
    };

    let adjusted_len = length / count as f32;

    let mut t = 0.0;
    let mut draw = true;
    for _ in 0..count {
        let next_t = t + adjusted_len;
        if draw {
            let a = start + dir * t;
            let b = start + dir * next_t;
            draw_line_to(a, b, thickness, color, renderer);
        }
        t = next_t;
        draw = !draw;
    }
}

draw_variants! {
    fn dashed_line(start: Vec2, end: Vec2, thickness: f32, color: Color, segment_length: f32) [renderer] {
        dashed_line_internal(start, end, thickness, color, segment_length, renderer);
    }
}

pub trait ToCollider<T> {
    fn to_collider(&self) -> T;
}

impl ToCollider<collision::Circle> for Circle {
    fn to_collider(&self) -> collision::Circle {
        collision::Circle {
            center: self.center,
            radius: self.encompassing_radius(),
        }
    }
}

impl ToCollider<Polygon> for Poly {
    fn to_collider(&self) -> Polygon {
        Polygon {
            vertices: self.gen_points(),
        }
    }
}

fn draw_square_outline_path(
    points: &[Vec2],
    color: Color,
    thickness: f32,
    mut renderer: Renderer2D,
) {
    points.array_windows().for_each(|[a, b]| {
        renderer.add_shape(&Line2D::new(*a, *b, thickness, color).with_caps());
    });

    renderer
        .add_shape(&Line2D::new(points[points.len() - 1], points[0], thickness, color).with_caps());
}

fn draw_circle_outline_path(
    points: &[Vec2],
    thickness: f32,
    color: Color,
    mut renderer: Renderer2D,
) {
    points
        .iter()
        .for_each(|p| Circle::new(*p, Vec2::splat(thickness / 2.0), color).draw_to(renderer));

    points.array_windows().for_each(|[a, b]| {
        renderer.add_shape(&Line2D::new(*a, *b, thickness, color));
    });

    renderer.add_shape(&Line2D::new(
        points[points.len() - 1],
        points[0],
        thickness,
        color,
    ));
}

draw_variants!(
    fn circle_line(start: Vec2, end: Vec2, thickness: f32, color: Color) [renderer] {
        draw_circle_to(start, thickness / 2.0, color, renderer);
        draw_circle_to(end, thickness / 2.0, color, renderer);
        draw_line_to(start, end, thickness, color, renderer);
    }
);

#[derive(Debug, Clone, Copy)]
pub struct GradientPoint {
    width: f32,
    color: Color,
}

impl GradientPoint {
    pub fn new(color: Color, width: f32) -> Self {
        Self { width, color }
    }
}

draw_variants! {
    fn quadratic_bezier(a: Vec2, b: Vec2, c: Vec2, color: Color, thickness: f32) [renderer] {
        let mut renderer = renderer;
        renderer.add_sdf(
        SdfInstance::quadratic_bezier(a, b, c).with_stroke(thickness / 2.0, color, SdfStroke::Outside));
    }
}

draw_variants! {
    fn sdf(sdf: SdfInstance) [renderer] {
        let mut renderer = renderer;
        renderer.add_sdf(sdf);
    }
}

draw_variants! {
    fn pixel(pos: Vec2, color: Color) [renderer] {
        let mut renderer = renderer;
        renderer.add_pixel(pos, color);
    }
}

draw_variants! {
    fn pixel_line(a: Vec2, b: Vec2, color: Color) [renderer] {
        let mut renderer = renderer;
        renderer.add_pixel_line(a, b, color);
    }
}

draw_variants! {
    fn custom_shape(points: &[Vec2], color: Color) [renderer] {
        let mut renderer = renderer;
        let (vertices, indices) = gen_mesh_from_points(&points, color);
        renderer.add_mesh(&vertices, &indices);
    }
}
