use bevy_math::{BVec3, Mat3, Mat4, Quat, Vec3, Vec4Swizzles};
use glium::BackfaceCullingMode;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Transform3D {
    mat: Mat4,
    dirty: bool,
    scale: Vec3,
    rotation: Quat,
    translation: Vec3,
    mirror: BVec3,
}

impl Transform3D {
    pub const IDENTITY: Self = Self {
        mat: Mat4::IDENTITY,
        dirty: false,
        scale: Vec3::ONE,
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
        mirror: BVec3::FALSE,
    };

    pub fn update_matrix(&mut self) {
        if !self.dirty {
            return;
        }

        let effective_scale = Vec3::new(
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
            if self.mirror.z {
                -self.scale.z
            } else {
                self.scale.z
            },
        );

        self.mat =
            Mat4::from_scale_rotation_translation(effective_scale, self.rotation, self.translation);
        self.dirty = false;
    }

    pub fn should_flip_culling(&self) -> bool {
        let mirror_count = [self.mirror.x, self.mirror.y, self.mirror.z]
            .iter()
            .filter(|&&m| m)
            .count();
        mirror_count % 2 == 1
    }

    pub fn desired_culling_mode(&self) -> BackfaceCullingMode {
        if self.should_flip_culling() {
            glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise
        } else {
            glium::draw_parameters::BackfaceCullingMode::CullClockwise
        }
    }

    pub fn matrix(&mut self) -> Mat4 {
        self.update_matrix();
        self.mat
    }

    pub fn into_normal_matrix(&mut self) -> Mat3 {
        let mat = self.matrix();
        let mat = Mat3::from_mat4(mat);
        mat.inverse().transpose()
    }

    pub fn transformed_point(&mut self, point: Vec3) -> Vec3 {
        (self.matrix() * point.extend(1.0)).xyz()
    }

    pub fn transform_point(&mut self, point: &mut Vec3) {
        *point = (self.matrix() * point.extend(1.0)).xyz();
    }

    fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    pub fn scale_mut(&mut self) -> &mut Vec3 {
        self.mark_dirty();
        &mut self.scale
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.mark_dirty();
    }

    pub fn from_scale(scale: Vec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self.mark_dirty();
        self
    }

    pub fn scale_by(&mut self, scale: Vec3) {
        self.scale *= scale;
        self.mark_dirty();
    }

    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    pub fn rotation_mut(&mut self) -> &mut Quat {
        self.mark_dirty();
        &mut self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.mark_dirty();
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self.mark_dirty();
        self
    }

    pub fn rotate_by(&mut self, rotation: Quat) {
        self.rotation *= rotation;
        self.mark_dirty();
    }

    pub fn translation(&self) -> Vec3 {
        self.translation
    }

    pub fn translation_mut(&mut self) -> &mut Vec3 {
        self.mark_dirty();
        &mut self.translation
    }

    pub fn set_translation(&mut self, translation: Vec3) {
        self.translation = translation;
        self.mark_dirty();
    }

    pub fn from_translation(translation: Vec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.translation = translation;
        transform
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self.mark_dirty();
        self
    }

    pub fn translate_by(&mut self, translation: Vec3) {
        self.translation += translation;
        self.mark_dirty();
    }

    pub fn mirror(&self) -> BVec3 {
        self.mirror
    }

    pub fn mirror_mut(&mut self) -> &mut BVec3 {
        self.mark_dirty();
        &mut self.mirror
    }

    pub fn set_mirror(&mut self, mirror: BVec3) {
        self.mirror = mirror;
        self.mark_dirty();
    }

    pub fn from_mirror(mirror: BVec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.mirror = mirror;
        transform
    }

