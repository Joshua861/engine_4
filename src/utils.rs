use bevy_math::{UVec2, Vec2, Vec3};

pub(crate) fn extend_vec2(v: &Vec2) -> Vec3 {
    Vec3::new(v.x, v.y, 1.0)
}
