use bevy_math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec2,
    pub direction: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct RaycastHit {
    pub point: Vec2,
    pub distance: f32,
    pub normal: Vec2,
}

impl Ray {
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn from_points(from: Vec2, to: Vec2) -> Self {
        Self::new(from, to - from)
    }

    pub fn point_at(&self, t: f32) -> Vec2 {
        self.origin + self.direction * t
    }
}

pub trait Raycast {
    fn raycast(&self, ray: &Ray) -> Option<RaycastHit>;

    fn raycast_max(&self, ray: &Ray, max_distance: f32) -> Option<RaycastHit> {
        self.raycast(ray).filter(|hit| hit.distance <= max_distance)
    }
}

impl Raycast for super::Circle {
    fn raycast(&self, ray: &Ray) -> Option<RaycastHit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        let t = if t1 >= 0.0 {
            t1
        } else if t2 >= 0.0 {
            t2
        } else {
            return None;
        };

        let point = ray.point_at(t);
        let normal = (point - self.center).normalize();

        Some(RaycastHit {
            point,
            distance: t,
            normal,
        })
    }
}

impl Raycast for super::Square {
    fn raycast(&self, ray: &Ray) -> Option<RaycastHit> {
        let min = Vec2::new(
            self.center.x - self.half_size,
            self.center.y - self.half_size,
        );
        let max = Vec2::new(
            self.center.x + self.half_size,
            self.center.y + self.half_size,
        );

        let inv_dir = Vec2::new(1.0 / ray.direction.x, 1.0 / ray.direction.y);

        let t1 = (min.x - ray.origin.x) * inv_dir.x;
        let t2 = (max.x - ray.origin.x) * inv_dir.x;
        let t3 = (min.y - ray.origin.y) * inv_dir.y;
        let t4 = (max.y - ray.origin.y) * inv_dir.y;

        let tmin = t1.min(t2).max(t3.min(t4));
        let tmax = t1.max(t2).min(t3.max(t4));

        if tmax < 0.0 || tmin > tmax {
            return None;
        }

        let t = if tmin >= 0.0 { tmin } else { tmax };
        if t < 0.0 {
            return None;
        }

        let point = ray.point_at(t);

        let normal = {
            let eps = 0.0001;
            if (point.x - min.x).abs() < eps {
                Vec2::new(-1.0, 0.0)
            } else if (point.x - max.x).abs() < eps {
                Vec2::new(1.0, 0.0)
            } else if (point.y - min.y).abs() < eps {
                Vec2::new(0.0, -1.0)
            } else {
                Vec2::new(0.0, 1.0)
            }
        };

        Some(RaycastHit {
            point,
            distance: t,
            normal,
        })
    }
}

impl Raycast for super::Polygon {
    fn raycast(&self, ray: &Ray) -> Option<RaycastHit> {
        if self.vertices.len() < 3 {
            return None;
        }

        let mut closest_hit: Option<RaycastHit> = None;
        let mut min_distance = f32::INFINITY;

        for i in 0..self.vertices.len() {
            let v1 = self.vertices[i];
            let v2 = self.vertices[(i + 1) % self.vertices.len()];

            if let Some(t) = ray_segment_intersection(ray, v1, v2) {
                if t >= 0.0 && t < min_distance {
                    let point = ray.point_at(t);

                    let edge = v2 - v1;
                    let edge_normal = Vec2::new(-edge.y, edge.x).normalize();

                    let normal = if edge_normal.dot(ray.direction) < 0.0 {
                        edge_normal
                    } else {
                        -edge_normal
                    };

                    min_distance = t;
                    closest_hit = Some(RaycastHit {
                        point,
                        distance: t,
                        normal,
                    });
                }
            }
        }

        closest_hit
    }
}

impl Raycast for super::Point {
    fn raycast(&self, ray: &Ray) -> Option<RaycastHit> {
        let to_point = self.position - ray.origin;
        let t = to_point.dot(ray.direction);

        if t < 0.0 {
            return None;
        }

        let closest = ray.point_at(t);
        let distance_to_ray = self.position.distance(closest);

        if distance_to_ray < 0.001 {
            Some(RaycastHit {
                point: self.position,
                distance: t,
                normal: Vec2::ZERO,
            })
        } else {
            None
        }
    }
}

