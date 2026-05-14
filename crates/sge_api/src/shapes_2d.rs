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
use sge_types::{ColorVertex2D, Pattern};
use sge_vectors::{Vec2, vec2};

use crate::draw_to;

dyn_clone::clone_trait_object!(Shape2DExt);
pub trait Shape2DExt: Shape2D {
    fn draw_to(&self, renderer: Renderer2D);

    fn draw(&self) {
        self.draw_to(draw_queue_2d().renderer())
    }

    fn draw_world(&self) {
        if self.is_visible_in_world() {
            self.draw_to(world_draw_queue_2d().renderer())
        }
    }

    fn draw_outline_to(&self, renderer: Renderer2D, thickness: f32, color: Color);

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

impl Shape2DExt for Circle {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_circle(self.center, self.radius, self.color);
    }

    fn draw_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_circle_with_outline(
            self.center,
            self.radius,
            Color::TRANSPARENT,
            thickness,
            color,
        );
    }

    fn draw_with_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_circle_with_outline(self.center, self.radius, self.color, thickness, color);
    }
}

impl Shape2DExt for CircleWithOutline {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_circle_with_outline(
            self.center,
            self.radius,
            self.fill_color,
            self.outline_thickness,
            self.outline_color,
        );
    }

    fn draw_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_circle_with_outline(
            self.center,
            self.radius,
            Color::TRANSPARENT,
            thickness,
            color,
        );
    }

    fn draw_with_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_circle_with_outline(
            self.center,
            self.radius,
            self.fill_color,
            thickness,
            color,
        );
    }
}

impl Shape2DExt for Sector {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_sector(
            self.center,
            self.radius,
            self.fill_color,
            self.start_angle,
            self.end_angle,
        );
    }

    fn draw_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_sector_with_outline(
            self.center,
            self.radius,
            Color::TRANSPARENT,
            thickness,
            color,
            self.start_angle,
            self.end_angle,
        );
    }

    fn draw_with_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_sector_with_outline(
            self.center,
            self.radius,
            self.fill_color,
            thickness,
            color,
            self.start_angle,
            self.end_angle,
        );
    }
}

impl Shape2DExt for Rect {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_shape(self);
    }

    fn draw_outline_to(&self, renderer: Renderer2D, thickness: f32, color: Color) {
        let points = self.points();
        draw_square_outline_path(&points, color, thickness, renderer);
    }

    fn draw_with_outline_to(&self, renderer: Renderer2D, thickness: f32, color: Color) {
        self.draw_to(renderer);
        self.draw_outline_to(renderer, thickness, color);
    }
}

impl Shape2DExt for Triangle {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_shape(self);
    }

    fn draw_outline_to(&self, renderer: Renderer2D, thickness: f32, color: Color) {
        let points = self.points();
        draw_circle_outline_path(&points, thickness, color, renderer);
    }
}

impl Shape2DExt for Line2D {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_shape(self);
    }

    fn draw_outline_to(&self, _renderer: Renderer2D, _thickness: f32, _color: Color) {
        unimplemented!()
    }
}

impl Shape2DExt for Pixel {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_pixel(self);
    }

    fn draw_outline_to(&self, _renderer: Renderer2D, _thickness: f32, _color: Color) {
        unimplemented!()
    }
}

impl Shape2DExt for PixelLine {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_pixel_line(self);
    }

    fn draw_outline_to(&self, _renderer: Renderer2D, _thickness: f32, _color: Color) {
        unimplemented!()
    }
}

impl Shape2DExt for Poly {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_shape(self);
    }

    fn draw_outline_to(&self, renderer: Renderer2D, thickness: f32, color: Color) {
        let points = self.gen_points();
        draw_circle_outline_path(&points, thickness, color, renderer);
    }
}

impl Shape2DExt for CustomShape {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_shape(self);
    }

    fn draw_outline_to(&self, renderer: Renderer2D, thickness: f32, color: Color) {
        let points = &self.points;
        draw_circle_outline_path(points, thickness, color, renderer);
    }
}

