use glium::winit::event::MouseButton;
use sge_input::get_input;
use sge_vectors::{Vec2, Vec3};

use sge_camera::get_camera_3d_mut;

pub struct OrbitCameraController {
    pub target: Vec3,
    pub distance: f32,
    pub theta: f32, // horizontal angle
    pub phi: f32,   // vertical angle
    pub is_orbiting: bool,
    pub last_mouse_pos: Vec2,
    pub enabled: bool,
    pub sensitivity: f32,
    pub zoom_speed: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub min_phi: f32,
    pub max_phi: f32,
}

impl OrbitCameraController {
    pub fn new(target: Vec3) -> Self {
        Self {
            target,
            distance: 5.0,
            theta: 0.0,
            phi: std::f32::consts::FRAC_PI_4,
            is_orbiting: false,
            last_mouse_pos: Vec2::ZERO,
            enabled: true,
            sensitivity: 0.003,
            zoom_speed: 0.5,
            min_distance: 1.0,
            max_distance: 100.0,
            min_phi: 0.1,
            max_phi: std::f32::consts::PI - 0.1,
        }
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance.clamp(self.min_distance, self.max_distance);
    }

    pub fn set_angles(&mut self, theta: f32, phi: f32) {
        self.theta = theta;
        self.phi = phi.clamp(self.min_phi, self.max_phi);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }

    pub fn set_zoom_speed(&mut self, zoom_speed: f32) {
        self.zoom_speed = zoom_speed;
    }

    pub fn set_distance_limits(&mut self, min: f32, max: f32) {
        self.min_distance = min;
        self.max_distance = max;
        self.distance = self.distance.clamp(min, max);
    }

    pub fn set_angle_limits(&mut self, min_phi: f32, max_phi: f32) {
        self.min_phi = min_phi;
        self.max_phi = max_phi;
        self.phi = self.phi.clamp(min_phi, max_phi);
    }

    pub fn update(&mut self) {
        let input = &get_input();
        if !self.enabled {
            return;
        }

        if input.mouse_pressed(MouseButton::Left) {
            self.is_orbiting = true;
            if let Some((x, y)) = input.cursor() {
                self.last_mouse_pos = Vec2::new(x, y);
            }
        }

        if input.mouse_released(MouseButton::Left) {
            self.is_orbiting = false;
        }

        if self.is_orbiting
            && let Some((x, y)) = input.cursor()
        {
            let current_mouse_pos = Vec2::new(x, y);
            let mut mouse_diff = current_mouse_pos - self.last_mouse_pos;
            mouse_diff.y = -mouse_diff.y;

            self.theta += mouse_diff.x * self.sensitivity;
            self.phi += mouse_diff.y * self.sensitivity;

            self.phi = self.phi.clamp(self.min_phi, self.max_phi);

            self.last_mouse_pos = current_mouse_pos;
        }

        let diff = input.scroll_diff();
        if diff.1 != 0.0 {
            let scroll = diff.1;
            self.distance -= scroll * self.zoom_speed;
            self.distance = self.distance.clamp(self.min_distance, self.max_distance);
        }

        self.apply_to_camera();
    }

    pub fn apply_to_camera(&self) {
        if !self.enabled {
            return;
        }

        let x = self.distance * self.phi.sin() * self.theta.cos();
        let y = self.distance * self.phi.cos();
        let z = self.distance * self.phi.sin() * self.theta.sin();

        let camera = get_camera_3d_mut();
        *camera.eye_mut() = Vec3::new(x, y, z) + self.target;
        *camera.target_mut() = self.target;
    }

    pub fn get_camera_position(&self) -> Vec3 {
        let x = self.distance * self.phi.sin() * self.theta.cos();
        let y = self.distance * self.phi.cos();
        let z = self.distance * self.phi.sin() * self.theta.sin();

        self.target + Vec3::new(x, y, z)
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_angles(&self) -> (f32, f32) {
        (self.theta, self.phi)
    }
}
