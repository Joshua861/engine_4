use sge_api::shapes_2d::draw_circle_with_outline_world;
use sge_camera::{get_camera_2d, get_camera_2d_mut};
use sge_color::Color;
use sge_vectors::Vec2;
use sge_window::min_window_dimension;

pub struct FollowCameraController {
    pub margin_proportion: f32,
    pub pan_speed: f32,
}

impl FollowCameraController {
    pub fn new() -> Self {
        Self {
            pan_speed: 1.0,
            margin_proportion: 0.4,
        }
    }

    pub fn with_margin_proportion(mut self, margin_proportion: f32) -> Self {
        self.margin_proportion = margin_proportion;
        self
    }

    pub fn with_pan_speed(mut self, pan_speed: f32) -> Self {
        self.pan_speed = pan_speed;
        self
    }

    pub fn maybe_update(&mut self, position: Option<Vec2>) {
        if let Some(position) = position {
            self.update(position);
        }
    }

    pub fn update(&mut self, position: Vec2) {
        debug_assert!(
            (0.0..1.0).contains(&self.margin_proportion),
            "Follow camera controller margin proportion must be between 0.0 and 1.0"
        );

        let camera_position = get_camera_2d().translation();
        let camera_scale = get_camera_2d().scale();
        let max_dist =
            (min_window_dimension() * (1.0 - self.margin_proportion)) / camera_scale * 0.5;
        let (normal, len) = (position - camera_position).normalize_and_length();

        if len > max_dist {
            let pos = -normal * max_dist + position;
            get_camera_2d_mut().set_translation(pos);
        }
    }

    pub fn debug_show_margins(&self) {
        let camera_position = get_camera_2d().translation();
        let camera_scale = get_camera_2d().scale();
        let max_dist =
            (min_window_dimension() * (1.0 - self.margin_proportion)) / camera_scale * 0.5;

        draw_circle_with_outline_world(
            camera_position,
            max_dist,
            Color::SKY_500.with_alpha(0.1),
            Color::SKY_500.with_alpha(0.3),
            10.0,
        );
    }
}
