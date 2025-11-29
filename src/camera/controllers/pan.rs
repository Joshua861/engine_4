use crate::prelude::*;

pub struct PanningCameraController {}

impl PanningCameraController {
    pub fn update(&mut self) {
        if mouse_held(MouseButton::Left) {
            let diff: Vec2 = mouse_diff().into();

            mutate_camera_2d(|camera| {
                camera.translation -= diff / camera.scale;
            });
        }

        if scroll_diff().1 != 0.0 {
            let diff = scroll_diff().1;
            let diff = (diff * 0.1) + 1.0;

            camera2d_zoom_at(cursor_pos(), diff);
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PanningCameraController {
    fn default() -> Self {
        Self::new()
    }
}
