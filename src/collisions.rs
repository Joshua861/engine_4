use glam::Vec2;

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

    pub fn is_visible_in_world(&self) -> bool {
        let camera = &mut get_state().camera;
        let (view_min, view_max) = camera.visible_bounds();
        let view_bounds = AABB::new(view_min, view_max);
        self.intersects(&view_bounds)
    }
}

pub trait HasBounds {
    fn bounds(&self) -> AABB;
}

impl HasBounds for Circle {
    fn bounds(&self) -> AABB {
        AABB::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
    }
}

impl HasBounds for Square {
    fn bounds(&self) -> AABB {
        AABB::from_center_size(self.center, Vec2::splat(self.half_size * 2.0))
    }
}

impl HasBounds for Polygon {
    fn bounds(&self) -> AABB {
        if self.vertices.is_empty() {
            return AABB::new(Vec2::ZERO, Vec2::ZERO);
        }

        let mut min = self.vertices[0];
        let mut max = self.vertices[0];

        for vertex in &self.vertices[1..] {
            min = min.min(*vertex);
            max = max.max(*vertex);
        }

        AABB::new(min, max)
    }
}

impl HasBounds for Point {
    fn bounds(&self) -> AABB {
        AABB::from_center_size(self.position, Vec2::ZERO)
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

use crate::{get_state, shapes};

pub trait ToCollider<T> {
    fn to_collider(&self) -> T;
}

impl ToCollider<Circle> for shapes::Circle {
    fn to_collider(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.radius,
        }
    }
}

impl ToCollider<Polygon> for shapes::Poly {
    fn to_collider(&self) -> Polygon {
        Polygon {
            vertices: self.gen_points(),
        }
    }
}

impl ToCollider<Polygon> for shapes::CustomShape {
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

pub fn rect(x: f32, y: f32, w: f32, h: f32) -> AABB {
    let min = Vec2::new(x, y);
    let size = Vec2::new(w, h);
    AABB {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collisions::ray::*;
    use glam::Vec2;

    // Circle-Circle tests
    #[test]
    fn circle_circle_intersect() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let c2 = Circle {
            center: Vec2::new(8.0, 0.0),
            radius: 5.0,
        };
        assert!(c1.intersects_with(&c2));
    }

    #[test]
    fn circle_circle_touching() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let c2 = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 5.0,
        };
        assert!(c1.intersects_with(&c2));
    }

