use bevy_math::{BVec2, Mat4, Quat, Vec2, Vec4Swizzles};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

    pub fn transformed_point(&mut self, point: Vec2) -> Vec2 {
        (self.matrix() * point.extend(0.0).extend(0.0)).xy()
    }

    pub fn transform_point(&mut self, point: &mut Vec2) {
        *point = (self.matrix() * point.extend(0.0).extend(0.0)).xy();
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

impl Mul<Vec2> for Transform2D {
    type Output = Vec2;
    fn mul(mut self, rhs: Vec2) -> Self::Output {
        self.transformed_point(rhs)
    }
}

impl MulAssign for Transform2D {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
