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
