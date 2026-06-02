use glium::winit::window::Window;
use sge_vectors::{Mat3, Mat4, Rect, Vec2, Vec4, vec2};

const BIG_NUMBER: f32 = 9999.9;

#[derive(Clone, Debug, Copy)]
pub struct Camera2D {
    translation: Vec2,
    scale: f32,
    rotation: f32,

    offset: Vec2,
    flip_y: bool,

    window_size: Vec2,

    view_matrix: Mat3,
    pub(crate) inverse_view_matrix: Mat3,
    projection_matrix: Mat4,
    needs_update: bool,
    visible_bounds: (Vec2, Vec2),
}

impl Camera2D {
    pub fn new(window_width: u32, window_height: u32, flip_y: bool) -> Self {
        let mut camera = Self {
            translation: Vec2::ZERO,
            offset: Vec2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            window_size: Vec2::new(window_width as f32, window_height as f32),
            view_matrix: Mat3::IDENTITY,
            inverse_view_matrix: Mat3::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            visible_bounds: (Vec2::ZERO, Vec2::ZERO),
            needs_update: true,
            flip_y,
        };
        camera.update_matrices();
        camera
    }

    pub fn from_window(window: &Window, flip_y: bool) -> Self {
        let size = window.inner_size();
        Self::new(size.width, size.height, flip_y)
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

        self.needs_update = false;

        let sign = if self.flip_y { 1.0 } else { -1.0 };
        let translation_matrix =
            Mat3::from_translation((sign * self.translation) + (sign * self.offset));
        let rotation_matrix = Mat3::from_angle(sign * self.rotation);
        let scale_matrix = Mat3::from_scale(Vec2::splat(self.scale) * vec2(1.0, -sign));

        self.view_matrix = scale_matrix * rotation_matrix * translation_matrix;
        self.inverse_view_matrix = self.view_matrix.inverse();
        self.projection_matrix = self.generate_projection_matrix();

        self.visible_bounds = self.calculate_visible_bounds();
    }

    pub fn screen_center(&self) -> Vec2 {
        self.window_size * 0.5
    }

    pub fn screen_to_world(&mut self, screen_pos: Vec2) -> Vec2 {
        self.update_matrices();

        if self.scale == 0.0 {
            return self.translation;
        }

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
        self.visible_bounds
    }

    fn calculate_visible_bounds(&mut self) -> (Vec2, Vec2) {
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

    pub fn translate_by(&mut self, delta: Vec2) {
        self.translation += delta;
        self.mark_dirty();
    }

    pub fn pan_to(&mut self, world_pos: Vec2) {
        self.translation = world_pos;
        self.mark_dirty();
    }

    pub fn pan_by_screen(&mut self, screen_delta: Vec2) {
        let world_delta = self.screen_distance_to_world(1.0) * screen_delta;

        let angle = if self.flip_y {
            self.rotation
        } else {
            -self.rotation
        };
        let cos = angle.cos();
        let sin = angle.sin();
        let rotated = Vec2::new(
            world_delta.x * cos - world_delta.y * sin,
            world_delta.x * sin + world_delta.y * cos,
        );
        self.translate_by(rotated);
    }

    pub fn lerp_to(&mut self, target: Vec2, t: f32) {
        self.translation = self.translation.lerp(target, t);
        self.mark_dirty();
    }

    pub fn clamp_to_bounds(&mut self, min: Vec2, max: Vec2) {
        self.translation = self.translation.clamp(min, max);
        self.mark_dirty();
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

    pub fn set_scale(&mut self, new_scale: f32) {
        let center = self.screen_center();
        let factor = new_scale / self.scale.max(f32::EPSILON);
        self.zoom_at(center, factor);
    }

    pub fn zoom_to_fit(&mut self, rect: Rect, padding: f32) {
        let world_w = rect.width() + padding * 2.0;
        let world_h = rect.height() + padding * 2.0;
        let scale_x = self.window_size.x / world_w.max(f32::EPSILON);
        let scale_y = self.window_size.y / world_h.max(f32::EPSILON);
        self.scale = scale_x.min(scale_y);
        self.translation = rect.center();
        self.mark_dirty();
    }

    pub fn lerp_zoom_to(&mut self, target_scale: f32, t: f32) {
        self.scale = self.scale + (target_scale - self.scale) * t;
        self.mark_dirty();
    }

    pub fn rotate_by(&mut self, delta_radians: f32) {
        self.rotation += delta_radians;
        self.mark_dirty();
    }

    pub fn set_rotation(&mut self, radians: f32) {
        self.rotation = radians;
        self.mark_dirty();
    }

    pub fn rotate_around(&mut self, pivot: Vec2, delta_radians: f32) {
        let to_pivot = self.translation - pivot;
        let cos = delta_radians.cos();
        let sin = delta_radians.sin();
        let rotated = Vec2::new(
            to_pivot.x * cos - to_pivot.y * sin,
            to_pivot.x * sin + to_pivot.y * cos,
        );
        self.translation = pivot + rotated;
        self.rotation += delta_radians;
        self.mark_dirty();
    }

    pub fn clear_offset(&mut self) {
        self.offset = Vec2::ZERO;
        self.mark_dirty();
    }

    pub fn center(&self) -> Vec2 {
        self.translation
    }

    pub fn contains_point(&mut self, world_pos: Vec2) -> bool {
        let (min, max) = self.visible_bounds();
        world_pos.cmpge(min).all() && world_pos.cmple(max).all()
    }

    pub fn overlaps_aabb(&mut self, min: Vec2, max: Vec2) -> bool {
        let (vmin, vmax) = self.visible_bounds();
        min.x <= vmax.x && max.x >= vmin.x && min.y <= vmax.y && max.y >= vmin.y
    }

    pub fn visible_rect(&mut self) -> Rect {
        let (min, max) = self.visible_bounds();
        Rect::from_corners(min, max)
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

    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    pub fn offset_mut(&mut self) -> &mut Vec2 {
        self.mark_dirty();
        &mut self.offset
    }

    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
        self.mark_dirty();
    }

    pub fn offset_by(&mut self, delta: Vec2) {
        self.offset += delta;
        self.mark_dirty();
    }

    pub fn translation(&self) -> Vec2 {
        self.translation
    }

    pub fn set_translation(&mut self, translation: Vec2) {
        self.translation = translation;
        self.mark_dirty();
    }

    pub fn translation_mut(&mut self) -> &mut Vec2 {
        self.mark_dirty();
        &mut self.translation
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn scale_mut(&mut self) -> &mut f32 {
        self.mark_dirty();
        &mut self.scale
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn rotation_mut(&mut self) -> &mut f32 {
        self.mark_dirty();
        &mut self.rotation
    }

    pub fn flip_y(&self) -> bool {
        self.flip_y
    }

    pub fn set_flip_y(&mut self, flip: bool) {
        if self.flip_y != flip {
            self.flip_y = flip;
            self.mark_dirty();
            self.update_matrices();
        }
    }
}

pub fn projection_from_window(window: &Window, flip_y: bool) -> Mat4 {
    let size = window.inner_size();
    projection(size.width, size.height, flip_y)
}

pub fn projection(width: u32, height: u32, flip_y: bool) -> Mat4 {
    if flip_y {
        Mat4::orthographic_rh(0.0, width as f32, 0.0, height as f32, -1.0, 1.0)
    } else {
        Mat4::orthographic_rh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0)
    }
}
