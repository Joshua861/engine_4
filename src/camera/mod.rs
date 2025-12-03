use bevy_math::{Mat3, Mat4, Vec2, Vec3, Vec4};
use glium::winit::window::Window;

use crate::EngineState;
const BIG_NUMBER: f32 = 9999.9;

pub mod controllers;

#[derive(Clone, Debug, Copy)]
pub struct Camera2D {
    pub translation: Vec2,
    pub scale: f32,
    pub rotation: f32,

    window_size: Vec2,

    view_matrix: Mat3,
    inverse_view_matrix: Mat3,
    projection_matrix: Mat4,
    needs_update: bool,
}

#[derive(Clone, Debug, Copy)]
pub(crate) struct Cameras {
    pub flat: Mat4,
    pub d2: Camera2D,
    pub d3: Camera3D,
}

impl EngineState {
    pub fn cameras_for_resolution(&self, width: u32, height: u32) -> Cameras {
        let mut d2 = self.camera_2d;
        d2.update_sizes(width, height);
        let flat = projection(width, height);
        let mut d3 = self.camera_3d;
        d3.update_sizes(width, height);

        Cameras { flat, d2, d3 }
    }

    pub fn cameras(&self) -> Cameras {
        Cameras {
            flat: self.flat_projection,
            d2: self.camera_2d,
            d3: self.camera_3d,
        }
    }
}

impl Camera2D {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let mut camera = Self {
            translation: Vec2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            window_size: Vec2::new(window_width as f32, window_height as f32),
            view_matrix: Mat3::IDENTITY,
            inverse_view_matrix: Mat3::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            needs_update: true,
        };
        camera.update_matrices();
        camera
    }

    pub fn from_window(window: &Window) -> Self {
        let size = window.inner_size();
        Self::new(size.width, size.height)
    }

    pub fn update_sizes(&mut self, window_width: u32, window_height: u32) {
        self.window_size = Vec2::new(window_width as f32, window_height as f32);
        self.needs_update = true;
    }

    pub fn mark_dirty(&mut self) {
        self.needs_update = true;
    }

    pub fn update_matrices(&mut self) {
        if !self.needs_update {
            return;
        }

        let translation_matrix = Mat3::from_translation(-self.translation);
        let rotation_matrix = Mat3::from_angle(-self.rotation);
        let scale_matrix = Mat3::from_scale(Vec2::splat(self.scale));

        self.view_matrix = scale_matrix * rotation_matrix * translation_matrix;
        self.inverse_view_matrix = self.view_matrix.inverse();

        self.projection_matrix = self.generate_projection_matrix();

        self.needs_update = false;
    }

    fn screen_center(&self) -> Vec2 {
        self.window_size * 0.5
    }

    pub fn screen_to_world(&mut self, screen_pos: Vec2) -> Vec2 {
        self.update_matrices();

        let centered = screen_pos - self.screen_center();
        let world_pos_homogeneous = self.inverse_view_matrix * centered.extend(1.0);

        world_pos_homogeneous.truncate()
    }

    pub fn world_to_screen(&mut self, world_pos: Vec2) -> Vec2 {
        self.update_matrices();

        let camera_pos_homogeneous = self.view_matrix * world_pos.extend(1.0);
        let camera_pos = camera_pos_homogeneous.truncate();

        camera_pos + self.screen_center()
    }

    pub fn visible_bounds(&mut self) -> (Vec2, Vec2) {
        self.update_matrices();

        let top_left = self.screen_to_world(Vec2::ZERO);
        let top_right = self.screen_to_world(Vec2::new(self.window_size.x, 0.0));
        let bottom_left = self.screen_to_world(Vec2::new(0.0, self.window_size.y));
        let bottom_right = self.screen_to_world(self.window_size);

        let min = top_left.min(top_right).min(bottom_left).min(bottom_right);
        let max = top_left.max(top_right).max(bottom_left).max(bottom_right);

        (min, max)
    }

    pub fn world_distance_to_screen(&self, world_distance: f32) -> f32 {
        world_distance * self.scale
    }

    pub fn screen_distance_to_world(&self, screen_distance: f32) -> f32 {
        screen_distance / self.scale
    }

    pub fn zoom_at(&mut self, screen_pos: Vec2, zoom_factor: f32) {
        let world_pos = self.screen_to_world(screen_pos);
        self.scale *= zoom_factor;
        self.mark_dirty();

        let new_screen_pos = self.world_to_screen(world_pos);
        let screen_delta = screen_pos - new_screen_pos;
        let world_delta = screen_delta / self.scale;
        self.translation -= world_delta;
        self.mark_dirty();
    }

    pub fn window_size(&self) -> Vec2 {
        self.window_size
    }

    fn generate_projection_matrix(&mut self) -> Mat4 {
        let half_width = self.window_size.x * 0.5;
        let half_height = self.window_size.y * 0.5;

        let ortho = Mat4::orthographic_rh(
            -half_width,
            half_width,
            half_height,
            -half_height,
            -BIG_NUMBER,
            BIG_NUMBER,
        );

        let view_mat4 = Mat4::from_cols(
            self.view_matrix.x_axis.extend(0.0),
            self.view_matrix.y_axis.extend(0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            self.view_matrix.z_axis.extend(1.0),
        );

        ortho * view_mat4
    }

    pub fn projection_matrix(&mut self) -> Mat4 {
        self.update_matrices();

        self.projection_matrix
    }
}

