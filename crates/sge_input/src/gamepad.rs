use gilrs::{Axis, Button, Gamepad};
use sge_vectors::Vec2;

pub trait GamepadExt {
    fn left_stick(&self) -> Vec2;
    fn right_stick(&self) -> Vec2;
    fn d_pad(&self) -> Vec2;
}

impl GamepadExt for Gamepad<'_> {
    fn right_stick(&self) -> Vec2 {
        Vec2::new(self.value(Axis::RightStickX), self.value(Axis::RightStickY))
    }

    fn left_stick(&self) -> Vec2 {
        Vec2::new(self.value(Axis::LeftStickX), self.value(Axis::LeftStickY))
    }

    fn d_pad(&self) -> Vec2 {
        let up: f32 = self.is_pressed(Button::DPadUp).into();
        let down: f32 = self.is_pressed(Button::DPadDown).into();
        let y = up - down;

        let right: f32 = self.is_pressed(Button::DPadRight).into();
        let left: f32 = self.is_pressed(Button::DPadLeft).into();
        let x = right - left;

        Vec2::new(x, y)
    }
}
