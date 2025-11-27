use bevy_math::Vec2;
use glium::winit::event::MouseButton;

use crate::api::{camera2d_zoom_at, cursor_pos, get_input, mutate_camera_2d};

pub struct PanningCameraController {}

impl PanningCameraController {
    pub fn update(&mut self) {
        let input = get_input();

        if input.mouse_held(MouseButton::Left) {
            let diff: Vec2 = input.mouse_diff().into();

            mutate_camera_2d(|camera| {
                camera.translation -= diff / camera.scale;
            });
        }

        if input.scroll_diff().1 != 0.0 {
            let diff = input.scroll_diff().1;
            let diff = (diff * 0.1) + 1.0;

            camera2d_zoom_at(cursor_pos(), diff);
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}