fn ray_segment_intersection(ray: &Ray, v1: Vec2, v2: Vec2) -> Option<f32> {
    let segment = v2 - v1;
    let ray_cross_segment = cross_2d(ray.direction, segment);

    if ray_cross_segment.abs() < f32::EPSILON {
        return None;
    }

    let to_segment = v1 - ray.origin;
    let t = cross_2d(to_segment, segment) / ray_cross_segment;
    let u = cross_2d(to_segment, ray.direction) / ray_cross_segment;

    if t >= 0.0 && u >= 0.0 && u <= 1.0 {
        Some(t)
    } else {
        None
    }
}

fn cross_2d(v1: Vec2, v2: Vec2) -> f32 {
    v1.x * v2.y - v1.y * v2.x
}

#[cfg(test)]
mod raycast_tests {
    use super::*;
    use crate::collisions::*;
    use bevy_math::Vec2;

    // Basic ray tests
    #[test]
    fn test_ray_creation() {
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));
        assert_eq!(ray.origin, Vec2::ZERO);
        assert_eq!(ray.direction, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_ray_from_points() {
        let ray = Ray::from_points(Vec2::ZERO, Vec2::new(10.0, 0.0));
        assert_eq!(ray.origin, Vec2::ZERO);
        assert_eq!(ray.direction, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_ray_direction_normalized() {
        let ray = Ray::new(Vec2::ZERO, Vec2::new(3.0, 4.0));
        let length = ray.direction.length();
        assert!((length - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ray_point_at() {
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));
        let point = ray.point_at(5.0);
        assert_eq!(point, Vec2::new(5.0, 0.0));
    }

    // Circle raycast tests
    #[test]
    fn test_ray_hits_circle_center() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 7.0).abs() < 0.001);
        assert_eq!(hit.normal, Vec2::new(-1.0, 0.0));
    }

    #[test]
    fn test_ray_misses_circle() {
        let circle = Circle {
            center: Vec2::new(10.0, 10.0),
            radius: 2.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast(&ray).is_none());
    }

    #[test]
    fn test_ray_inside_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 10.0,
        };
        let ray = Ray::new(Vec2::new(5.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_tangent_to_circle() {
        let circle = Circle {
            center: Vec2::new(0.0, 5.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_behind_circle() {
        let circle = Circle {
            center: Vec2::new(-10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast(&ray).is_none());
    }

    // Square raycast tests
    #[test]
    fn test_ray_hits_square_front() {
        let square = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = square.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 5.0).abs() < 0.001);
        assert_eq!(hit.normal, Vec2::new(-1.0, 0.0));
    }

    #[test]
    fn test_ray_hits_square_corner() {
        let square = Square {
            center: Vec2::new(10.0, 10.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_misses_square() {
        let square = Square {
            center: Vec2::new(10.0, 10.0),
            half_size: 2.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(square.raycast(&ray).is_none());
    }

    #[test]
    fn test_ray_inside_square() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 10.0,
        };
        let ray = Ray::new(Vec2::new(5.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    // Polygon raycast tests
    #[test]
    fn test_ray_hits_triangle() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, -5.0),
                Vec2::new(10.0, 5.0),
                Vec2::new(15.0, 0.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_ray_misses_triangle() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, 10.0),
                Vec2::new(15.0, 10.0),
                Vec2::new(12.5, 15.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(polygon.raycast(&ray).is_none());
    }

    #[test]
    fn test_ray_hits_square_polygon() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, -5.0),
                Vec2::new(15.0, -5.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(10.0, 5.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_hits_closest_edge() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, -5.0),
                Vec2::new(15.0, -5.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(10.0, 5.0),
            ],
        };
        let ray = Ray::new(Vec2::new(-5.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        // Should hit the left edge at x=10
        assert!((hit.point.x - 10.0).abs() < 0.001);
    }

    // Point raycast tests
    #[test]
    fn test_ray_hits_point() {
        let point = Point::new(Vec2::new(10.0, 0.0));
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = point.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_misses_point() {
        let point = Point::new(Vec2::new(10.0, 5.0));
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(point.raycast(&ray).is_none());
    }

    // Max distance tests
    #[test]
    fn test_raycast_max_distance_hit() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast_max(&ray, 20.0).is_some());
    }

    #[test]
    fn test_raycast_max_distance_miss() {
        let circle = Circle {
            center: Vec2::new(100.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast_max(&ray, 50.0).is_none());
    }

    // Multiple hits tests
    #[test]
    fn test_ray_through_multiple_circles() {
        let circle1 = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let circle2 = Circle {
            center: Vec2::new(20.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit1 = circle1.raycast(&ray);
        let hit2 = circle2.raycast(&ray);

        assert!(hit1.is_some());
        assert!(hit2.is_some());

        // First circle should be closer
        assert!(hit1.unwrap().distance < hit2.unwrap().distance);
    }

    // Diagonal ray tests
    #[test]
    fn test_diagonal_ray_hits_circle() {
        let circle = Circle {
            center: Vec2::new(10.0, 10.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_diagonal_ray_hits_square() {
        let square = Square {
            center: Vec2::new(10.0, 10.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    // Normal vector tests
    #[test]
    fn test_circle_hit_normal_pointing_out() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray).unwrap();
        // Normal should point back towards ray origin
        assert!(hit.normal.dot(ray.direction) < 0.0);
    }

    #[test]
    fn test_square_hit_normal_perpendicular() {
        let square = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = square.raycast(&ray).unwrap();
        // Normal should be perpendicular to the hit surface
        assert_eq!(hit.normal.length(), 1.0);
    }

    // Edge cases
    #[test]
    fn test_ray_origin_on_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 10.0,
        };
        let ray = Ray::new(Vec2::new(10.0, 0.0), Vec2::new(1.0, 0.0));

        // Ray starting on circle surface
        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_origin_on_square_edge() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 10.0,
        };
        let ray = Ray::new(Vec2::new(10.0, 0.0), Vec2::new(1.0, 0.0));

        // Ray starting on square edge
        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_zero_length_ray_direction() {
        let ray = Ray::new(Vec2::ZERO, Vec2::ZERO);
        // Should handle gracefully (normalize will produce NaN or zero)
        assert!(ray.direction.length().is_nan() || ray.direction == Vec2::ZERO);
    }

    // Complex polygon tests
    #[test]
    fn test_ray_hits_concave_polygon() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, 0.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(12.0, 3.0), // Creates concave shape
                Vec2::new(15.0, -5.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_hits_pentagon() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, 0.0),
                Vec2::new(12.0, 3.0),
                Vec2::new(11.0, 6.0),
                Vec2::new(9.0, 6.0),
                Vec2::new(8.0, 3.0),
            ],
        };
        let ray = Ray::new(Vec2::new(10.0, -5.0), Vec2::new(0.0, 1.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());
    }

    // Angle tests
    #[test]
    fn test_ray_at_45_degrees() {
        let circle = Circle {
            center: Vec2::new(10.0, 10.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_at_shallow_angle() {
        let circle = Circle {
            center: Vec2::new(100.0, 1.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.01).normalize());

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    // Negative direction tests
    #[test]
    fn test_ray_negative_direction() {
        let circle = Circle {
            center: Vec2::new(-10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(-1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_downward() {
        let square = Square {
            center: Vec2::new(0.0, -10.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(0.0, -1.0));

        let hit = square.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 5.0).abs() < 0.001);
    }

    // Distance accuracy tests
    #[test]
    fn test_raycast_distance_accuracy() {
        let circle = Circle {
            center: Vec2::new(100.0, 0.0),
            radius: 10.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray).unwrap();
        assert!((hit.distance - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_raycast_hit_point_on_surface() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray).unwrap();
        let distance_from_center = hit.point.distance(circle.center);
        assert!((distance_from_center - circle.radius).abs() < 0.001);
    }
}
