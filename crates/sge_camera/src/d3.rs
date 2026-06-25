use glium::winit::window::Window;
use sge_math::ray::Ray3D;
use sge_vectors::{Mat4, Quat, Vec2, Vec3, Vec4, Vec4Swizzles};

#[derive(Clone, Copy, Debug)]
pub struct Camera3D {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    fovy: f32,
    znear: f32,
    zfar: f32,
    isometric: bool,

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
            fovy: 60.0,
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

    pub fn world_to_screen(&mut self, world_pos: Vec3) -> Option<Vec2> {
        let clip = self.view_proj() * world_pos.extend(1.0);

        if clip.w <= 0.0 {
            return None;
        }

        let ndc = clip.xyz() / clip.w;

        if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 || ndc.z < -1.0 || ndc.z > 1.0
        {
            return None;
        }

        let screen = Vec2::new(
            (ndc.x + 1.0) * 0.5 * self.window_size.x,
            (1.0 - ndc.y) * 0.5 * self.window_size.y,
        );

        Some(screen)
    }

    pub fn screen_to_world(&mut self, screen_pos: Vec2) -> Ray3D {
        let ndc_x = (screen_pos.x / self.window_size.x) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen_pos.y / self.window_size.y) * 2.0;
        let ndc = Vec4::new(ndc_x, ndc_y, -1.0, 1.0);

        let inv_view_proj = self.view_proj().inverse();
        let world_pos = inv_view_proj * ndc;
        let world_pos = world_pos.xyz() / world_pos.w;