impl Shape2DExt for RoundedRectangle {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_rounded_rectangle(
            self.center(),
            self.size,
            self.corner_radius,
            self.fill_color,
            self.outline_thickness,
            self.outline_color,
        );
    }

    fn draw_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_rounded_rectangle(
            self.center(),
            self.size,
            self.corner_radius,
            Color::TRANSPARENT,
            thickness,
            color,
        );
    }

    fn draw_with_outline_to(&self, mut renderer: Renderer2D, thickness: f32, color: Color) {
        renderer.add_rounded_rectangle(
            self.center(),
            self.size,
            self.corner_radius,
            self.fill_color,
            thickness,
            color,
        );
    }
}

impl Shape2DExt for RadialGradient {
    fn draw_to(&self, mut renderer: Renderer2D) {
        renderer.add_radial_gradient(
            self.center,
            self.radius,
            self.inner_color,
            self.outer_color,
            self.outline_thickness,
            self.outline_color,
            self.gradient_offset,
        );
    }

    fn draw_outline_to(&self, _renderer: Renderer2D, _thickness: f32, _color: Color) {
        unimplemented!()
    }
}

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
        => Line2D { start, end, thickness, color, rot },

    poly [outline, with_outline]:
        center: Vec2, sides: usize, radius: f32, rotation: f32, color: Color
        => Poly { center, sides, radius, rotation, color },

    custom_shape [outline, with_outline]:
        points: Vec<Vec2>, color: Color
        => CustomShape { points, color },

    hexagon [outline, with_outline]:
        center: Vec2, radius: f32, color: Color
        => Poly { center, sides: 6, radius, rotation: 0.0, color },

    hexagon_pointy [outline, with_outline]:
        center: Vec2, radius: f32, color: Color
        => Poly { center, sides: 6, radius, rotation: std::f32::consts::FRAC_PI_6, color },

    pixel []:
        pos: Vec2, color: Color
        => Pixel { pos, color },

    pixel_line []:
        start: Vec2, end: Vec2, color: Color
        => PixelLine { start, end, color },

    radial_gradient []:
        center: Vec2, radius: Vec2, inner_color: Color, outer_color: Color,
        outline_thickness: f32, outline_color: Color, gradient_offset: Vec2
        => RadialGradient {
            center, radius, inner_color, outer_color,
            outline_thickness, outline_color, gradient_offset,
        },
}

draw_variants! {
    fn circle(center: Vec2, radius: f32, color: Color) {
        screen(renderer) { renderer.add_circle(center, Vec2::splat(radius), color); }
        world(renderer)  {
            let shape = Circle { center, radius: Vec2::splat(radius), color };
            if shape.bounds().is_visible_in_world() {
                renderer.add_circle(center, Vec2::splat(radius), color);
            }
        }
    }
}

draw_variants! {
    fn ellipse(center: Vec2, radius: Vec2, color: Color) {
        screen(renderer) { renderer.add_circle(center, radius, color); }
        world(renderer)  {
            let shape = Circle { center, radius, color };
            if shape.bounds().is_visible_in_world() {
                renderer.add_circle(center, radius, color);
            }
        }
    }
}

draw_variants! {
    fn circle_outline(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
        screen(renderer) {
            renderer.add_circle_with_outline(
                center, Vec2::splat(radius),
                outline_color.with_alpha(0.0), thickness, outline_color,
            );
        }
        world(renderer) {
            let shape = Circle { center, radius: Vec2::splat(radius + thickness), color: outline_color };
            if shape.bounds().is_visible_in_world() {
                renderer.add_circle_with_outline(
                    center, Vec2::splat(radius),
                    Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                );
            }
        }
    }
}

draw_variants! {
    fn ellipse_outline(center: Vec2, radius: Vec2, outline_color: Color, thickness: f32) {
        screen(renderer) {
            renderer.add_circle_with_outline(
                center, radius,
                Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
            );
        }
        world(renderer) {
            let shape = Circle { center, radius: radius + Vec2::splat(thickness), color: outline_color };
            if shape.bounds().is_visible_in_world() {
                renderer.add_circle_with_outline(
                    center, radius,
                    Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                );
            }
        }
    }
}

