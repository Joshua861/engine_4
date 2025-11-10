use crate::{
    Vertex,
    collisions::{AABB, HasBounds},
    color::Color,
    get_state,
};
use bevy_math::Vec2;
use glium::implement_vertex;
use std::f32::consts::TAU;

#[derive(Copy, Clone, Debug)]
pub struct CircleInstance {
    pub center: [f32; 2],
    pub radius: f32,
    pub color: [f32; 4],
}

implement_vertex!(CircleInstance, center, radius, color);

impl CircleInstance {
    pub fn new(center: Vec2, radius: f32, color: Color) -> Self {
        Self {
            center: [center.x, center.y],
            radius,
            color: color.for_gpu(),
        }
    }
}

pub trait Shape: HasBounds {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>);
    fn is_visible_in_world(&self) -> bool {
        self.bounds().is_visible_in_world()
    }
}

pub(crate) struct Circle {
    pub center: Vec2,
    pub radius: f32,
    pub color: Color,
}

impl HasBounds for Circle {
    fn bounds(&self) -> AABB {
        AABB::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
    }
}

impl Shape for Circle {
    fn points(&self, _starting_index: u32) -> (Vec<u32>, Vec<Vertex>) {
        (vec![], vec![])
    }
}

pub(crate) struct Rect {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl HasBounds for Rect {
    fn bounds(&self) -> AABB {
        AABB::new(self.top_left, self.top_left + self.size)
    }
}

impl Rect {
    fn gen_quad(&self) -> Vec<Vertex> {
        let tl = self.top_left;
        let br = self.top_left + self.size;

        vec![
            Vertex::new(tl.x, tl.y, self.color),
            Vertex::new(br.x, tl.y, self.color),
            Vertex::new(tl.x, br.y, self.color),
            Vertex::new(br.x, br.y, self.color),
        ]
    }
}

impl Shape for Rect {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>) {
        let quad = self.gen_quad();
        let indices = QUAD_INDICES.map(|n| n + starting_index).to_vec();
        (indices, quad)
    }
}

pub struct Triangle {
    pub points: [Vec2; 3],
    pub color: Color,
}

impl HasBounds for Triangle {
    fn bounds(&self) -> AABB {
        let min = self.points[0].min(self.points[1]).min(self.points[2]);
        let max = self.points[0].max(self.points[1]).max(self.points[2]);
        AABB::new(min, max)
    }
}

impl Shape for Triangle {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>) {
        let tri = self.points.map(|p| Vertex::new(p.x, p.y, self.color));
        let indices = starting_index..starting_index + 3;
        (indices.collect(), tri.to_vec())
    }
}

pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl HasBounds for Line {
    fn bounds(&self) -> AABB {
        let half_thick = self.thickness * 0.5;
        AABB::new(
            self.start.min(self.end) - Vec2::splat(half_thick),
            self.start.max(self.end) + Vec2::splat(half_thick),
        )
    }
}

impl Line {
    fn gen_mesh(&self) -> Option<Vec<Vertex>> {
        let direction = self.end - self.start;
        let length = direction.length();

        if length == 0.0 {
            return None;
        }

        let normalized = direction / length;
        let perpendicular = Vec2::new(-normalized.y, normalized.x) * self.thickness / 2.0;

        Some(vec![
            Vertex::new(
                self.start.x - perpendicular.x,
                self.start.y - perpendicular.y,
                self.color,
            ),
            Vertex::new(
                self.end.x - perpendicular.x,
                self.end.y - perpendicular.y,
                self.color,
            ),
            Vertex::new(
                self.start.x + perpendicular.x,
                self.start.y + perpendicular.y,
                self.color,
            ),
            Vertex::new(
                self.end.x + perpendicular.x,
                self.end.y + perpendicular.y,
                self.color,
            ),
        ])
    }
}

impl Shape for Line {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>) {
        if let Some(mesh) = self.gen_mesh() {
            (QUAD_INDICES.map(|n| n + starting_index).to_vec(), mesh)
        } else {
            (vec![], vec![])
        }
    }
}