        Ray3D::new(self.eye, world_pos - self.eye)
    }

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

    pub fn view_matrix(&mut self) -> Mat4 {
        self.update_matrices();
        self.view_matrix
    }

    pub fn proj_matrix(&mut self) -> Mat4 {
        self.update_matrices();
        self.proj_matrix
    }

    pub fn window_aspect_ratio(&self) -> f32 {
        self.window_size.x / self.window_size.y
    }

    pub fn window_size(&self) -> Vec2 {
        self.window_size
    }

    pub fn forward(&self) -> Vec3 {
        (self.target - self.eye).normalize_or_zero()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize_or_zero()
    }

    pub fn local_up(&self) -> Vec3 {
        self.right().cross(self.forward()).normalize_or_zero()
    }

    pub fn distance(&self) -> f32 {
        (self.target - self.eye).length()
    }

    pub fn move_forward(&mut self, amount: f32) {
        let delta = self.forward() * amount;
        self.eye += delta;
        self.target += delta;
        self.mark_dirty();
    }

    pub fn strafe(&mut self, amount: f32) {
        let delta = self.right() * amount;
        self.eye += delta;
        self.target += delta;
        self.mark_dirty();
    }

    pub fn move_up(&mut self, amount: f32) {
        let delta = self.up * amount;
        self.eye += delta;
        self.target += delta;
        self.mark_dirty();
    }

    pub fn translate_by(&mut self, delta: Vec3) {
        self.eye += delta;
        self.target += delta;
        self.mark_dirty();
    }

    pub fn set_eye(&mut self, new_eye: Vec3) {
        let offset = self.target - self.eye;
        self.eye = new_eye;
        self.target = new_eye + offset;
        self.mark_dirty();
    }

    pub fn lerp_to(&mut self, target_eye: Vec3, t: f32) {
        let offset = self.target - self.eye;
        self.eye = self.eye.lerp(target_eye, t);
        self.target = self.eye + offset;
        self.mark_dirty();
    }

    pub fn look_at(&mut self, world_pos: Vec3) {
        self.target = world_pos;
        self.mark_dirty();
    }

    pub fn lerp_look_at(&mut self, new_target: Vec3, t: f32) {
        self.target = self.target.lerp(new_target, t);
        self.mark_dirty();
    }

    pub fn yaw(&mut self, radians: f32) {
        let rotation = Quat::from_axis_angle(self.up, radians);
        let offset = rotation * (self.target - self.eye);
        self.target = self.eye + offset;
        self.mark_dirty();
    }

    pub fn pitch(&mut self, radians: f32) {
        let right = self.right();
        let rotation = Quat::from_axis_angle(right, radians);
        let offset = rotation * (self.target - self.eye);

        let new_forward = offset.normalize_or_zero();
        if new_forward.dot(self.up).abs() < 0.999 {
            self.target = self.eye + offset;
            self.mark_dirty();
        }
    }

    pub fn roll(&mut self, radians: f32) {
        let forward = self.forward();
        let rotation = Quat::from_axis_angle(forward, radians);
        self.up = rotation * self.up;
        self.mark_dirty();
    }

    pub fn orbit(&mut self, yaw_radians: f32, pitch_radians: f32) {
        let offset = self.eye - self.target;

        let yaw_rot = Quat::from_axis_angle(Vec3::Y, yaw_radians);
        let offset = yaw_rot * offset;

        let right = offset.cross(Vec3::Y).normalize_or_zero();
        let pitch_rot = Quat::from_axis_angle(right, pitch_radians);
        let offset = pitch_rot * offset;

        let new_forward = (-offset).normalize_or_zero();
        if new_forward.dot(Vec3::Y).abs() < 0.999 {
            self.eye = self.target + offset;
            self.mark_dirty();
        }
    }

    pub fn pan_orbit(&mut self, yaw: f32, pitch: f32, zoom_factor: f32) {
        self.orbit(yaw, pitch);
        self.zoom(zoom_factor);
    }

    pub fn zoom(&mut self, factor: f32) {
        let offset = (self.eye - self.target) * factor;
        self.eye = self.target + offset;
        self.mark_dirty();
    }

    pub fn dolly(&mut self, amount: f32) {
        let dir = (self.eye - self.target).normalize_or_zero();
        self.eye += dir * amount;
        self.mark_dirty();
    }

    pub fn pan(&mut self, right_amount: f32, up_amount: f32) {
        let delta = self.right() * right_amount + self.local_up() * up_amount;
        self.eye += delta;
        self.target += delta;
        self.mark_dirty();
    }

    pub fn set_fov(&mut self, fovy_degrees: f32) {
        self.fovy = fovy_degrees.clamp(1.0, 170.0);
        self.mark_dirty();
    }

    pub fn zoom_fov(&mut self, factor: f32) {
        self.set_fov(self.fovy * factor);
    }

    pub fn toggle_isometric(&mut self) {
        self.isometric = !self.isometric;
        self.mark_dirty();
    }

    pub fn set_clip_planes(&mut self, znear: f32, zfar: f32) {
        self.znear = znear;
        self.zfar = zfar;
        self.mark_dirty();
    }

    pub fn eye(&self) -> Vec3 {
        self.eye
    }

    pub fn eye_mut(&mut self) -> &mut Vec3 {
        self.mark_dirty();
        &mut self.eye
    }

    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn target_mut(&mut self) -> &mut Vec3 {
        self.mark_dirty();
        &mut self.target
    }

    pub fn fovy(&self) -> f32 {
        self.fovy
    }

    pub fn fovy_mut(&mut self) -> &mut f32 {
        self.mark_dirty();
        &mut self.fovy
    }

    pub fn set_fovy(&mut self, fovy_degrees: f32) {
        self.fovy = fovy_degrees.clamp(1.0, 170.0);
        self.mark_dirty();
    }

    pub fn isometric(&self) -> bool {
        self.isometric
    }

    pub fn isometric_mut(&mut self) -> &mut bool {
        self.mark_dirty();
        &mut self.isometric
    }

    pub fn set_isometric(&mut self, isometric: bool) {
        self.isometric = isometric;
        self.mark_dirty();
    }

    pub fn offset_from_target(&self) -> Vec3 {
        self.eye - self.target
    }
}