pub fn projection_from_window(window: &Window) -> Mat4 {
    let size = window.inner_size();
    projection(size.width, size.height)
}

pub fn projection(width: u32, height: u32) -> Mat4 {
    Mat4::orthographic_rh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0)
}

#[derive(Clone, Copy, Debug)]
pub struct Camera3D {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub isometric: bool,
    window_size: Vec2,
    view_proj: Mat4,
    view_matrix: Mat4,
    proj_matrix: Mat4,
    needs_update: bool,
}

impl Camera3D {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let mut camera = Self {
            eye: Vec3::ZERO,
            target: Vec3::NEG_Z,
            up: Vec3::Y,
            window_size: Vec2::new(window_width as f32, window_height as f32),
            fovy: 100.0,
            znear: 0.1,
            zfar: 1000.0,
            isometric: false,
            needs_update: true,
            view_proj: Mat4::ZERO,
            view_matrix: Mat4::ZERO,
            proj_matrix: Mat4::ZERO,
        };
        camera.update_matrices();
        camera
    }

    /// Does nothing if self.needs_update is false
    pub fn update_matrices(&mut self) {
        if !self.needs_update {
            return;
        }
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);

        let proj = if self.isometric {
            let distance = (self.eye - self.target).length();
            let height = distance * (self.fovy.to_radians() / 2.0).tan();
            let width = height * self.window_aspect_ratio();

            Mat4::orthographic_rh(-width, width, -height, height, self.znear, self.zfar)
        } else {
            Mat4::perspective_rh(
                self.fovy.to_radians(),
                self.window_aspect_ratio(),
                self.znear,
                self.zfar,
            )
        };

        self.view_matrix = view;
        self.proj_matrix = proj;
        self.view_proj = proj * view;
        self.needs_update = false;
    }

    pub fn view_proj(&mut self) -> Mat4 {
        self.update_matrices();
        self.view_proj
    }

    pub fn window_aspect_ratio(&self) -> f32 {
        self.window_size.x / self.window_size.y
    }

    pub fn from_window(window: &Window) -> Self {
        let size = window.inner_size();
        Self::new(size.width, size.height)
    }

    pub fn mark_dirty(&mut self) {
        self.needs_update = true;
    }

    pub fn update_sizes(&mut self, window_width: u32, window_height: u32) {
        self.window_size = Vec2::new(window_width as f32, window_height as f32);
        self.needs_update = true;
    }
}