    pub fn with_mirror(mut self, mirror: BVec3) -> Self {
        self.mirror = mirror;
        self.mark_dirty();
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

    pub fn mirror_z(&mut self) {
        self.mirror.z = !self.mirror.z;
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

    pub fn translate_z(&mut self, z: f32) {
        self.translation.z += z;
        self.mark_dirty();
    }

    pub fn translation_x(&self) -> f32 {
        self.translation.x
    }

    pub fn translation_y(&self) -> f32 {
        self.translation.y
    }

    pub fn translation_z(&self) -> f32 {
        self.translation.z
    }

    pub fn set_translation_x(&mut self, x: f32) {
        self.translation.x = x;
        self.mark_dirty();
    }

    pub fn set_translation_y(&mut self, y: f32) {
        self.translation.y = y;
        self.mark_dirty();
    }

    pub fn set_translation_z(&mut self, z: f32) {
        self.translation.z = z;
        self.mark_dirty();
    }

    pub fn scale_x(&self) -> f32 {
        self.scale.x
    }

    pub fn scale_y(&self) -> f32 {
        self.scale.y
    }

    pub fn scale_z(&self) -> f32 {
        self.scale.z
    }

    pub fn set_scale_x(&mut self, x: f32) {
        self.scale.x = x;
        self.mark_dirty();
    }

    pub fn set_scale_y(&mut self, y: f32) {
        self.scale.y = y;
        self.mark_dirty();
    }

    pub fn set_scale_z(&mut self, z: f32) {
        self.scale.z = z;
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

    pub fn scale_by_z(&mut self, z: f32) {
        self.scale.z *= z;
        self.mark_dirty();
    }

    pub fn from_scale_rotation(scale: Vec3, rotation: Quat) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform
    }

    pub fn from_scale_translation(scale: Vec3, translation: Vec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.translation = translation;
        transform
    }

    pub fn from_scale_mirror(scale: Vec3, mirror: BVec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.mirror = mirror;
        transform
    }

    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform.translation = translation;
        transform
    }

    pub fn from_rotation_mirror(rotation: Quat, mirror: BVec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_translation_mirror(translation: Vec3, mirror: BVec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.translation = translation;
        transform
    }

    pub fn from_scale_rotation_mirror(scale: Vec3, rotation: Quat, mirror: BVec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_scale_translation_mirror(scale: Vec3, translation: Vec3, mirror: BVec3) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_rotation_translation_mirror(
        rotation: Quat,
        translation: Vec3,
        mirror: BVec3,
    ) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.rotation = rotation;
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn from_scale_rotation_translation_mirror(
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
        mirror: BVec3,
    ) -> Self {
        let mut transform = Self::IDENTITY;
        transform.mark_dirty();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.translation = translation;
        transform.mirror = mirror;
        transform
    }

    pub fn translated_by(mut self, translation: Vec3) -> Self {
        self.translation += translation;
        self.mark_dirty();
        self
    }

    pub fn scaled_by(mut self, scale: Vec3) -> Self {
        self.scale *= scale;
        self.mark_dirty();
        self
    }

    pub fn rotated_by(mut self, rotation: Quat) -> Self {
        self.rotation *= rotation;
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

    pub fn mirrored_z(mut self) -> Self {
        self.mirror.z = !self.mirror.z;
        self.mark_dirty();
        self
    }

    pub fn mirrored_by(mut self, mirror: BVec3) -> Self {
        self.mirror = BVec3::new(
            self.mirror.x ^ mirror.x,
            self.mirror.y ^ mirror.y,
            self.mirror.z ^ mirror.z,
        );
        self.mark_dirty();
        self
    }

    pub fn with_scale_rotation_translation(
        mut self,
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Self {
        self.scale = scale;
        self.rotation = rotation;
        self.translation = translation;
        self.mark_dirty();
        self
    }
}

impl Add for Transform3D {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.scale += rhs.scale;
        self.rotation *= rhs.rotation;
        self.translation += rhs.translation;
        self.mirror = BVec3::new(
            self.mirror.x ^ rhs.mirror.x,
            self.mirror.y ^ rhs.mirror.y,
            self.mirror.z ^ rhs.mirror.z,
        );
        self.mark_dirty();
        self
    }
}

impl AddAssign for Transform3D {
    fn add_assign(&mut self, rhs: Self) {
        self.scale += rhs.scale;
        self.rotation *= rhs.rotation;
        self.translation += rhs.translation;
        self.mirror = BVec3::new(
            self.mirror.x ^ rhs.mirror.x,
            self.mirror.y ^ rhs.mirror.y,
            self.mirror.z ^ rhs.mirror.z,
        );
        self.mark_dirty();
    }
}

impl Sub for Transform3D {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.scale -= rhs.scale;
        self.rotation *= rhs.rotation.conjugate();
        self.translation -= rhs.translation;
        self.mirror = BVec3::new(
            self.mirror.x ^ rhs.mirror.x,
            self.mirror.y ^ rhs.mirror.y,
            self.mirror.z ^ rhs.mirror.z,
        );
        self.mark_dirty();
        self
    }
}

impl SubAssign for Transform3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.scale -= rhs.scale;
        self.rotation *= rhs.rotation.conjugate();
        self.translation -= rhs.translation;
        self.mirror = BVec3::new(
            self.mirror.x ^ rhs.mirror.x,
            self.mirror.y ^ rhs.mirror.y,
            self.mirror.z ^ rhs.mirror.z,
        );
        self.mark_dirty();
    }
}

impl Mul<f32> for Transform3D {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.scale *= rhs;
        self.rotation = Quat::IDENTITY.slerp(self.rotation, rhs);
        self.translation *= rhs;
        self.mark_dirty();
        self
    }
}

impl MulAssign<f32> for Transform3D {
    fn mul_assign(&mut self, rhs: f32) {
        self.scale *= rhs;
        self.rotation = Quat::IDENTITY.slerp(self.rotation, rhs);
        self.translation *= rhs;
        self.mark_dirty();
    }
}

impl Div<f32> for Transform3D {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self.scale /= rhs;
        self.rotation = Quat::IDENTITY.slerp(self.rotation, 1.0 / rhs);
        self.translation /= rhs;
        self.mark_dirty();
        self
    }
}

impl DivAssign<f32> for Transform3D {
    fn div_assign(&mut self, rhs: f32) {
        self.scale /= rhs;
        self.rotation = Quat::IDENTITY.slerp(self.rotation, 1.0 / rhs);
        self.translation /= rhs;
        self.mark_dirty();
    }
}

impl Mul for Transform3D {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        let lhs_mat = self.matrix();
        let mut rhs_copy = rhs;
        let rhs_mat = rhs_copy.matrix();

        self.mat = lhs_mat * rhs_mat;

        self.scale *= rhs.scale;

        self.rotation *= rhs.rotation;

        let rotated_translation = self.rotation * rhs.translation;
        self.translation += rotated_translation * self.scale;

        self.mirror = BVec3::new(
            self.mirror.x ^ rhs.mirror.x,
            self.mirror.y ^ rhs.mirror.y,
            self.mirror.z ^ rhs.mirror.z,
        );

        self.mark_dirty();
        self
    }
}

impl Mul<Vec3> for Transform3D {
    type Output = Vec3;
    fn mul(mut self, rhs: Vec3) -> Self::Output {
        self.transformed_point(rhs)
    }
}

impl MulAssign for Transform3D {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
