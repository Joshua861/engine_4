use sge_vectors::Vec3;

/// Line that starts at some point, and continues in some direction infinitely
pub struct Ray3D {
    pub offset: Vec3,
    pub direction: Vec3,
}

impl Ray3D {
    /// Direction must be non-zero
    pub fn new(offset: Vec3, direction: Vec3) -> Self {
        Self {
            offset,
            direction: direction.normalize(),
        }
    }

    /// Direction must be non-zero and already normalized
    pub fn new_normalized(offset: Vec3, direction: Vec3) -> Self {
        Self { offset, direction }
    }

    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let to_point = point - self.offset;
        let projection_length = to_point.dot(self.direction);
        if projection_length < 0.0 {
            return self.offset; // ray is not unidirectional, point behind
        }
        self.offset + self.direction * projection_length
    }

    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        let closest_point = self.closest_point(point);
        (closest_point - point).length()
    }

    pub fn distance_to_point_squared(&self, point: Vec3) -> f32 {
        let closest_point = self.closest_point(point);
        (closest_point - point).length_squared()
    }

    pub fn intersects_point(&self, point: Vec3) -> bool {
        self.distance_to_point_squared(point) <= 1e-6
    }

    pub fn intersects_sphere(&self, center: Vec3, radius: f32) -> bool {
        self.distance_to_point_squared(center) <= radius * radius
    }

    pub fn point_along(&self, distance: f32) -> Vec3 {
        self.offset + self.direction * distance
    }

    pub fn intersects_ray(&self, other: &Ray3D) -> bool {
        let cross = self.direction.cross(other.direction);
        if cross.length_squared() < 1e-6 {
            // parallel
            return false;
        }

        let to_other = other.offset - self.offset;
        let t = to_other.cross(other.direction).dot(cross) / cross.length_squared();
        let u = to_other.cross(self.direction).dot(cross) / cross.length_squared();

        t >= 0.0 && u >= 0.0
    }
}