pub struct Poly {
    pub sides: usize,
    pub radius: f32,
    pub center: Vec2,
    pub rotation: f32,
    pub color: Color,
}

impl HasBounds for Poly {
    fn bounds(&self) -> AABB {
        AABB::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
    }
}

impl Poly {
    pub fn gen_points(&self) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(self.sides);
        let angle_step = TAU / self.sides as f32;

        for i in 0..self.sides {
            let angle = angle_step * i as f32 + self.rotation;
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();
            points.push(Vec2::new(x, y));
        }

        points
    }

    pub fn gen_mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        let points = self.gen_points();
        gen_mesh_from_points(&points, self.color)
    }
}

impl Shape for Poly {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>) {
        let (vertices, indices) = self.gen_mesh();
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }
}

pub struct CustomShape {
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl HasBounds for CustomShape {
    fn bounds(&self) -> AABB {
        if self.points.is_empty() {
            return AABB::new(Vec2::ZERO, Vec2::ZERO);
        }

        let mut min = self.points[0];
        let mut max = self.points[0];

        for point in &self.points[1..] {
            min = min.min(*point);
            max = max.max(*point);
        }

        AABB::new(min, max)
    }
}

impl Shape for CustomShape {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>) {
        let (vertices, indices) = gen_mesh_from_points(&self.points, self.color);
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }
}

pub(crate) const QUAD_INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];

pub(crate) const UNIT_QUAD: [Vertex; 4] = [
    Vertex {
        position: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];

macro_rules! define_draw_functions {
    ($($name:ident: $($param:ident: $ptype:ty),* => $constructor:expr),*) => {
        $(
            pub fn $name($($param: $ptype),*) {
                let shape = $constructor;
                draw_shape(shape);
            }

            paste::item! {
                pub fn [<$name _world>]($($param: $ptype),*) {
                    let shape = $constructor;
                    draw_shape_world(shape);
                }
            }
        )*
    };
}

#[rustfmt::skip]
define_draw_functions!(
    draw_rect: top_left: Vec2, size: Vec2, color: Color => Rect { top_left, size, color },
    draw_square: top_left: Vec2, size: f32, color: Color => Rect { top_left, size: Vec2::splat(size), color },
    draw_tri: a: Vec2, b: Vec2, c: Vec2, color: Color => Triangle { points: [a, b, c], color },
    draw_line: start: Vec2, end: Vec2, thickness: f32, color: Color => Line { start, end, thickness, color },
    draw_poly: center: Vec2, sides: usize, radius: f32, rotation: f32, color: Color => Poly { center, sides, radius, rotation, color },
    draw_custom_shape: points: Vec<Vec2>, color: Color => CustomShape { points, color }
);

pub fn draw_circle(center: Vec2, radius: f32, color: Color) {
    draw_circle_internal(center, radius, color);
}

pub fn draw_circle_world(center: Vec2, radius: f32, color: Color) {
    draw_circle_world_internal(center, radius, color);
}

pub fn draw_shape(shape: impl Shape) {
    get_state().draw_queue.add_shape(shape);
}

pub fn draw_shape_world(shape: impl Shape) {
    if shape.is_visible_in_world() {
        get_state().world_draw_queue.add_shape(shape);
    }
}

pub fn draw_circle_internal(center: Vec2, radius: f32, color: Color) {
    get_state().draw_queue.add_circle(center, radius, color);
}

pub fn draw_circle_world_internal(center: Vec2, radius: f32, color: Color) {
    let circle = Circle {
        center,
        radius,
        color,
    };
    if circle.is_visible_in_world() {
        get_state()
            .world_draw_queue
            .add_circle(center, radius, color);
    }
}

fn gen_mesh_from_points(points: &[Vec2], color: Color) -> (Vec<Vertex>, Vec<u32>) {
    let num_points = points.len();
    let vertices: Vec<_> = points
        .iter()
        .map(|p| Vertex::new(p.x, p.y, color))
        .collect();

    let mut indices = Vec::new();
    for i in 1..(num_points - 1) as u32 {
        indices.extend_from_slice(&[0, i, i + 1]);
    }

    (vertices, indices)
}
