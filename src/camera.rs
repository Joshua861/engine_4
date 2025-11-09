use glam::{Mat3, Mat4, Vec2, Vec4};
use glium::winit::window::Window;

#[derive(Clone, Debug)]
pub struct Camera {
    pub translation: Vec2,
    pub scale: f32,
    pub rotation: f32,

    window_size: Vec2,

    view_matrix: Mat3,
    inverse_view_matrix: Mat3,
    projection_matrix: Mat4,
    needs_update: bool,
}

impl Camera {
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
            -1.0,
            1.0,
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

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    fn create_test_camera() -> Camera {
        Camera::new(800, 600)
    }

    #[test]
    fn test_camera_initialization() {
        let camera = create_test_camera();
        assert_eq!(camera.translation, Vec2::ZERO);
        assert_eq!(camera.scale, 1.0);
        assert_eq!(camera.rotation, 0.0);
        assert_eq!(camera.window_size(), Vec2::new(800.0, 600.0));
    }

    #[test]
    fn test_screen_center() {
        let camera = create_test_camera();
        let center = camera.window_size() * 0.5;
        assert_eq!(center, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_screen_to_world_identity() {
        let mut camera = create_test_camera();
        let screen_pos = Vec2::new(400.0, 300.0); // center
        let world_pos = camera.screen_to_world(screen_pos);
        assert_eq!(world_pos, Vec2::ZERO);
    }

    #[test]
    fn test_world_to_screen_identity() {
        let mut camera = create_test_camera();
        let world_pos = Vec2::ZERO;
        let screen_pos = camera.world_to_screen(world_pos);
        assert_eq!(screen_pos, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_screen_world_roundtrip() {
        let mut camera = create_test_camera();
        let original = Vec2::new(100.0, 200.0);
        let world = camera.screen_to_world(original);
        let back = camera.world_to_screen(world);

        assert!((original.x - back.x).abs() < 0.001);
        assert!((original.y - back.y).abs() < 0.001);
    }

    #[test]
    fn test_translation() {
        let mut camera = create_test_camera();
        camera.translation = Vec2::new(100.0, 50.0);
        camera.mark_dirty();

        let world_pos = camera.screen_to_world(Vec2::new(400.0, 300.0));
        assert_eq!(world_pos, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn test_scale_doubles() {
        let mut camera = create_test_camera();
        camera.scale = 2.0;
        camera.mark_dirty();

        let screen_pos = Vec2::new(500.0, 300.0); // 100px right of center
        let world_pos = camera.screen_to_world(screen_pos);
        assert_eq!(world_pos, Vec2::new(50.0, 0.0)); // Should be 50 units in world
    }

    #[test]
    fn test_scale_halves() {
        let mut camera = create_test_camera();
        camera.scale = 0.5;
        camera.mark_dirty();

        let screen_pos = Vec2::new(500.0, 300.0); // 100px right of center
        let world_pos = camera.screen_to_world(screen_pos);
        assert_eq!(world_pos, Vec2::new(200.0, 0.0)); // Should be 200 units in world
    }

    #[test]
    fn test_zoom_at_maintains_point() {
        let mut camera = create_test_camera();
        let screen_point = Vec2::new(500.0, 400.0);
        let world_before = camera.screen_to_world(screen_point);

        camera.zoom_at(screen_point, 2.0);

        let world_after = camera.screen_to_world(screen_point);

        assert!((world_before.x - world_after.x).abs() < 0.1);
        assert!((world_before.y - world_after.y).abs() < 0.1);
    }

    #[test]
    fn test_visible_bounds_no_transform() {
        let mut camera = create_test_camera();
        let (min, max) = camera.visible_bounds();

        assert_eq!(min, Vec2::new(-400.0, -300.0));
        assert_eq!(max, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_visible_bounds_with_translation() {
        let mut camera = create_test_camera();
        camera.translation = Vec2::new(100.0, 50.0);
        camera.mark_dirty();

        let (min, max) = camera.visible_bounds();

        assert_eq!(min, Vec2::new(-300.0, -250.0));
        assert_eq!(max, Vec2::new(500.0, 350.0));
    }

    #[test]
    fn test_visible_bounds_with_scale() {
        let mut camera = create_test_camera();
        camera.scale = 2.0;
        camera.mark_dirty();

        let (min, max) = camera.visible_bounds();

        assert_eq!(min, Vec2::new(-200.0, -150.0));
        assert_eq!(max, Vec2::new(200.0, 150.0));
    }

    #[test]
    fn test_distance_conversions() {
        let camera = create_test_camera();
        let world_dist = 100.0;
        let screen_dist = camera.world_distance_to_screen(world_dist);
        assert_eq!(screen_dist, 100.0);

        let back = camera.screen_distance_to_world(screen_dist);
        assert_eq!(back, world_dist);
    }

    #[test]
    fn test_distance_conversions_scaled() {
        let mut camera = create_test_camera();
        camera.scale = 2.0;

        let world_dist = 100.0;
        let screen_dist = camera.world_distance_to_screen(world_dist);
        assert_eq!(screen_dist, 200.0);

        let back = camera.screen_distance_to_world(screen_dist);
        assert_eq!(back, world_dist);
    }

    #[test]
    fn test_update_sizes() {
        let mut camera = create_test_camera();
        camera.update_sizes(1024, 768);

        assert_eq!(camera.window_size(), Vec2::new(1024.0, 768.0));
    }

    #[test]
    fn test_multiple_transformations() {
        let mut camera = create_test_camera();
        camera.translation = Vec2::new(50.0, 25.0);
        camera.scale = 1.5;
        camera.mark_dirty();

        let screen_pos = Vec2::new(500.0, 400.0);
        let world_pos = camera.screen_to_world(screen_pos);
        let back = camera.world_to_screen(world_pos);

        assert!((screen_pos.x - back.x).abs() < 0.001);
        assert!((screen_pos.y - back.y).abs() < 0.001);
    }
}
