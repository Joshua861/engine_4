use sge_vectors::Vec2;

pub mod collision;
pub mod curves;
pub mod transform;
pub mod usize_rect;

pub trait Vec2Ext {
    fn invert_y(self) -> Self;
    fn invert_x(self) -> Self;
    fn rotated_around_point(self, point: Vec2, angle: f32) -> Self;
    fn rotated_around_origin(self, angle: f32) -> Self;
}

impl Vec2Ext for Vec2 {
    fn invert_y(self) -> Self {
        Vec2::new(self.x, -self.y)
    }

    fn invert_x(self) -> Self {
        Vec2::new(-self.x, self.y)
    }

    fn rotated_around_point(self, point: Vec2, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        let translated = self - point;

        Vec2::new(
            translated.x * cos - translated.y * sin,
            translated.x * sin + translated.y * cos,
        ) + point
    }

    fn rotated_around_origin(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Vec2::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }
}
