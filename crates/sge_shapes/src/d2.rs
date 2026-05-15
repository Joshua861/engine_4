use dyn_clone::DynClone;
use sge_color::Color;
use sge_math::collision::{Aabb2d, HasBounds2D};
use sge_types::{ColorVertex2D, SdfInstance, SdfStroke};
use sge_vectors::{Mat3, Vec2, vec2};
use std::f32::consts::TAU;

pub trait Shape2D: HasBounds2D {
    fn sdf(&self) -> SdfInstance;
    fn is_visible_in_world(&self) -> bool {
        self.bounds().is_visible_in_world()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: Vec2,
    pub color: Color,
}

impl Shape2D for Circle {
    fn sdf(&self) -> SdfInstance {
        SdfInstance::ellipse(self.center, self.radius).with_fill_solid(self.color)
    }
}

impl HasBounds2D for Circle {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::from_center_size(self.center, self.radius * 2.0)
    }
}

impl Circle {
    pub fn encompassing_radius(&self) -> f32 {
        self.radius.x.max(self.radius.y)
    }

    pub fn from_top_left(top_left: Vec2, radius: Vec2, color: Color) -> Self {
        Self {
            center: top_left + radius,
            radius,
            color,
        }
    }

    pub fn from_diameter(a: Vec2, b: Vec2) -> Self {
        let center = (a + b) / 2.0;
        let radius = (b - a).abs() / 2.0;
        Self {
            center,
            radius,
            color: Color::WHITE,
        }
    }

