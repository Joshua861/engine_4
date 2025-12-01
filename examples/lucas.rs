use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy_math::VectorSpace;
use engine_4::prelude::*;

const SIZES: [f32; 24] = [
    4.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 4.8, 4.4, 4.0, 3.6, 3.4, 3.2, 3.0, 2.8, 2.7, 2.3, 1.5,
    1.0, 1.0, 1.0, 1.0, 0.5,
];
const RADIUS: f32 = 2.0;
const NUM_JOINTS: usize = 1;
const NUM_LEGS: usize = 4;

#[derive(Debug)]
struct Leg {
    points: [Vec2; NUM_JOINTS + 2],
    previous_point: Vec2,
    anchor: usize,
    segment: usize,
}

impl Leg {
    pub fn new(anchor: usize, segment: usize) -> Self {
        Self {
            points: [Vec2::ZERO; NUM_JOINTS + 2],
            previous_point: Vec2::ZERO,
            anchor,
            segment,
        }
    }
}

#[derive(Debug)]
struct Lizard {
    legs: [Leg; NUM_LEGS],
    body_points: Vec<Vec2>,
    vertices: Vec<Vec2>,
    position: Vec2,
    draw_debug: bool,
}

impl Lizard {
    pub fn new() -> Self {
        let (body_points, vertices) = Self::gen_body_vertex_points();

        Self {
            legs: [
                Leg::new(10, 4),
                Leg::new(30, 12),
                Leg::new(11, 3),
                Leg::new(31, 12),
            ],
            body_points,
            vertices,
            position: Vec2::ZERO,
            draw_debug: false,
        }
    }

    pub fn gen_body_vertex_points() -> (Vec<Vec2>, Vec<Vec2>) {
        let body_points = Self::generate_body_points();
        let vertices = Self::generate_vertices(&body_points);
        (body_points, vertices)
    }

    fn generate_body_points() -> Vec<Vec2> {
        let num_points = SIZES.len();
        let mut body_points = vec![Vec2::ZERO; num_points];

        for i in 1..num_points {
            let prev = body_points[i - 1];
            let diff = body_points[i] - prev;
            let angle = atan2(diff.y, diff.x);

            body_points[i] =
                Vec2::new(prev.x + angle.sin() * RADIUS, prev.y + angle.cos() * RADIUS);
        }

        body_points
    }

    fn generate_vertices(body_points: &[Vec2]) -> Vec<Vec2> {
        let num_points = body_points.len();
        let mut vertices = Vec::new();

        for i in 0..num_points {
            let angle = Self::calculate_angle_at_point(body_points, i);

            if i == 0 {
                let divisors = [(1.0, 4.0), (1.0, 2.0), (-1.0, 2.0), (-1.0, 4.0)];
                let head_verts: Vec<Vec2> = divisors
                    .iter()
                    .map(|&(sign, divisor)| {
                        Self::create_radial_point(body_points[i], angle, sign, divisor, i)
                    })
                    .collect();

                vertices.extend(head_verts);
            } else {
                let left = Self::create_radial_point(body_points[i], angle, 1.0, 2.0, i);
                vertices.push(left);
            }
        }

        for i in (0..num_points).rev() {
            let angle = Self::calculate_angle_at_point(body_points, i);

            if i == 0 {
                continue;
            } else {
                let right = Self::create_radial_point(body_points[i], angle, -1.0, 2.0, i);
                vertices.push(right);
            }
        }

        vertices
    }

    fn calculate_angle_at_point(body_points: &[Vec2], i: usize) -> f32 {
        let prev = if i == 0 {
            body_points[body_points.len() - 1]
        } else {
            body_points[i - 1]
        };
        let diff = body_points[i] - prev;
        atan2(diff.y, diff.x)
    }

    fn create_radial_point(
        center: Vec2,
        angle: f32,
        sign: f32,
        divisor: f32,
        index: usize,
    ) -> Vec2 {
        let offset_angle = angle + (std::f32::consts::PI / divisor) * sign;
        Vec2::new(
            center.x + SIZES[index] * offset_angle.cos(),
            center.y + SIZES[index] * offset_angle.sin(),
        )
    }

    fn update(&mut self, mouse_pos: Vec2) {
        let world_mouse_pos = screen_to_world(mouse_pos);

        self.update_follow_mouse(world_mouse_pos);
        self.update_body_physics();
    }

    fn update_follow_mouse(&mut self, world_mouse_pos: Vec2) {
        let diff = world_mouse_pos - self.body_points[0];
        let distance = diff.length();

        if distance > 0.5 {
            let angle = atan2(diff.y, diff.x);
            self.body_points[0] += Vec2::new(angle.cos(), angle.sin()) * 0.9;
        }
    }

    fn update_body_physics(&mut self) {
        for i in 1..self.body_points.len() {
            let target = self.body_points[i - 1];
            let current = self.body_points[i];
            let diff = target - current;
            let distance = diff.length();

            if distance > 0.0 {
                let direction = diff / distance;
                self.body_points[i] = target - direction * RADIUS;
            }
        }

        self.vertices = Self::generate_vertices(&self.body_points);
    }

    fn update_legs(&mut self) {}

    fn update_camera(&self) {
        let pos = screen_to_world(self.vertices[0]);
        mutate_camera_2d(|camera| {
            camera.translation = pos;
        });
    }

    fn draw(&mut self) {
        draw_custom_shape_world(self.vertices.clone(), Color::GREEN_500);

        for point in &self.body_points {
            draw_circle_world(*point, 0.5, Color::RED_300);
        }

        let mut brightness = 0.0;
        for point in &self.vertices {
            brightness += 0.02;
            draw_circle_world(*point, 0.3, Color::hsl(40.0, 0.5, brightness));
        }
    }
}

fn main() -> anyhow::Result<()> {
    init("Lucas")?;
    let mut controller = PanningCameraController::new();
    let mut lizard = Lizard::new();

    loop {
        clear_screen(Color::NEUTRAL_100);
        controller.update();

        lizard.update(cursor_pos());
        lizard.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
