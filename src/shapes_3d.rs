use bevy_math::Vec3;

use crate::Vertex3D;

#[derive(Debug, Clone, Copy)]
pub struct AABB3D {
    pub min: Vec3,
    pub max: Vec3,
}

pub trait HasBounds3D {
    fn bounds(&self) -> AABB3D;
}

pub trait Shape3D: HasBounds3D {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex3D>);
}