draw_variants! {
    fn circle_with_outline(center: Vec2, radius: f32, fill: Color, outline: Color, thickness: f32) {
        screen(renderer) {
            renderer.add_circle_with_outline(center, Vec2::splat(radius), fill, thickness, outline);
        }
        world(renderer) {
            let shape = Circle { center, radius: Vec2::splat(radius + thickness), color: fill };
            if shape.bounds().is_visible_in_world() {
                renderer.add_circle_with_outline(
                    center, Vec2::splat(radius), fill, thickness, outline,
                );
            }
        }
    }
}

draw_variants! {
    fn ellipse_with_outline(center: Vec2, radius: Vec2, fill: Color, outline: Color, thickness: f32) {
        screen(renderer) {
            renderer.add_circle_with_outline(center, radius, fill, thickness, outline);
        }
        world(renderer) {
            let shape = Circle { center, radius: radius + Vec2::splat(thickness), color: fill };
            if shape.bounds().is_visible_in_world() {
                renderer.add_circle_with_outline(center, radius, fill, thickness, outline);
            }
        }
    }
}

draw_variants! {
    fn sector(
        center: Vec2, radius: f32, start_angle: f32, end_angle: f32, color: Color
    ) {
        screen(renderer) {
            renderer.add_sector(center, Vec2::splat(radius), color, start_angle, end_angle);
        }
        world(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color: color, start_angle, end_angle, outline_color: color, outline_thickness: 0.0 };
            if shape.bounds().is_visible_in_world() {
                renderer.add_sector(center, Vec2::splat(radius), color, start_angle, end_angle);
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
            renderer.add_sector_with_outline(
                center, Vec2::splat(radius),
                Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                start_angle, end_angle,
            );
        }
        world(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color: Color::TRANSPARENT, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_sector_with_outline(
                    center, Vec2::splat(radius),
                    Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                    start_angle, end_angle,
                );
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
            renderer.add_sector_with_outline(
                center, Vec2::splat(radius), fill_color, thickness, outline_color,
                start_angle, end_angle,
            );
        }
        world(renderer) {
            let shape = Sector { center, radius: Vec2::splat(radius), fill_color, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_sector_with_outline(
                    center, Vec2::splat(radius), fill_color, thickness, outline_color,
                    start_angle, end_angle,
                );
            }
        }
    }
}