    pub fn with_outline(self, outline_thickness: f32, outline_color: Color) -> CircleWithOutline {
        CircleWithOutline {
            center: self.center,
            radius: self.radius,
            fill_color: self.color,
            outline_thickness,
            outline_color,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_radius(mut self, radius: Vec2) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_radius_uniform(mut self, radius: f32) -> Self {
        self.radius = Vec2::splat(radius);
        self
    }

    pub fn with_center(mut self, center: Vec2) -> Self {
        self.center = center;
        self
    }

    pub fn new(center: Vec2, radius: Vec2, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CircleWithOutline {
    pub center: Vec2,
    pub radius: Vec2,
    pub outline_color: Color,
    pub outline_thickness: f32,
    pub fill_color: Color,
}

impl CircleWithOutline {
    pub fn from_top_left(
        top_left: Vec2,
        radius: Vec2,
        outline_color: Color,
        outline_thickness: f32,
        fill_color: Color,
    ) -> Self {
        Self {
            center: top_left + radius,
            radius,
            outline_color,
            outline_thickness,
            fill_color,
        }
    }

    pub fn new(
        center: Vec2,
        radius: Vec2,
        outline_color: Color,
        outline_thickness: f32,
        fill_color: Color,
    ) -> Self {
        Self {
            center,
            radius,
            outline_color,
            outline_thickness,
            fill_color,
        }
    }
}

impl HasBounds2D for CircleWithOutline {
    fn bounds(&self) -> Aabb2d {
        let total_radius = self.radius + Vec2::splat(self.outline_thickness);
        Aabb2d::from_center_size(self.center, total_radius * 2.0)
    }
}

impl Shape2D for CircleWithOutline {
    fn sdf(&self) -> SdfInstance {
        SdfInstance::ellipse(self.center, self.radius)
            .with_fill_solid(self.fill_color)
            .with_stroke(
                self.outline_thickness,
                self.outline_color,
                SdfStroke::Inside,
            )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RoundedRectangle {
    pub top_left: Vec2,
    pub size: Vec2,
    pub fill_color: Color,
    pub corner_radius: f32,
    pub outline_thickness: f32,
    pub outline_color: Color,
}

impl HasBounds2D for RoundedRectangle {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::new(self.top_left, self.top_left + self.size)
    }
}

impl Shape2D for RoundedRectangle {
    fn sdf(&self) -> SdfInstance {
        SdfInstance::rect(self.center(), self.size)
            .with_corner_radius(self.corner_radius)
            .with_fill_solid(self.fill_color)
            .with_stroke(
                self.outline_thickness,
                self.outline_color,
                SdfStroke::Inside,
            )
    }
}

impl RoundedRectangle {
    pub fn new(top_left: Vec2, size: Vec2, color: Color, corner_radius: f32) -> Self {
        Self {
            top_left,
            size,
            fill_color: color,
            corner_radius,
            outline_color: color,
            outline_thickness: 0.0,
        }
    }

    pub fn square(top_left: Vec2, size: f32, color: Color, corner_radius: f32) -> Self {
        Self::new(top_left, Vec2::splat(size), color, corner_radius)
    }

    pub fn square_with_outline(
        top_left: Vec2,
        size: f32,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self::with_outline(
            top_left,
            Vec2::splat(size),
            color,
            corner_radius,
            outline_thickness,
            outline_color,
        )
    }

    pub fn with_outline(
        top_left: Vec2,
        size: Vec2,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            top_left,
            size,
            fill_color: color,
            corner_radius,
            outline_thickness,
            outline_color,
        }
    }

    pub fn from_center(center: Vec2, size: Vec2, color: Color, corner_radius: f32) -> Self {
        Self::new(center - size / 2.0, size, color, corner_radius)
    }

    pub fn from_center_with_outline(
        center: Vec2,
        size: Vec2,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self::with_outline(
            center - size / 2.0,
            size,
            color,
            corner_radius,
            outline_thickness,
            outline_color,
        )
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub rot: f32,
}

impl HasBounds2D for Rect {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::new(self.top_left, self.top_left + self.size)
    }
}

impl Rect {
    pub fn new(top_left: Vec2, size: Vec2, color: Color) -> Self {
        Self {
            top_left,
            size,
            color,
            rot: 0.0,
        }
    }

    pub fn with_rotation(mut self, rot: f32) -> Self {
        self.rot = rot;
        self
    }

    pub fn from_center(center: Vec2, size: Vec2, color: Color) -> Self {
        Self::new(center - size / 2.0, size, color)
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }

    pub fn new_square(top_left: Vec2, size: f32, color: Color) -> Self {
        Self::new(top_left, Vec2::splat(size), color)
    }

    pub fn new_size(size: Vec2) -> Self {
        Self::new(Vec2::ZERO, size, Color::WHITE)
    }

    pub fn from_square_center(center: Vec2, size: f32, color: Color) -> Self {
        Self::from_center(center, Vec2::splat(size), color)
    }

    pub fn points(&self) -> [Vec2; 4] {
        if self.rot == 0.0 {
            let tl = self.top_left;
            let br = self.top_left + self.size;
            let tr = vec2(br.x, tl.y);
            let bl = vec2(tl.x, br.y);

            [tl, tr, br, bl]
        } else {
            let rot = self.rot;
            let half_size = self.size / 2.0;
            let mat = Mat3::from_translation(self.top_left + half_size) * Mat3::from_angle(rot);
            let x = half_size.x;
            let y = half_size.y;

            [vec2(-x, -y), vec2(x, -y), vec2(x, y), vec2(-x, y)].map(|v| mat.transform_point2(v))
        }
    }

    pub fn tri_points(&self) -> [Vec2; 4] {
        if self.rot == 0.0 {
            #[cfg(not(feature = "round_coords"))]
            let top_left = self.top_left;
            #[cfg(feature = "round_coords")]
            let top_left = self.top_left.round();

            #[cfg(not(feature = "round_coords"))]
            let size = self.size;
            #[cfg(feature = "round_coords")]
            let size = self.size.round();

            let tl = top_left;
            let br = top_left + size;
            let tr = vec2(br.x, tl.y);
            let bl = vec2(tl.x, br.y);

            [tl, tr, bl, br]
        } else {
            let rot = self.rot;
            let half_size = self.size / 2.0;
            let mat = Mat3::from_translation(self.top_left + half_size) * Mat3::from_angle(rot);
            let x = half_size.x;
            let y = half_size.y;

            [vec2(-x, -y), vec2(x, -y), vec2(-x, y), vec2(x, y)].map(|v| mat.transform_point2(v))
        }
    }

    pub fn gen_quad(&self) -> Vec<ColorVertex2D> {
        self.tri_points()
            .iter()
            .map(|p| ColorVertex2D::new(p.x, p.y, self.color))
            .collect()
    }
}

impl Shape2D for Rect {
    fn sdf(&self) -> SdfInstance {
        SdfInstance::rect(self.center(), self.size).with_fill_solid(self.color)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Vec2; 3],
    pub color: Color,
    pub rot: f32,
}

impl Triangle {
    pub fn points(&self) -> [Vec2; 3] {
        self.rotated_points()
    }

    pub fn new(points: [Vec2; 3], color: Color) -> Self {
        Self {
            points,
            color,
            rot: 0.0,
        }
    }

    pub fn with_rotation(mut self, rot: f32) -> Self {
        self.rot = rot;
        self
    }

    pub fn center(&self) -> Vec2 {
        (self.points[0] + self.points[1] + self.points[2]) / 3.0
    }

    fn rotated_points(&self) -> [Vec2; 3] {
        if self.rot == 0.0 {
            return self.points;
        }
        let center = self.center();
        let mat = Mat3::from_translation(center)
            * Mat3::from_angle(self.rot)
            * Mat3::from_translation(-center);
        let points = self.points.map(|p| mat.transform_point2(p));

        #[cfg(not(feature = "round_coords"))]
        return points;
        #[cfg(feature = "round_coords")]
        return points.map(|p| p.round());
    }
}

impl HasBounds2D for Triangle {
    fn bounds(&self) -> Aabb2d {
        let pts = self.rotated_points();
        let min = pts[0].min(pts[1]).min(pts[2]);
        let max = pts[0].max(pts[1]).max(pts[2]);
        Aabb2d::new(min, max)
    }
}

impl Shape2D for Triangle {
    fn sdf(&self) -> SdfInstance {
        SdfInstance::triangle(self.points[0], self.points[1], self.points[2])
            .with_fill_solid(self.color)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line2D {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl Line2D {
    pub fn new(start: Vec2, end: Vec2, thickness: f32, color: Color) -> Self {
        Self {
            start,
            end,
            thickness,
            color,
        }
    }

    pub fn center(&self) -> Vec2 {
        (self.start + self.end) / 2.0
    }

    pub fn with_caps(mut self) -> Self {
        self.add_caps();
        self
    }

    pub fn add_caps(&mut self) {
        let dir = (self.end - self.start).normalize();
        let half = dir * self.thickness / 2.0;
        self.start -= half;
        self.end += half;
    }

    pub fn add_half_caps(&mut self) {
        let dir = (self.end - self.start).normalize();
        let half = dir * self.thickness / 4.0;
        self.start -= half;
        self.end += half;
    }

    pub fn with_half_caps(mut self) -> Self {
        self.add_half_caps();
        self
    }
}

impl HasBounds2D for Line2D {
    fn bounds(&self) -> Aabb2d {
        let half_thick = self.thickness * 0.5;
        Aabb2d::new(
            self.start.min(self.end) - Vec2::splat(half_thick),
            self.start.max(self.end) + Vec2::splat(half_thick),
        )
    }
}

impl Line2D {
    fn points(&self) -> [Vec2; 4] {
        let (start, end) = (self.start, self.end);
        #[cfg(feature = "round_coords")]
        let start = start.round();
        #[cfg(feature = "round_coords")]
        let end = end.round();
        let direction = end - start;
        let length = direction.length();

        let normalized = direction / length;
        let perpendicular = Vec2::new(-normalized.y, normalized.x) * self.thickness / 2.0;

        [
            Vec2::new(start.x - perpendicular.x, start.y - perpendicular.y),
            Vec2::new(end.x - perpendicular.x, end.y - perpendicular.y),
            Vec2::new(start.x + perpendicular.x, start.y + perpendicular.y),
            Vec2::new(end.x + perpendicular.x, end.y + perpendicular.y),
        ]
    }
}

impl Shape2D for Line2D {
    fn sdf(&self) -> SdfInstance {
        let [a, b, c, d] = self.points();
        SdfInstance::quad(a, b, c, d).with_fill_solid(self.color)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Poly {
    pub sides: usize,
    pub radius: f32,
    pub center: Vec2,
    pub rotation: f32,
    pub color: Color,
}

impl HasBounds2D for Poly {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
    }
}

impl Poly {
    pub fn new(center: Vec2, radius: f32, sides: usize, color: Color) -> Self {
        Self {
            center,
            radius,
            sides,
            rotation: 0.0,
            color,
        }
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn gen_points(&self) -> Vec<Vec2> {
        #[cfg(feature = "round_coords")]
        let center = self.center.round();
        #[cfg(not(feature = "round_coords"))]
        let center = self.center;
        #[cfg(feature = "round_coords")]
        let radius = self.radius.round();
        #[cfg(not(feature = "round_coords"))]
        let radius = self.radius;

        let mut points = Vec::with_capacity(self.sides);
        let angle_step = TAU / self.sides as f32;

        for i in 0..self.sides {
            let angle = angle_step * i as f32 + self.rotation;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            points.push(Vec2::new(x, y));
        }

        points
    }

    pub fn gen_mesh(&self) -> (Vec<ColorVertex2D>, Vec<u32>) {
        let points = self.gen_points();
        gen_mesh_from_points(&points, self.color)
    }
}

impl Shape2D for Poly {
    fn sdf(&self) -> SdfInstance {
        SdfInstance::star(self.center, self.radius, self.sides as f32, 1.0)
            .with_fill_solid(self.color)
            .with_rotation(self.rotation)
    }
}

#[derive(Clone, Debug)]
pub struct Sector {
    pub center: Vec2,
    pub radius: Vec2,
    pub fill_color: Color,
    pub outline_thickness: f32,
    pub outline_color: Color,
    pub start_angle: f32,
    pub end_angle: f32,
}

impl HasBounds2D for Sector {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::from_center_size(self.center, self.radius * 2.0)
    }
}

impl Shape2D for Sector {
    fn sdf(&self) -> SdfInstance {
        SdfInstance::sector(self.center, self.radius, self.start_angle, self.end_angle)
            .with_fill_solid(self.fill_color)
            .with_stroke(
                self.outline_thickness,
                self.outline_color,
                SdfStroke::Inside,
            )
    }
}

pub const QUAD_INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];

pub const UNIT_QUAD: [ColorVertex2D; 4] = [
    ColorVertex2D {
        position: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    ColorVertex2D {
        position: [1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    ColorVertex2D {
        position: [-1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    ColorVertex2D {
        position: [1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];

pub fn gen_mesh_from_points(points: &[Vec2], color: Color) -> (Vec<ColorVertex2D>, Vec<u32>) {
    if points.len() < 3 {
        return (vec![], vec![]);
    }

    let mut polygon_builder = lyon::tessellation::path::Path::builder();
    #[cfg(not(feature = "round_coords"))]
    polygon_builder.begin(lyon::math::point(points[0].x, points[0].y));
    #[cfg(feature = "round_coords")]
    polygon_builder.begin(lyon::math::point(points[0].x.round(), points[0].y.round()));
    for point in &points[1..] {
        #[cfg(not(feature = "round_coords"))]
        polygon_builder.line_to(lyon::math::point(point.x, point.y));
        #[cfg(feature = "round_coords")]
        polygon_builder.line_to(lyon::math::point(point.x.round(), point.y.round()));
    }
    polygon_builder.end(false);
    let polygon = polygon_builder.build();

    struct VertexConstructor {
        color: Color,
    }

    impl lyon::tessellation::FillVertexConstructor<ColorVertex2D> for VertexConstructor {
        fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> ColorVertex2D {
            let pos = vertex.position();
            ColorVertex2D::new(pos.x, pos.y, self.color)
        }
    }

    let mut tessellator = lyon::tessellation::FillTessellator::new();
    let mut buffers = lyon::tessellation::VertexBuffers::<ColorVertex2D, u32>::new();

    tessellator
        .tessellate_path(
            &polygon,
            &lyon::tessellation::FillOptions::non_zero(),
            &mut lyon::tessellation::BuffersBuilder::new(&mut buffers, VertexConstructor { color }),
        )
        .unwrap();

    let vertices = buffers.vertices;
    let indices = buffers.indices;

    (vertices, indices)
}
