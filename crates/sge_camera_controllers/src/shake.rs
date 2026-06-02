use sge_camera::{get_camera_2d, get_camera_2d_mut};
use sge_time::{delta_time, time};
use sge_vectors::Vec2;

pub struct CameraShakeController {
    trauma: f32,
    decay: f32,
    max_offset: f32,
    frequency: f32,
}

impl CameraShakeController {
    pub fn new(max_offset: f32, frequency: f32, decay: f32) -> Self {
        Self {
            trauma: 0.0,
            decay,
            max_offset,
            frequency,
        }
    }

    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }

    pub fn update(&mut self) {
        self.trauma = (self.trauma - self.decay * delta_time()).max(0.0);

        let shake = self.trauma * self.trauma;

        let offset = if shake > 0.0 {
            Vec2::new(
                self.max_offset * shake * (smooth_noise(time() * self.frequency) - 0.5) * 2.0,
                self.max_offset
                    * shake
                    * (smooth_noise(time() * self.frequency + 999.0) - 0.5)
                    * 2.0,
            ) / get_camera_2d().scale()
        } else {
            Vec2::ZERO
        };

        get_camera_2d_mut().set_offset(offset);
    }
}

fn smooth_noise(t: f32) -> f32 {
    let i = t.floor() as i32;
    let f = t.fract();
    let u = f * f * (3.0 - 2.0 * f);
    lerp(hash(i), hash(i + 1), u)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn hash(n: i32) -> f32 {
    let x = n.wrapping_mul(1836311903).wrapping_add(521288629);
    let x = x ^ (x >> 14);
    let x = x.wrapping_mul(1685821657);
    let x = x ^ (x >> 16);
    x as f32 / i32::MAX as f32
}