draw_variants! {
    fn ellipse_sector(
        center: Vec2, radius: Vec2, start_angle: f32, end_angle: f32, color: Color
    ) {
        screen(renderer) {
            renderer.add_sector(center, radius, color, start_angle, end_angle);
        }
        world(renderer) {
            let shape = Sector { center, radius, fill_color: color, start_angle, end_angle, outline_color: color, outline_thickness: 0.0 };
            if shape.bounds().is_visible_in_world() {
                renderer.add_sector(center, radius, color, start_angle, end_angle);
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
            renderer.add_sector_with_outline(
                center, radius,
                Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                start_angle, end_angle,
            );
        }
        world(renderer) {
            let shape = Sector { center, radius: radius, fill_color: Color::TRANSPARENT, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_sector_with_outline(
                    center, radius,
                    Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                    start_angle, end_angle,
                );
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
            renderer.add_sector_with_outline(
                center, radius, fill_color, thickness, outline_color,
                start_angle, end_angle,
            );
        }
        world(renderer) {
            let shape = Sector { center, radius, fill_color, start_angle, end_angle, outline_color, outline_thickness: thickness };
            if shape.bounds().is_visible_in_world() {
                renderer.add_sector_with_outline(
                    center, radius, fill_color, thickness, outline_color,
                    start_angle, end_angle,
                );
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

fn rect_gradient_mesh(
    top_left: Vec2,
    size: Vec2,
    c_tl: Color,
    c_tr: Color,
    c_bl: Color,
    c_br: Color,
) -> [ColorVertex2D; 4] {
    let tl = top_left;
    let tr = top_left + Vec2::new(size.x, 0.0);
    let bl = top_left + Vec2::new(0.0, size.y);
    let br = top_left + size;
    [
        ColorVertex2D::new(tl.x, tl.y, c_tl),
        ColorVertex2D::new(tr.x, tr.y, c_tr),
        ColorVertex2D::new(bl.x, bl.y, c_bl),
        ColorVertex2D::new(br.x, br.y, c_br),
    ]
}

draw_variants! {
    fn triangle_gradient(a: Vec2, b: Vec2, c: Vec2, ca: Color, cb: Color, cc: Color) {
        screen(renderer) {
            renderer.add_mesh(
                &[ColorVertex2D::new(a.x, a.y, ca), ColorVertex2D::new(b.x, b.y, cb), ColorVertex2D::new(c.x, c.y, cc)],
                &[0, 1, 2],
            );
        }
        world(renderer) {
            let tri = Triangle { points: [a, b, c], color: Color::TRANSPARENT, rot: 0.0 };
            if !tri.bounds().is_visible_in_world() { return; }
            renderer.add_mesh(
                &[ColorVertex2D::new(a.x, a.y, ca), ColorVertex2D::new(b.x, b.y, cb), ColorVertex2D::new(c.x, c.y, cc)],
                &[0, 1, 2],
            );
        }
    }
}

draw_variants! {
    fn rect_gradient(
        top_left: Vec2, size: Vec2,
        c_tl: Color, c_tr: Color, c_bl: Color, c_br: Color,
    ) {
        screen(renderer) {
            renderer.add_mesh(&rect_gradient_mesh(top_left, size, c_tl, c_tr, c_bl, c_br), &QUAD_INDICES);
        }
        world(renderer) {
            let rect = Rect { top_left, size, color: Color::TRANSPARENT, rot: 0.0 };
            if !rect.bounds().is_visible_in_world() { return; }
            renderer.add_mesh(&rect_gradient_mesh(top_left, size, c_tl, c_tr, c_bl, c_br), &QUAD_INDICES);
        }
    }
}

draw_variants! {
    fn square_gradient_all(
        top_left: Vec2, size: f32,
        c_tl: Color, c_tr: Color, c_bl: Color, c_br: Color,
    ) [renderer] {
        draw_rect_gradient_to(top_left, Vec2::splat(size), c_tl, c_tr, c_bl, c_br, renderer);
    }
}

fn line_gradient_vertices(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    c_start_left: Color,
    c_start_right: Color,
    c_end_left: Color,
    c_end_right: Color,
) -> Vec<ColorVertex2D> {
    Line2D {
        start,
        end,
        thickness,
        color: Color::TRANSPARENT,
        rot: 0.0,
    }
    .gen_mesh(0)
    .1
    .into_iter()
    .zip([c_start_right, c_end_right, c_start_left, c_end_left])
    .map(|(mut v, c)| {
        v.color = c.for_gpu();
        v
    })
    .collect()
}

draw_variants! {
    fn line_gradient(start: Vec2, end: Vec2, thickness: f32, start_color: Color, end_color: Color) [renderer] {
        draw_line_gradient_ex_to(start, end, thickness, start_color, start_color, end_color, end_color, renderer);
    }
}

draw_variants! {
    fn line_gradient_ex(
        start: Vec2, end: Vec2, thickness: f32,
        c_start_left: Color, c_start_right: Color,
        c_end_left: Color, c_end_right: Color,
    ) {
        screen(renderer) {
            let verts = line_gradient_vertices(start, end, thickness, c_start_left, c_start_right, c_end_left, c_end_right);
            renderer.add_mesh(&verts, &QUAD_INDICES);
        }
        world(renderer) {
            let line = Line2D { start, end, thickness, color: Color::TRANSPARENT, rot: 0.0 };
            if !line.bounds().is_visible_in_world() { return; }
            let verts = line_gradient_vertices(start, end, thickness, c_start_left, c_start_right, c_end_left, c_end_right);
            if verts.is_empty() { return; }
            renderer.add_mesh(&verts, &QUAD_INDICES);
        }
    }
}

draw_variants! {
    fn gradient_path(points: &[Vec2], thickness: f32, start: Color, end: Color) [renderer] {
        draw_gradient_path_internal(points, thickness, start, end, draw_line_gradient_to, renderer);
    }
}

fn draw_gradient_path_internal<F>(
    points: &[Vec2],
    thickness: f32,
    start: Color,
    end: Color,
    mut draw_seg: F,
    renderer: Renderer2D,
) where
    F: FnMut(Vec2, Vec2, f32, Color, Color, Renderer2D),
{
    if points.len() < 2 {
        return;
    }
    let total_len: f32 = points.windows(2).map(|p| (p[1] - p[0]).length()).sum();
    if total_len == 0.0 {
        return;
    }
    let mut acc = 0.0;
    for seg in points.windows(2) {
        let len = (seg[1] - seg[0]).length();
        let t0 = acc / total_len;
        let t1 = (acc + len) / total_len;
        draw_seg(
            seg[0],
            seg[1],
            thickness,
            start.blend(end, t0),
            start.blend(end, t1),
            renderer,
        );
        acc += len;
    }
}

macro_rules! draw_rect_gradient_dirs {
    ($stem:ident, $size_ty:ty, $to_vec2:expr) => {
        paste::paste! {
            draw_variants! {
                fn [<$stem _gradient_vertical>](top_left: Vec2, size: $size_ty, top: Color, bottom: Color) [renderer] {
                    draw_rect_gradient_to(top_left, $to_vec2(size), top, top, bottom, bottom, renderer);
                }
            }
            draw_variants! {
                fn [<$stem _gradient_horizontal>](top_left: Vec2, size: $size_ty, left: Color, right: Color) [renderer] {
                    draw_rect_gradient_to(top_left, $to_vec2(size), left, right, left, right, renderer);
                }
            }
            draw_variants! {
                fn [<$stem _gradient_tl_br>](top_left: Vec2, size: $size_ty, tl: Color, br: Color) [renderer] {
                    let m = tl.blend_halfway(br);
                    draw_rect_gradient_to(top_left, $to_vec2(size), tl, m, m, br, renderer);
                }
            }
            draw_variants! {
                fn [<$stem _gradient_tr_bl>](top_left: Vec2, size: $size_ty, tr: Color, bl: Color) [renderer] {
                    let m = tr.blend_halfway(bl);
                    draw_rect_gradient_to(top_left, $to_vec2(size), m, tr, bl, m, renderer);
                }
            }
        }
    };
}

draw_rect_gradient_dirs!(square, f32, Vec2::splat);
draw_rect_gradient_dirs!(rect, Vec2, |s: Vec2| s);

macro_rules! gen_radial_gradient_variants {
    () => {
        gen_radial_gradient_variants!(@shape circle,  radius: f32,  Vec2::splat(radius));
        gen_radial_gradient_variants!(@shape ellipse, radius: Vec2, radius);
    };

    (@shape $name:ident, $radius_param:ident: $radius_ty:ty, $radius_expr:expr) => {
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [],
            outline_params: [],
            outline_val:    [0.0, Color::TRANSPARENT],
            offset_params:  [],
            offset_val:     [Vec2::ZERO],
            vis_radius:     [$radius_expr]
        );
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [_with_outline],
            outline_params: [outline_thickness: f32, outline_color: Color],
            outline_val:    [outline_thickness, outline_color],
            offset_params:  [],
            offset_val:     [Vec2::ZERO],
            vis_radius:     [$radius_expr + Vec2::splat(outline_thickness)]
        );
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [_offset],
            outline_params: [],
            outline_val:    [0.0, Color::TRANSPARENT],
            offset_params:  [gradient_offset: Vec2],
            offset_val:     [gradient_offset],
            vis_radius:     [$radius_expr]
        );
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [_with_outline_offset],
            outline_params: [outline_thickness: f32, outline_color: Color],
            outline_val:    [outline_thickness, outline_color],
            offset_params:  [gradient_offset: Vec2],
            offset_val:     [gradient_offset],
            vis_radius:     [$radius_expr + Vec2::splat(outline_thickness)]
        );
    };

    (
        @emit $name:ident, $radius_param:ident: $radius_ty:ty, $radius_expr:expr,
        fn_suffix:      [$($suffix:tt)*],
        outline_params: [$($outline_param:ident: $outline_ty:ty),*],
        outline_val:    [$outline_thickness_val:expr, $outline_color_val:expr],
        offset_params:  [$($offset_param:ident: $offset_ty:ty),*],
        offset_val:     [$offset_val:expr],
        vis_radius:     [$vis_radius:expr]
    ) => {
        paste::paste! {
            draw_variants! {
                fn [<radial_gradient_ $name $($suffix)*>](
                    center: Vec2,
                    $radius_param: $radius_ty,
                    inner: Color,
                    outer: Color,
                    $($outline_param: $outline_ty,)*
                    $($offset_param: $offset_ty,)*
                ) {
                    screen(renderer) {
                        RadialGradient {
                            center, radius: $radius_expr, inner_color: inner, outer_color: outer,
                            outline_thickness: $outline_thickness_val, outline_color: $outline_color_val,
                            gradient_offset: $offset_val,
                        }.draw_to(renderer);
                    }
                    world(renderer) {
                        let shape = RadialGradient {
                            center, radius: $radius_expr, inner_color: inner, outer_color: outer,
                            outline_thickness: $outline_thickness_val, outline_color: $outline_color_val,
                            gradient_offset: $offset_val,
                        };

                        if shape.bounds().is_visible_in_world() {
                            shape.draw_to(renderer);
                        }
                    }
                }
            }
        }
    };
}

gen_radial_gradient_variants!();

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

impl ToCollider<Polygon> for CustomShape {
    fn to_collider(&self) -> Polygon {
        Polygon {
            vertices: self.points.clone(),
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
        .for_each(|p| renderer.add_circle(*p, Vec2::splat(thickness / 2.0), color));

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

fn draw_multi_point_gradient_internal(
    pos: Vec2,
    size: Vec2,
    orientation: Orientation,
    points: &[GradientPoint],
    renderer: Renderer2D,
) {
    let total_width: f32 = points.iter().map(|p| p.width).sum();
    if total_width == 0.0 {
        return;
    }
    let main_axis = match orientation {
        Orientation::Vertical => size.y,
        Orientation::Horizontal => size.x,
    };
    let cross_axis = match orientation {
        Orientation::Vertical => size.x,
        Orientation::Horizontal => size.y,
    };
    let width_multiplier = main_axis / total_width;
    let mut offset = 0.0f32;
    for i in 0..points.len() {
        let point = &points[i];
        let next_color = points.get(i + 1).map(|p| p.color).unwrap_or(point.color);
        let w = point.width * width_multiplier;
        let (top_left, seg_size, c_tl, c_tr, c_bl, c_br) = match orientation {
            Orientation::Vertical => (
                pos + vec2(0.0, offset),
                vec2(cross_axis, w),
                point.color,
                point.color,
                next_color,
                next_color,
            ),
            Orientation::Horizontal => (
                pos + vec2(offset, 0.0),
                vec2(w, cross_axis),
                point.color,
                next_color,
                point.color,
                next_color,
            ),
        };
        draw_rect_gradient_to(top_left, seg_size, c_tl, c_tr, c_bl, c_br, renderer);
        offset += w;
    }
}

draw_variants! {
    fn multi_point_gradient(
        pos: Vec2,
        size: Vec2,
        orientation: Orientation,
        points: &[GradientPoint],
    ) [renderer] {
        draw_multi_point_gradient_internal(pos, size, orientation, points, renderer);
    }
}

draw_variants! {
    fn quadratic_bezier(a: Vec2, b: Vec2, c: Vec2, color: Color, thickness: f32) [renderer] {
        let mut renderer = renderer;
        renderer.add_quadratic_bezier(a, b, c, color, thickness);
    }
}

draw_variants! {
    fn cubic_bezier(a: Vec2, b: Vec2, c: Vec2,d: Vec2, color: Color, thickness: f32) [renderer] {
        let mut renderer = renderer;
        renderer.add_cubic_bezier(a, b, c, d, color, thickness);
    }
}

draw_variants! {
    fn shape_with_pattern(shape: impl Shape2D, alt_color: Color, pattern: Pattern, scale: f32) [renderer] {
        let mut renderer = renderer;
        renderer.add_shape_with_pattern(&shape, alt_color, pattern, scale);
    }
}