    #[test]
    fn circle_circle_no_intersect() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let c2 = Circle {
            center: Vec2::new(20.0, 0.0),
            radius: 5.0,
        };
        assert!(!c1.intersects_with(&c2));
    }

    #[test]
    fn circle_circle_contained() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 10.0,
        };
        let c2 = Circle {
            center: Vec2::new(1.0, 1.0),
            radius: 2.0,
        };
        assert!(c1.intersects_with(&c2));
    }

    // Circle-Square tests
    #[test]
    fn circle_square_intersect() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(7.0, 0.0),
            half_size: 5.0,
        };
        assert!(circle.intersects_with(&square));
    }

    #[test]
    fn circle_square_corner_intersect() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(7.0, 7.0),
            half_size: 5.0,
        };
        assert!(circle.intersects_with(&square));
    }

    #[test]
    fn circle_square_no_intersect() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(20.0, 0.0),
            half_size: 5.0,
        };
        assert!(!circle.intersects_with(&square));
    }

    #[test]
    fn circle_inside_square() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 2.0,
        };
        let square = Square {
            center: Vec2::ZERO,
            half_size: 10.0,
        };
        assert!(circle.intersects_with(&square));
    }

    // Square-Square tests
    #[test]
    fn square_square_intersect() {
        let s1 = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let s2 = Square {
            center: Vec2::new(7.0, 0.0),
            half_size: 5.0,
        };
        assert!(s1.intersects_with(&s2));
    }

    #[test]
    fn square_square_touching() {
        let s1 = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let s2 = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        assert!(s1.intersects_with(&s2));
    }

    #[test]
    fn square_square_no_intersect() {
        let s1 = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let s2 = Square {
            center: Vec2::new(20.0, 0.0),
            half_size: 5.0,
        };
        assert!(!s1.intersects_with(&s2));
    }

    // Point tests
    #[test]
    fn point_in_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let point = Point::new(Vec2::new(3.0, 0.0));
        assert!(circle.intersects_with(&point));
    }

    #[test]
    fn point_outside_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let point = Point::new(Vec2::new(10.0, 0.0));
        assert!(!circle.intersects_with(&point));
    }

    #[test]
    fn point_in_square() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let point = Point::new(Vec2::new(3.0, 3.0));
        assert!(square.intersects_with(&point));
    }

    #[test]
    fn point_outside_square() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let point = Point::new(Vec2::new(10.0, 10.0));
        assert!(!square.intersects_with(&point));
    }

    // Polygon tests
    #[test]
    fn triangle_contains_point() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, 10.0),
            ],
        };
        assert!(polygon.contains_point(Vec2::new(5.0, 3.0)));
    }

    #[test]
    fn triangle_does_not_contain_point() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, 10.0),
            ],
        };
        assert!(!polygon.contains_point(Vec2::new(15.0, 15.0)));
    }

    #[test]
    fn polygon_circle_intersect() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
            ],
        };
        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 2.0,
        };
        assert!(polygon.intersects_with(&circle));
    }

    #[test]
    fn polygon_circle_edge_intersect() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
            ],
        };
        let circle = Circle {
            center: Vec2::new(12.0, 5.0),
            radius: 3.0,
        };
        assert!(polygon.intersects_with(&circle));
    }

    #[test]
    fn polygon_polygon_intersect() {
        let poly1 = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
            ],
        };
        let poly2 = Polygon {
            vertices: vec![
                Vec2::new(5.0, 5.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(15.0, 15.0),
                Vec2::new(5.0, 15.0),
            ],
        };
        assert!(poly1.intersects_with(&poly2));
    }

    // AABB tests
    #[test]
    fn aabb_intersect() {
        let aabb1 = AABB::new(Vec2::ZERO, Vec2::new(10.0, 10.0));
        let aabb2 = AABB::new(Vec2::new(5.0, 5.0), Vec2::new(15.0, 15.0));
        assert!(aabb1.intersects(&aabb2));
    }

    #[test]
    fn aabb_no_intersect() {
        let aabb1 = AABB::new(Vec2::ZERO, Vec2::new(10.0, 10.0));
        let aabb2 = AABB::new(Vec2::new(20.0, 20.0), Vec2::new(30.0, 30.0));
        assert!(!aabb1.intersects(&aabb2));
    }

    #[test]
    fn aabb_from_center_size() {
        let aabb = AABB::from_center_size(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        assert_eq!(aabb.min, Vec2::ZERO);
        assert_eq!(aabb.max, Vec2::new(10.0, 10.0));
    }

    #[test]
    fn aabb_expand() {
        let aabb = AABB::new(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        let expanded = aabb.expand(2.0);
        assert_eq!(expanded.min, Vec2::new(3.0, 3.0));
        assert_eq!(expanded.max, Vec2::new(12.0, 12.0));
    }

    // HasBounds tests
    #[test]
    fn circle_bounds() {
        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 3.0,
        };
        let bounds = circle.bounds();
        assert_eq!(bounds.min, Vec2::new(2.0, 2.0));
        assert_eq!(bounds.max, Vec2::new(8.0, 8.0));
    }

    #[test]
    fn square_bounds() {
        let square = Square {
            center: Vec2::new(5.0, 5.0),
            half_size: 3.0,
        };
        let bounds = square.bounds();
        assert_eq!(bounds.min, Vec2::new(2.0, 2.0));
        assert_eq!(bounds.max, Vec2::new(8.0, 8.0));
    }

    #[test]
    fn polygon_bounds() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(1.0, 2.0),
                Vec2::new(5.0, 1.0),
                Vec2::new(7.0, 6.0),
                Vec2::new(3.0, 8.0),
            ],
        };
        let bounds = polygon.bounds();
        assert_eq!(bounds.min, Vec2::new(1.0, 1.0));
        assert_eq!(bounds.max, Vec2::new(7.0, 8.0));
    }

    // Symmetry tests
    #[test]
    fn intersection_symmetry() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(7.0, 0.0),
            half_size: 5.0,
        };
        assert_eq!(
            circle.intersects_with(&square),
            square.intersects_with(&circle)
        );
    }

    #[test]
    fn collision_then_raycast() {
        let circle1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let circle2 = Circle {
            center: Vec2::new(8.0, 0.0),
            radius: 5.0,
        };

        // They should intersect
        assert!(circle1.intersects_with(&circle2));

        // Ray from circle1 to circle2 should hit
        let ray = Ray::from_points(circle1.center, circle2.center);
        assert!(circle2.raycast(&ray).is_some());
    }

    #[test]
    fn aabb_contains_shapes() {
        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 2.0,
        };
        let square = Square {
            center: Vec2::new(5.0, 5.0),
            half_size: 3.0,
        };

        let circle_bounds = circle.bounds();
        let square_bounds = square.bounds();

        // Square should contain circle
        assert!(square_bounds.intersects(&circle_bounds));
    }

    #[test]
    fn point_in_all_overlapping_shapes() {
        let point = Point::new(Vec2::new(5.0, 5.0));

        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 3.0,
        };
        let square = Square {
            center: Vec2::new(5.0, 5.0),
            half_size: 4.0,
        };
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(3.0, 3.0),
                Vec2::new(7.0, 3.0),
                Vec2::new(7.0, 7.0),
                Vec2::new(3.0, 7.0),
            ],
        };

        assert!(circle.intersects_with(&point));
        assert!(square.intersects_with(&point));
        assert!(polygon.intersects_with(&point));
    }

    #[test]
    fn raycast_through_overlapping_shapes() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let square = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));
        let circle_hit = circle.raycast(&ray);
        let square_hit = square.raycast(&ray);
        assert!(circle_hit.is_some());
        assert!(square_hit.is_some());

        assert!(square_hit.unwrap().distance < circle_hit.unwrap().distance);
    }

    #[test]
    fn bounds_after_collision() {
        let mut shapes: Vec<Box<dyn HasBounds>> = vec![
            Box::new(Circle {
                center: Vec2::new(0.0, 0.0),
                radius: 5.0,
            }),
            Box::new(Square {
                center: Vec2::new(8.0, 0.0),
                half_size: 3.0,
            }),
        ];

        // Get all bounds
        let bounds: Vec<AABB> = shapes.iter().map(|s| s.bounds()).collect();

        // Check if any bounds intersect
        let mut intersections = 0;
        for i in 0..bounds.len() {
            for j in (i + 1)..bounds.len() {
                if bounds[i].intersects(&bounds[j]) {
                    intersections += 1;
                }
            }
        }

        assert!(intersections > 0);
    }

    #[test]
    fn complex_scene() {
        // Create a scene with multiple shapes
        let shapes = vec![
            Circle {
                center: Vec2::new(0.0, 0.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(20.0, 0.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(40.0, 0.0),
                radius: 5.0,
            },
        ];

        // Cast a ray through all of them
        let ray = Ray::new(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 0.0));

        let hits: Vec<_> = shapes
            .iter()
            .filter_map(|shape| shape.raycast(&ray))
            .collect();

        assert_eq!(hits.len(), 3);

        // Check they're in order
        for i in 0..hits.len() - 1 {
            assert!(hits[i].distance < hits[i + 1].distance);
        }
    }

    #[test]
    fn spatial_partitioning_scenario() {
        // Simulate a grid-based collision system
        let grid_size = 10.0;
        let shapes = vec![
            (
                0,
                0,
                Circle {
                    center: Vec2::new(5.0, 5.0),
                    radius: 2.0,
                },
            ),
            (
                1,
                0,
                Circle {
                    center: Vec2::new(15.0, 5.0),
                    radius: 2.0,
                },
            ),
            (
                0,
                1,
                Circle {
                    center: Vec2::new(5.0, 15.0),
                    radius: 2.0,
                },
            ),
        ];

        let query_point = Point::new(Vec2::new(6.0, 6.0));

        // Only check shapes in the same grid cell (0, 0)
        let nearby_shapes: Vec<_> = shapes
            .iter()
            .filter(|(x, y, _)| *x == 0 && *y == 0)
            .collect();

        assert_eq!(nearby_shapes.len(), 1);
        assert!(nearby_shapes[0].2.intersects_with(&query_point));
    }

    #[test]
    fn raycast_returns_closest_hit() {
        let shapes = vec![
            Circle {
                center: Vec2::new(10.0, 0.0),
                radius: 2.0,
            },
            Circle {
                center: Vec2::new(20.0, 0.0),
                radius: 2.0,
            },
            Circle {
                center: Vec2::new(30.0, 0.0),
                radius: 2.0,
            },
        ];

        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        // Find closest hit manually
        let closest = shapes
            .iter()
            .filter_map(|shape| shape.raycast(&ray))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        assert!(closest.is_some());
        let closest = closest.unwrap();

        // Should hit the first circle
        assert!((closest.distance - 8.0).abs() < 0.1);
    }

    #[test]
    fn collision_filtering_by_bounds() {
        let shapes = vec![
            Circle {
                center: Vec2::new(0.0, 0.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(100.0, 100.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(200.0, 200.0),
                radius: 5.0,
            },
        ];

        let query_bounds = AABB::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0));

        // Filter shapes by bounds first
        let nearby: Vec<_> = shapes
            .iter()
            .filter(|shape| shape.bounds().intersects(&query_bounds))
            .collect();

        assert_eq!(nearby.len(), 1);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use crate::collisions::ray::*;
    use glam::Vec2;

    #[test]
    fn many_circle_collisions() {
        let circles: Vec<Circle> = (0..100)
            .map(|i| Circle {
                center: Vec2::new((i % 10) as f32 * 10.0, (i / 10) as f32 * 10.0),
                radius: 5.0,
            })
            .collect();

        let test_circle = Circle {
            center: Vec2::new(45.0, 45.0),
            radius: 8.0,
        };

        let mut collision_count = 0;
        for circle in &circles {
            if test_circle.intersects_with(circle) {
                collision_count += 1;
            }
        }

        assert!(collision_count > 0);
    }

    #[test]
    fn many_raycasts() {
        let shapes: Vec<Circle> = (0..50)
            .map(|i| Circle {
                center: Vec2::new((i as f32) * 20.0, 0.0),
                radius: 5.0,
            })
            .collect();

        let ray = Ray::new(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 0.0));

        let mut hit_count = 0;
        for shape in &shapes {
            if shape.raycast(&ray).is_some() {
                hit_count += 1;
            }
        }

        assert_eq!(hit_count, shapes.len());
    }

    #[test]
    fn aabb_broadphase_effectiveness() {
        let shapes: Vec<Square> = (0..100)
            .map(|i| Square {
                center: Vec2::new((i % 10) as f32 * 100.0, (i / 10) as f32 * 100.0),
                half_size: 10.0,
            })
            .collect();

        let test_aabb = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0));

        let mut potential_collisions = 0;
        for shape in &shapes {
            if test_aabb.intersects(&shape.bounds()) {
                potential_collisions += 1;
            }
        }

        // Should filter out most shapes
        assert!(potential_collisions < shapes.len() / 2);
    }
}
