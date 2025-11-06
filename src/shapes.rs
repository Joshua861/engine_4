use crate::{EngineDisplay, Vertex, color::Color, get_state, programs::Programs};
use glam::{Mat4, Vec2};
use glium::{
    Blend, DrawParameters, Frame, IndexBuffer, Surface, VertexBuffer, implement_vertex, uniform,
};
use std::f32::consts::TAU;

// Instance data for circles
#[derive(Copy, Clone, Debug)]
pub struct CircleInstance {
    pub center: [f32; 2],
    pub radius: f32,
    pub color: [f32; 4],
}

implement_vertex!(CircleInstance, center, radius, color);

impl CircleInstance {
    fn new(center: Vec2, radius: f32, color: Color) -> Self {
        Self {
            center: [center.x, center.y],
            radius,
            color: color.for_gpu(),
        }
    }
}

// AABB for culling
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    pub fn expand(self, amount: f32) -> Self {
        Self {
            min: self.min - Vec2::splat(amount),
            max: self.max + Vec2::splat(amount),
        }
    }
}

pub(crate) const QUAD_INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];

// Unit quad for instancing (will be scaled by shader)
const UNIT_QUAD: [Vertex; 4] = [
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

pub trait Shape {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>);
}

pub(crate) struct Circle {
    pub center: Vec2,
    pub radius: f32,
    pub color: Color,
}

impl Circle {
    pub fn bounds(&self) -> AABB {
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
        let vertices: Vec<_> = points
            .iter()
            .map(|p| Vertex::new(p.x, p.y, self.color))
            .collect();

        let mut indices = Vec::new();
        for i in 1..(self.sides - 1) as u32 {
            indices.extend_from_slice(&[0, i, i + 1]);
        }

        (vertices, indices)
    }
}

impl Shape for Poly {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex>) {
        let (vertices, indices) = self.gen_mesh();
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }
}

// Represents a batch of draw calls that should be executed together
#[derive(Debug)]
enum DrawBatch {
    Shapes {
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    Circles {
        instances: Vec<CircleInstance>,
    },
}

pub struct DrawQueue {
    batches: Vec<DrawBatch>,
    current_shape_vertices: Vec<Vertex>,
    current_shape_indices: Vec<u32>,
    current_shape_max_index: u32,
    current_circles: Vec<CircleInstance>,
}

impl DrawQueue {
    pub fn empty() -> Self {
        Self {
            batches: vec![],
            current_shape_vertices: vec![],
            current_shape_indices: vec![],
            current_shape_max_index: 0,
            current_circles: vec![],
        }
    }

    fn flush_shapes(&mut self) {
        if !self.current_shape_vertices.is_empty() {
            self.batches.push(DrawBatch::Shapes {
                vertices: std::mem::take(&mut self.current_shape_vertices),
                indices: std::mem::take(&mut self.current_shape_indices),
            });
            self.current_shape_max_index = 0;
        }
    }

    fn flush_circles(&mut self) {
        if !self.current_circles.is_empty() {
            self.batches.push(DrawBatch::Circles {
                instances: std::mem::take(&mut self.current_circles),
            });
        }
    }

    pub fn add_shape(&mut self, shape: impl Shape) {
        // If we have pending circles, flush them first to maintain draw order
        self.flush_circles();

        let (mut indices, mut vertices) = shape.points(self.current_shape_max_index);
        self.current_shape_max_index += vertices.len() as u32;
        self.current_shape_vertices.append(&mut vertices);
        self.current_shape_indices.append(&mut indices);
    }

    pub fn add_circle(&mut self, center: Vec2, radius: f32, color: Color) {
        // If we have pending shapes, flush them first to maintain draw order
        self.flush_shapes();

        self.current_circles
            .push(CircleInstance::new(center, radius, color));
    }
}

pub fn is_world_shape_visible(bounds: AABB) -> bool {
    let camera = &mut get_state().camera;
    let (view_min, view_max) = camera.visible_bounds();
    let view_bounds = AABB::new(view_min, view_max);
    bounds.intersects(&view_bounds)
}

pub fn draw_shape(shape: impl Shape) {
    get_state().draw_queue.add_shape(shape);
}

pub fn draw_shape_world(shape: impl Shape) {
    get_state().world_draw_queue.add_shape(shape);
}

pub fn draw_circle_internal(center: Vec2, radius: f32, color: Color) {
    get_state().draw_queue.add_circle(center, radius, color);
}

pub fn draw_circle_world_internal(center: Vec2, radius: f32, color: Color) {
    let bounds = AABB::from_center_size(center, Vec2::splat(radius * 2.0));
    if !is_world_shape_visible(bounds) {
        return;
    }
    get_state()
        .world_draw_queue
        .add_circle(center, radius, color);
}

impl DrawQueue {
    pub fn draw(
        &mut self,
        frame: &mut Frame,
        display: &EngineDisplay,
        programs: &Programs,
        projection: &Mat4,
    ) {
        // Flush any remaining batches
        self.flush_shapes();
        self.flush_circles();

        // Draw all batches in order
        for batch in &self.batches {
            match batch {
                DrawBatch::Shapes { vertices, indices } => {
                    let vertex_buffer = VertexBuffer::new(display, vertices).unwrap();
                    let index_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        indices,
                    )
                    .unwrap();

                    let uniforms = uniform! {
                        transform: projection.to_cols_array_2d(),
                    };

                    let params = DrawParameters {
                        blend: Blend::alpha_blending(),
                        ..Default::default()
                    };

                    frame
                        .draw(
                            &vertex_buffer,
                            &index_buffer,
                            &programs.flat,
                            &uniforms,
                            &params,
                        )
                        .unwrap();
                }
                DrawBatch::Circles { instances } => {
                    let quad_buffer = VertexBuffer::new(display, &UNIT_QUAD).unwrap();
                    let instance_buffer = VertexBuffer::dynamic(display, instances).unwrap();
                    let index_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &QUAD_INDICES,
                    )
                    .unwrap();

                    let uniforms = uniform! {
                        transform: projection.to_cols_array_2d(),
                    };

                    let params = DrawParameters {
                        blend: Blend::alpha_blending(),
                        ..Default::default()
                    };

                    frame
                        .draw(
                            (&quad_buffer, instance_buffer.per_instance().unwrap()),
                            &index_buffer,
                            &programs.circle,
                            &uniforms,
                            &params,
                        )
                        .unwrap();
                }
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Self::empty()
    }
}
