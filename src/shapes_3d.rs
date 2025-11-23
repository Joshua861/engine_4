
use bevy_math::Vec3;

use crate::Vertex3D;

// has a bounding box. not exact. not to be used for proper collisions, just culling
pub trait HasBounds3D {
    fn bounds(&self) -> AABB3D;
}

pub trait Shape3D: HasBounds3D {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex3D>);
}

#[derive(Debug, Clone, Copy)]
pub struct AABB3D {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB3D {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn from_center_size(center: Vec3, size: Vec3) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn expand(self, amount: f32) -> Self {
        Self {
            min: self.min - Vec3::splat(amount),
            max: self.max + Vec3::splat(amount),
        }
    }
}
