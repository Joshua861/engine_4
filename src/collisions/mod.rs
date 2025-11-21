use bevy_math::Vec2;

pub mod ray;

pub trait IntersectsWith<T> {
    fn intersects_with(&self, other: &T) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub center: Vec2,
    pub half_size: f32,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<Vec2>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub position: Vec2,
}

impl Point {
    pub fn new(p: Vec2) -> Self {
        Self { position: p }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AABB2D {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB2D {
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

    pub fn intersects(&self, other: &AABB2D) -> bool {
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

    pub fn is_visible_in_world(&self) -> bool {
        let camera = &mut get_state().camera_2d;
        let (view_min, view_max) = camera.visible_bounds();
        let view_bounds = AABB2D::new(view_min, view_max);
        self.intersects(&view_bounds)
    }
}

pub trait HasBounds2D {
    fn bounds(&self) -> AABB2D;
}

impl HasBounds2D for Circle {
    fn bounds(&self) -> AABB2D {
        AABB2D::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
    }
}

impl HasBounds2D for Square {
    fn bounds(&self) -> AABB2D {
        AABB2D::from_center_size(self.center, Vec2::splat(self.half_size * 2.0))
    }
}

impl HasBounds2D for Polygon {
    fn bounds(&self) -> AABB2D {
        if self.vertices.is_empty() {
            return AABB2D::new(Vec2::ZERO, Vec2::ZERO);
        }

        let mut min = self.vertices[0];
        let mut max = self.vertices[0];

        for vertex in &self.vertices[1..] {
            min = min.min(*vertex);
            max = max.max(*vertex);
        }

        AABB2D::new(min, max)
    }
}

impl HasBounds2D for Point {
    fn bounds(&self) -> AABB2D {
        AABB2D::from_center_size(self.position, Vec2::ZERO)
    }
}

impl IntersectsWith<Circle> for Circle {
    fn intersects_with(&self, other: &Circle) -> bool {
        let distance_squared = self.center.distance_squared(other.center);
        let radius_sum = self.radius + other.radius;
        distance_squared <= radius_sum * radius_sum
    }
}

impl IntersectsWith<Square> for Circle {
    fn intersects_with(&self, square: &Square) -> bool {
        let closest = Vec2::new(
            self.center.x.clamp(
                square.center.x - square.half_size,
                square.center.x + square.half_size,
            ),
            self.center.y.clamp(
                square.center.y - square.half_size,
                square.center.y + square.half_size,
            ),
        );

        let distance_squared = self.center.distance_squared(closest);
        distance_squared <= self.radius * self.radius
    }
}

impl IntersectsWith<Circle> for Square {
    fn intersects_with(&self, circle: &Circle) -> bool {
        circle.intersects_with(self)
    }
}

impl IntersectsWith<Point> for Circle {
    fn intersects_with(&self, point: &Point) -> bool {
        let distance_squared = self.center.distance_squared(point.position);
        distance_squared <= self.radius * self.radius
    }
}

impl IntersectsWith<Circle> for Point {
    fn intersects_with(&self, circle: &Circle) -> bool {
        circle.intersects_with(self)
    }
}

impl IntersectsWith<Polygon> for Circle {
    fn intersects_with(&self, polygon: &Polygon) -> bool {
        if polygon.vertices.is_empty() {
            return false;
        }

        if polygon.contains_point(self.center) {
            return true;
        }

        for i in 0..polygon.vertices.len() {
            let v1 = polygon.vertices[i];
            let v2 = polygon.vertices[(i + 1) % polygon.vertices.len()];

            if self.intersects_line_segment(v1, v2) {
                return true;
            }
        }

        false
    }
}

impl IntersectsWith<Circle> for Polygon {
    fn intersects_with(&self, circle: &Circle) -> bool {
        circle.intersects_with(self)
    }
}

impl IntersectsWith<Square> for Square {
    fn intersects_with(&self, other: &Square) -> bool {
        let x_overlap =
            (self.center.x - other.center.x).abs() <= (self.half_size + other.half_size);
        let y_overlap =
            (self.center.y - other.center.y).abs() <= (self.half_size + other.half_size);
        x_overlap && y_overlap
    }
}

impl IntersectsWith<Point> for Square {
    fn intersects_with(&self, point: &Point) -> bool {
        let dx = (point.position.x - self.center.x).abs();
        let dy = (point.position.y - self.center.y).abs();
        dx <= self.half_size && dy <= self.half_size
    }
}

impl IntersectsWith<Square> for Point {
    fn intersects_with(&self, square: &Square) -> bool {
        square.intersects_with(self)
    }
}

impl IntersectsWith<Polygon> for Square {
    fn intersects_with(&self, polygon: &Polygon) -> bool {
        if polygon.vertices.is_empty() {
            return false;
        }

        for vertex in &polygon.vertices {
            if self.intersects_with(&Point { position: *vertex }) {
                return true;
            }
        }

        let corners = [
            Vec2::new(
                self.center.x - self.half_size,
                self.center.y - self.half_size,
            ),
            Vec2::new(
                self.center.x + self.half_size,
                self.center.y - self.half_size,
            ),
            Vec2::new(
                self.center.x + self.half_size,
                self.center.y + self.half_size,
            ),
            Vec2::new(
                self.center.x - self.half_size,
                self.center.y + self.half_size,
            ),
        ];

        for corner in &corners {
            if polygon.contains_point(*corner) {
                return true;
            }
        }

        for i in 0..polygon.vertices.len() {
            let v1 = polygon.vertices[i];
            let v2 = polygon.vertices[(i + 1) % polygon.vertices.len()];

            if self.intersects_line_segment(v1, v2) {
                return true;
            }
        }

        false
    }
}

impl IntersectsWith<Square> for Polygon {
    fn intersects_with(&self, square: &Square) -> bool {
        square.intersects_with(self)
    }
}

impl IntersectsWith<Polygon> for Polygon {
    fn intersects_with(&self, other: &Polygon) -> bool {
        if self.vertices.len() < 3 || other.vertices.len() < 3 {
            return false;
        }

        for vertex in &self.vertices {
            if other.contains_point(*vertex) {
                return true;
            }
        }

        for vertex in &other.vertices {
            if self.contains_point(*vertex) {
                return true;
            }
        }

        for i in 0..self.vertices.len() {
            let a1 = self.vertices[i];
            let a2 = self.vertices[(i + 1) % self.vertices.len()];

            for j in 0..other.vertices.len() {
                let b1 = other.vertices[j];
                let b2 = other.vertices[(j + 1) % other.vertices.len()];

                if line_segments_intersect(a1, a2, b1, b2) {
                    return true;
                }
            }
        }

        false
    }
}

impl IntersectsWith<Point> for Polygon {
    fn intersects_with(&self, point: &Point) -> bool {
        self.contains_point(point.position)
    }
}

impl IntersectsWith<Polygon> for Point {
    fn intersects_with(&self, polygon: &Polygon) -> bool {
        polygon.intersects_with(self)
    }
}

impl IntersectsWith<Point> for Point {
    fn intersects_with(&self, other: &Point) -> bool {
        self.position.distance_squared(other.position) < f32::EPSILON
    }
}

impl Circle {
    fn intersects_line_segment(&self, v1: Vec2, v2: Vec2) -> bool {
        let closest = closest_point_on_segment(self.center, v1, v2);
        let distance_squared = self.center.distance_squared(closest);
        distance_squared <= self.radius * self.radius
    }
}

impl Square {
    fn intersects_line_segment(&self, v1: Vec2, v2: Vec2) -> bool {
        if self.intersects_with(&Point { position: v1 })
            || self.intersects_with(&Point { position: v2 })
        {
            return true;
        }

        let min = Vec2::new(
            self.center.x - self.half_size,
            self.center.y - self.half_size,
        );
        let max = Vec2::new(
            self.center.x + self.half_size,
            self.center.y + self.half_size,
        );

        let edges = [
            (Vec2::new(min.x, min.y), Vec2::new(max.x, min.y)),
            (Vec2::new(max.x, min.y), Vec2::new(max.x, max.y)),
            (Vec2::new(max.x, max.y), Vec2::new(min.x, max.y)),
            (Vec2::new(min.x, max.y), Vec2::new(min.x, min.y)),
        ];

        for (e1, e2) in &edges {
            if line_segments_intersect(v1, v2, *e1, *e2) {
                return true;
            }
        }

        false
    }
}

impl Polygon {
    pub fn contains_point(&self, point: Vec2) -> bool {
        if self.vertices.len() < 3 {
            return false;
        }

        let mut inside = false;
        let n = self.vertices.len();

        for i in 0..n {
            let v1 = self.vertices[i];
            let v2 = self.vertices[(i + 1) % n];

            if ((v1.y > point.y) != (v2.y > point.y))
                && (point.x < (v2.x - v1.x) * (point.y - v1.y) / (v2.y - v1.y) + v1.x)
            {
                inside = !inside;
            }
        }

        inside
    }
}

fn closest_point_on_segment(point: Vec2, v1: Vec2, v2: Vec2) -> Vec2 {
    let segment = v2 - v1;
    let segment_length_squared = segment.length_squared();

    if segment_length_squared == 0.0 {
        return v1;
    }

    let t = ((point - v1).dot(segment) / segment_length_squared).clamp(0.0, 1.0);
    v1 + segment * t
}

fn line_segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = cross_2d(b2 - b1, a1 - b1);
    let d2 = cross_2d(b2 - b1, a2 - b1);
    let d3 = cross_2d(a2 - a1, b1 - a1);
    let d4 = cross_2d(a2 - a1, b2 - a1);

    if d1 * d2 < 0.0 && d3 * d4 < 0.0 {
        return true;
    }

    if d1.abs() < f32::EPSILON && on_segment(b1, a1, b2) {
        return true;
    }
    if d2.abs() < f32::EPSILON && on_segment(b1, a2, b2) {
        return true;
    }
    if d3.abs() < f32::EPSILON && on_segment(a1, b1, a2) {
        return true;
    }
    if d4.abs() < f32::EPSILON && on_segment(a1, b2, a2) {
        return true;
    }

    false
}

fn cross_2d(v1: Vec2, v2: Vec2) -> f32 {
    v1.x * v2.y - v1.y * v2.x
}

fn on_segment(p: Vec2, q: Vec2, r: Vec2) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

use crate::{get_state, shapes_2d};

pub trait ToCollider<T> {
    fn to_collider(&self) -> T;
}

impl ToCollider<Circle> for shapes_2d::Circle {
    fn to_collider(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.radius,
        }
    }
}

impl ToCollider<Polygon> for shapes_2d::Poly {
    fn to_collider(&self) -> Polygon {
        Polygon {
            vertices: self.gen_points(),
        }
    }
}

impl ToCollider<Polygon> for shapes_2d::CustomShape {
    fn to_collider(&self) -> Polygon {
        Polygon {
            vertices: self.points.clone(),
        }
    }
}

pub fn circle(x: f32, y: f32, r: f32) -> Circle {
    Circle {
        center: Vec2::new(x, y),
        radius: r,
    }
}

pub fn rect(x: f32, y: f32, w: f32, h: f32) -> AABB2D {
    let min = Vec2::new(x, y);
    let size = Vec2::new(w, h);
    AABB2D {
        min,
        max: min + size,
    }
}

pub fn square(x: f32, y: f32, size: f32) -> Square {
    Square {
        center: Vec2::new(x, y),
        half_size: size / 2.0,
    }
}
