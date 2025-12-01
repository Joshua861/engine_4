use crate::{
    collisions::{AABB2D, HasBounds2D},
    color::Color,
    draw_queue_2d::{DrawQueue2D, Vertex2D},
    get_state,
};
use bevy_math::{BVec2, Mat4, Quat, Vec2};
use std::f32::consts::TAU;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Shape2D: HasBounds2D {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>);
    fn is_visible_in_world(&self) -> bool {
        self.bounds().is_visible_in_world()
    }
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D);
    fn draw(&self) {
        self.add_to_draw_queue(get_state().draw_queue_2d())
    }
    fn draw_world(&self) {
        if self.is_visible_in_world() {
            self.add_to_draw_queue(get_state().world_draw_queue_2d())
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: Vec2,
    pub color: Color,
}

impl Shape2D for Circle {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_circle(self.center, self.radius, self.color);
    }

    fn points(&self, _starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

impl HasBounds2D for Circle {
    fn bounds(&self) -> AABB2D {
        AABB2D::from_center_size(self.center, self.radius * 2.0)
    }
}

impl Circle {
    pub fn encompassing_radius(&self) -> f32 {
        self.radius.x.max(self.radius.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CircleOutline {
    pub center: Vec2,
    pub radius: Vec2,
    pub color: Color,
    pub thickness: f32,
}

impl HasBounds2D for CircleOutline {
    fn bounds(&self) -> AABB2D {
        let total_radius = self.radius + Vec2::splat(self.thickness);
        AABB2D::from_center_size(self.center, total_radius * 2.0)
    }
}

impl Shape2D for CircleOutline {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_circle_with_outline(
            self.center,
            self.radius,
            Color::TRANSPARENT,
            self.thickness,
            self.color,
        );
    }

    fn points(&self, _starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl HasBounds2D for Rect {
    fn bounds(&self) -> AABB2D {
        AABB2D::new(self.top_left, self.top_left + self.size)
    }
}

impl Rect {
    fn gen_quad(&self) -> Vec<Vertex2D> {
        let tl = self.top_left;
        let br = self.top_left + self.size;

        vec![
            Vertex2D::new(tl.x, tl.y, self.color),
            Vertex2D::new(br.x, tl.y, self.color),
            Vertex2D::new(tl.x, br.y, self.color),
            Vertex2D::new(br.x, br.y, self.color),
        ]
    }
}

impl Shape2D for Rect {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let quad = self.gen_quad();
        let indices = QUAD_INDICES.map(|n| n + starting_index).to_vec();
        (indices, quad)
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Vec2; 3],
    pub color: Color,
}

impl HasBounds2D for Triangle {
    fn bounds(&self) -> AABB2D {
        let min = self.points[0].min(self.points[1]).min(self.points[2]);
        let max = self.points[0].max(self.points[1]).max(self.points[2]);
        AABB2D::new(min, max)
    }
}

impl Shape2D for Triangle {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let tri = self.points.map(|p| Vertex2D::new(p.x, p.y, self.color));
        let indices = starting_index..starting_index + 3;
        (indices.collect(), tri.to_vec())
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl HasBounds2D for Line {
    fn bounds(&self) -> AABB2D {
        let half_thick = self.thickness * 0.5;
        AABB2D::new(
            self.start.min(self.end) - Vec2::splat(half_thick),
            self.start.max(self.end) + Vec2::splat(half_thick),
        )
    }
}

impl Line {
    fn gen_mesh(&self) -> Option<Vec<Vertex2D>> {
        let direction = self.end - self.start;
        let length = direction.length();

        if length == 0.0 {
            return None;
        }

        let normalized = direction / length;
        let perpendicular = Vec2::new(-normalized.y, normalized.x) * self.thickness / 2.0;

        Some(vec![
            Vertex2D::new(
                self.start.x - perpendicular.x,
                self.start.y - perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.end.x - perpendicular.x,
                self.end.y - perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.start.x + perpendicular.x,
                self.start.y + perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.end.x + perpendicular.x,
                self.end.y + perpendicular.y,
                self.color,
            ),
        ])
    }
}

impl Shape2D for Line {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        if let Some(mesh) = self.gen_mesh() {
            (QUAD_INDICES.map(|n| n + starting_index).to_vec(), mesh)
        } else {
            (vec![], vec![])
        }
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
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
    fn bounds(&self) -> AABB2D {
        AABB2D::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
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

    pub fn gen_mesh(&self) -> (Vec<Vertex2D>, Vec<u32>) {
        let points = self.gen_points();
        gen_mesh_from_points(&points, self.color)
    }
}

impl Shape2D for Poly {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let (vertices, indices) = self.gen_mesh();
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

#[derive(Clone, Debug)]
pub struct CustomShape {
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl HasBounds2D for CustomShape {
    fn bounds(&self) -> AABB2D {
        if self.points.is_empty() {
            return AABB2D::new(Vec2::ZERO, Vec2::ZERO);
        }

        let mut min = self.points[0];
        let mut max = self.points[0];

        for point in &self.points[1..] {
            min = min.min(*point);
            max = max.max(*point);
        }

        AABB2D::new(min, max)
    }
}

impl Shape2D for CustomShape {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let (vertices, indices) = gen_mesh_from_points(&self.points, self.color);
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

pub(crate) const QUAD_INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];

pub(crate) const UNIT_QUAD: [Vertex2D; 4] = [
    Vertex2D {
        position: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [-1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];

macro_rules! define_draw_functions {
    ($($name:ident: $($param:ident: $ptype:ty),* => $constructor:expr),*,) => {
        $(
            pub fn $name($($param: $ptype),*) {
                let shape = $constructor;
                draw_shape(&shape);
            }

            paste::item! {
                pub fn [<$name _world>]($($param: $ptype),*) {
                    let shape = $constructor;
                    draw_shape_world(&shape);
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
    draw_custom_shape: points: Vec<Vec2>, color: Color => CustomShape { points, color },
    draw_hexagon: center: Vec2, radius: f32, color: Color => Poly { center, sides: 6, radius, rotation: 0.0, color },
    draw_hexagon_pointy: center: Vec2, radius: f32, color: Color => Poly { center, sides: 6, radius, rotation: std::f32::consts::FRAC_PI_6, color },
);

pub fn draw_circle(center: Vec2, radius: f32, color: Color) {
    get_state()
        .draw_queue_2d()
        .add_circle(center, Vec2::splat(radius), color);
}

pub fn draw_circle_world(center: Vec2, radius: f32, color: Color) {
    let circle = Circle {
        center,
        radius: Vec2::splat(radius),
        color,
    };
    if circle.bounds().is_visible_in_world() {
        get_state()
            .world_draw_queue_2d()
            .add_circle(center, Vec2::splat(radius), color);
    }
}

pub fn draw_ellipse(center: Vec2, radius: Vec2, color: Color) {
    get_state()
        .draw_queue_2d()
        .add_circle(center, radius, color);
}

pub fn draw_ellipse_world(center: Vec2, radius: Vec2, color: Color) {
    let circle = Circle {
        center,
        radius,
        color,
    };
    if circle.bounds().is_visible_in_world() {
        get_state()
            .world_draw_queue_2d()
            .add_circle(center, radius, color);
    }
}

pub fn draw_circle_outline(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
    get_state().draw_queue_2d().add_circle_with_outline(
        center,
        Vec2::splat(radius),
        outline_color.with_alpha(0.0),
        thickness,
        outline_color,
    );
}

pub fn draw_circle_outline_world(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
    let circle = Circle {
        center,
        radius: Vec2::splat(radius + thickness),
        color: outline_color,
    };
    if circle.bounds().is_visible_in_world() {
        get_state().world_draw_queue_2d().add_circle_with_outline(
            center,
            Vec2::splat(radius),
            Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
            thickness,
            outline_color,
        );
    }
}

pub fn draw_ellipse_outline(center: Vec2, radius: Vec2, outline_color: Color, thickness: f32) {
    get_state().draw_queue_2d().add_circle_with_outline(
        center,
        radius,
        Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
        thickness,
        outline_color,
    );
}

pub fn draw_ellipse_outline_world(
    center: Vec2,
    radius: Vec2,
    outline_color: Color,
    thickness: f32,
) {
    let circle = Circle {
        center,
        radius: radius + Vec2::splat(thickness),
        color: outline_color,
    };
    if circle.bounds().is_visible_in_world() {
        get_state().world_draw_queue_2d().add_circle_with_outline(
            center,
            radius,
            Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
            thickness,
            outline_color,
        );
    }
}

pub fn draw_circle_with_outline(
    center: Vec2,
    radius: f32,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    get_state().draw_queue_2d().add_circle_with_outline(
        center,
        Vec2::splat(radius),
        fill,
        thickness,
        outline,
    );
}

pub fn draw_circle_with_outline_world(
    center: Vec2,
    radius: f32,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    let circle = Circle {
        center,
        radius: Vec2::splat(radius + thickness),
        color: fill,
    };
    if circle.bounds().is_visible_in_world() {
        get_state().world_draw_queue_2d().add_circle_with_outline(
            center,
            Vec2::splat(radius),
            fill,
            thickness,
            outline,
        );
    }
}

pub fn draw_ellipse_with_outline(
    center: Vec2,
    radius: Vec2,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    get_state()
        .draw_queue_2d()
        .add_circle_with_outline(center, radius, fill, thickness, outline);
}

pub fn draw_ellipse_with_outline_world(
    center: Vec2,
    radius: Vec2,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    let circle = Circle {
        center,
        radius: radius + Vec2::splat(thickness),
        color: fill,
    };
    if circle.bounds().is_visible_in_world() {
        get_state()
            .world_draw_queue_2d()
            .add_circle_with_outline(center, radius, fill, thickness, outline);
    }
}

pub fn draw_shape(shape: &impl Shape2D) {
    shape.draw();
}

pub fn draw_shape_world(shape: &impl Shape2D) {
    shape.draw_world();
}

fn gen_mesh_from_points(points: &[Vec2], color: Color) -> (Vec<Vertex2D>, Vec<u32>) {
    if points.len() < 3 {
        return (vec![], vec![]);
    }

    let mut polygon_builder = lyon::tessellation::path::Path::builder();
    polygon_builder.begin(lyon::math::point(points[0].x, points[0].y));
    for point in &points[1..] {
        polygon_builder.line_to(lyon::math::point(point.x, point.y));
    }
    polygon_builder.end(false);
    let polygon = polygon_builder.build();

    struct VertexConstructor {
        color: Color,
    }

    impl lyon::tessellation::FillVertexConstructor<Vertex2D> for VertexConstructor {
        fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> Vertex2D {
            let pos = vertex.position();
            Vertex2D::new(pos.x, pos.y, self.color)
        }
    }

    let mut tessellator = lyon::tessellation::FillTessellator::new();
    let mut buffers = lyon::tessellation::VertexBuffers::<Vertex2D, u32>::new();

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

#[derive(Clone, Copy)]
pub struct Transform2D {
    mat: Mat4,
    dirty: bool,
    scale: Vec2,
    rotation: f32,
    translation: Vec2,
    mirror: BVec2,
}

impl Transform2D {
    pub const IDENTITY: Self = Self {
        mat: Mat4::IDENTITY,
        dirty: false,
        scale: Vec2::ONE,
        rotation: 0.0,
        translation: Vec2::ZERO,
        mirror: BVec2::FALSE,
    };

    pub fn update_matrix(&mut self) {
        if !self.dirty {
            return;
        }

        let effective_scale = Vec2::new(
            if self.mirror.x {
                -self.scale.x
            } else {
                self.scale.x
            },
            if self.mirror.y {
                -self.scale.y
            } else {
                self.scale.y
            },
        );

        self.mat = Mat4::from_scale_rotation_translation(
            effective_scale.extend(1.0),
            Quat::from_rotation_z(self.rotation),
            self.translation.extend(0.0),
        );
    }

    pub fn matrix(&mut self) -> Mat4 {
        self.update_matrix();
        self.mat
    }

    fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    pub fn scale_mut(&mut self) -> &mut Vec2 {
        self.mark_dirty();
        &mut self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        self.mark_dirty();
    }

    pub fn from_scale(scale: Vec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn scale_by(&mut self, scale: Vec2) {
        self.scale *= scale;
        self.mark_dirty();
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn rotation_mut(&mut self) -> &mut f32 {
        self.mark_dirty();
        &mut self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.mark_dirty();
    }

    pub fn from_rotation(rotation: f32) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn rotate_by(&mut self, rotation: f32) {
        self.rotation += rotation;
        self.mark_dirty();
    }

    pub fn translation(&self) -> Vec2 {
        self.translation
    }

    pub fn translation_mut(&mut self) -> &mut Vec2 {
        self.mark_dirty();
        &mut self.translation
    }

    pub fn set_translation(&mut self, translation: Vec2) {
        self.translation = translation;
        self.mark_dirty();
    }

    pub fn from_translation(translation: Vec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.translation = translation;
        transform
    }

    pub fn with_translation(mut self, translation: Vec2) -> Self {
        self.translation = translation;
        self
    }

    pub fn translate_by(&mut self, translation: Vec2) {
        self.translation += translation;
        self.mark_dirty();
    }

    pub fn mirror(&self) -> BVec2 {
        self.mirror
    }

    pub fn mirror_mut(&mut self) -> &mut BVec2 {
        self.mark_dirty();
        &mut self.mirror
    }

    pub fn set_mirror(&mut self, mirror: BVec2) {
        self.mirror = mirror;
        self.mark_dirty();
    }

    pub fn from_mirror(mirror: BVec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.mirror = mirror;
        transform
    }

    pub fn with_mirror(mut self, mirror: BVec2) -> Self {
        self.mirror = mirror;
        self
    }

    pub fn mirror_x(&mut self) {
        self.mirror.x = !self.mirror.x;
        self.mark_dirty();
    }

    pub fn mirror_y(&mut self) {
        self.mirror.y = !self.mirror.y;
        self.mark_dirty();
    }

    pub fn translate_x(&mut self, x: f32) {
        self.translation.x += x;
        self.mark_dirty();
    }

    pub fn translate_y(&mut self, y: f32) {
        self.translation.y += y;
        self.mark_dirty();
    }

    pub fn translation_x(&self) -> f32 {
        self.translation.x
    }

    pub fn translation_y(&self) -> f32 {
        self.translation.y
    }

    pub fn set_translation_x(&mut self, x: f32) {
        self.translation.x = x;
        self.mark_dirty();
    }

    pub fn set_translation_y(&mut self, y: f32) {
        self.translation.y = y;
        self.mark_dirty();
    }

    pub fn scale_x(&self) -> f32 {
        self.scale.x
    }

    pub fn scale_y(&self) -> f32 {
        self.scale.y
    }

    pub fn set_scale_x(&mut self, x: f32) {
        self.scale.x = x;
        self.mark_dirty();
    }

    pub fn set_scale_y(&mut self, y: f32) {
        self.scale.y = y;
        self.mark_dirty();
    }

    pub fn scale_by_x(&mut self, x: f32) {
        self.scale.x *= x;
        self.mark_dirty();
    }

    pub fn scale_by_y(&mut self, y: f32) {
        self.scale.y *= y;
        self.mark_dirty();
    }

    pub fn from_scale_rotation(scale: Vec2, rotation: f32) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform
    }

    pub fn from_scale_translation(scale: Vec2, translation: Vec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.translation = translation;
        transform
    }

    pub fn from_scale_mirror(scale: Vec2, mirror: BVec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.mirror = mirror;
        transform
    }

    pub fn from_rotation_translation(rotation: f32, translation: Vec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform.translation = translation;
        transform
    }

    pub fn from_rotation_mirror(rotation: f32, mirror: BVec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_translation_mirror(translation: Vec2, mirror: BVec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_scale_rotation_translation(scale: Vec2, rotation: f32, translation: Vec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.translation = translation;
        transform
    }

    pub fn from_scale_rotation_mirror(scale: Vec2, rotation: f32, mirror: BVec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_scale_translation_mirror(scale: Vec2, translation: Vec2, mirror: BVec2) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_rotation_translation_mirror(
        rotation: f32,
        translation: Vec2,
        mirror: BVec2,
    ) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_scale_rotation_translation_mirror(
        scale: Vec2,
        rotation: f32,
        translation: Vec2,
        mirror: BVec2,
    ) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn translated_by(mut self, translation: Vec2) -> Self {
        self.translation += translation;
        self.mark_dirty();
        self
    }

    pub fn scaled_by(mut self, scale: Vec2) -> Self {
        self.scale *= scale;
        self.mark_dirty();
        self
    }

    pub fn rotated_by(mut self, rotation: f32) -> Self {
        self.rotation += rotation;
        self.mark_dirty();
        self
    }

    pub fn mirrored_x(mut self) -> Self {
        self.mirror.x = !self.mirror.x;
        self.mark_dirty();
        self
    }

    pub fn mirrored_y(mut self) -> Self {
        self.mirror.y = !self.mirror.y;
        self.mark_dirty();
        self
    }

    pub fn mirrored_by(mut self, mirror: BVec2) -> Self {
        self.mirror = BVec2::new(self.mirror.x ^ mirror.x, self.mirror.y ^ mirror.y);
        self.mark_dirty();
        self
    }
}

impl Add for Transform2D {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.scale += rhs.scale;
        self.rotation += rhs.rotation;
        self.translation += rhs.translation;
        self.mirror = BVec2::new(self.mirror.x ^ rhs.mirror.x, self.mirror.y ^ rhs.mirror.y);
        self.mark_dirty();
        self
    }
}

impl AddAssign for Transform2D {
    fn add_assign(&mut self, rhs: Self) {
        self.scale += rhs.scale;
        self.rotation += rhs.rotation;
        self.translation += rhs.translation;
        self.mirror = BVec2::new(self.mirror.x ^ rhs.mirror.x, self.mirror.y ^ rhs.mirror.y);
        self.mark_dirty();
    }
}

impl Sub for Transform2D {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.scale -= rhs.scale;
        self.rotation -= rhs.rotation;
        self.translation -= rhs.translation;
        self.mirror = BVec2::new(self.mirror.x ^ rhs.mirror.x, self.mirror.y ^ rhs.mirror.y);
        self.mark_dirty();
        self
    }
}

impl SubAssign for Transform2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.scale -= rhs.scale;
        self.rotation -= rhs.rotation;
        self.translation -= rhs.translation;
        self.mirror = BVec2::new(self.mirror.x ^ rhs.mirror.x, self.mirror.y ^ rhs.mirror.y);
        self.mark_dirty();
    }
}

impl Mul<f32> for Transform2D {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.scale *= rhs;
        self.rotation *= rhs;
        self.translation *= rhs;
        self.mark_dirty();
        self
    }
}

impl MulAssign<f32> for Transform2D {
    fn mul_assign(&mut self, rhs: f32) {
        self.scale *= rhs;
        self.rotation *= rhs;
        self.translation *= rhs;
        self.mark_dirty();
    }
}

impl Div<f32> for Transform2D {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self.scale /= rhs;
        self.rotation /= rhs;
        self.translation /= rhs;
        self.mark_dirty();
        self
    }
}

impl DivAssign<f32> for Transform2D {
    fn div_assign(&mut self, rhs: f32) {
        self.scale /= rhs;
        self.rotation /= rhs;
        self.translation /= rhs;
        self.mark_dirty();
    }
}

impl Mul for Transform2D {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        let lhs_mat = self.matrix();
        let mut rhs_copy = rhs;
        let rhs_mat = rhs_copy.matrix();

        self.mat = lhs_mat * rhs_mat;

        self.scale *= rhs.scale;

        self.rotation += rhs.rotation;

        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        let rotated_translation = Vec2::new(
            rhs.translation.x * cos - rhs.translation.y * sin,
            rhs.translation.x * sin + rhs.translation.y * cos,
        );
        self.translation += rotated_translation * self.scale;

        self.mirror = BVec2::new(self.mirror.x ^ rhs.mirror.x, self.mirror.y ^ rhs.mirror.y);

        self.mark_dirty();
        self
    }
}

impl MulAssign for Transform2D {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
