use std::f32::consts::{FRAC_PI_2, TAU};

use bon::bon;
use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};
use sge_api::shapes_2d::{Shape2DExt, draw_sdf, draw_sdf_world};
use sge_color::Color;
use sge_rng::rand;
use sge_shapes::d2::Shape2D;
use sge_time::{time, time_since};
use sge_types::SdfInstance;
use sge_vectors::Vec2;

pub struct ParticleSystem {
    particles: Vec<Particle>,
    spawners: Vec<InstantiatedEmitter>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: vec![],
            spawners: vec![],
        }
    }

    pub fn spawn_oneshot(&mut self, batch: &ParticleOneshot, pos: Vec2) {
        batch.generate(&mut self.particles, pos);
    }

    pub fn spawn_emitter(&mut self, emitter: ParticleEmitter, pos: Vec2) {
        let duration = vary(emitter.duration, emitter.duration_randomness);
        self.spawners.push(InstantiatedEmitter {
            inner: emitter,
            created: time(),
            duration,
            timer: 0.0,
            next_time: 0.0,
            pos,
        });
    }

    pub fn spawn_single(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update(&mut self) {
        self.remove_completed_emitters();
        self.update_emitters();

        self.remove_dead();
        self.step_all();
    }

    pub fn draw(&mut self) {
        for particle in &self.particles {
            particle.draw();
        }
    }

    pub fn draw_world(&mut self) {
        for particle in &self.particles {
            particle.draw_world();
        }
    }

    fn remove_completed_emitters(&mut self) {
        self.spawners.retain(|s| !s.needs_removing());
    }

    fn update_emitters(&mut self) {
        let system = unsafe { &mut *(self as *mut Self) };
        for spawner in &mut self.spawners {
            spawner.update(system);
        }
    }

    fn remove_dead(&mut self) {
        let now = time();
        self.particles.retain(|p| now - p.spawn_time < p.lifetime);
    }

    fn step_all(&mut self) {
        self.particles.par_chunks_mut(1000).for_each(|particles| {
            for p in particles {
                p.step();
            }
        });
    }

    pub fn num_particles(&self) -> usize {
        self.particles.len()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Particle {
    pub shape: SdfInstance,
    pub pos: Vec2,
    pub additional_velocity: Vec2,
    pub angular_velocity: f32,
    pub speed: f32,
    pub direction: f32,
    pub acceleration: Vec2,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub spawn_time: f32,
    pub lifetime: f32,
    pub start_color: Color,
    pub end_color: Color,
}

unsafe impl Sync for Particle {}
unsafe impl Send for Particle {}

impl Particle {
    fn current_color(&self) -> Color {
        let time_alive = time() - self.spawn_time;
        let ratio = (time_alive / self.lifetime).clamp(0.0, 1.0);
        self.start_color.blend(self.end_color, ratio)
    }

    fn step(&mut self) {
        self.direction += self.angular_velocity;
        self.additional_velocity += self.acceleration;
        self.pos += self.additional_velocity;
        self.pos += Vec2::new(self.direction.cos(), self.direction.sin()) * self.speed;
        self.rotation += self.rotation_speed;
        self.shape.fill_color_a = self.current_color();
        self.shape.rotation = self.rotation;
        self.shape.set_position(self.pos);
    }

    fn draw(&self) {
        draw_sdf(self.shape);
    }

    fn draw_world(&self) {
        draw_sdf_world(self.shape);
    }
}

#[derive(Clone)]
pub struct ParticleOneshot {
    pub shape: SdfInstance,
    pub color_randomness: Color,
    pub quantity: usize,
    pub quantity_randomness: f32,
    pub size_randomness: f32,
    pub direction: f32,
    pub direction_randomness: f32,
    pub rotation_speed: f32,
    pub rotation_speed_randomness: f32,
    pub speed: f32,
    pub speed_randomness: f32,
    pub acceleration: Vec2,
    pub acceleration_randomness: Vec2,
    pub lifetime: f32,
    pub lifetime_randomness: f32,
    pub end_color: Color,
    pub angular_velocity: f32,
    pub angular_velocity_randomness: f32,
    pub position_randomness: Vec2,
}

#[bon]
impl ParticleOneshot {
    #[allow(clippy::too_many_arguments)]
    #[builder]
    pub fn builder(
        shape: &impl Shape2D,
        color_randomness: Option<Color>,
        quantity: Option<usize>,
        quantity_randomness: Option<f32>,
        size_randomness: Option<f32>,
        direction: Option<f32>,
        direction_randomness: Option<f32>,
        rotation_speed: Option<f32>,
        rotation_speed_randomness: Option<f32>,
        speed: Option<f32>,
        speed_randomness: Option<f32>,
        acceleration: Option<Vec2>,
        acceleration_randomness: Option<Vec2>,
        lifetime: Option<f32>,
        lifetime_randomness: Option<f32>,
        end_color: Option<Color>,
        angular_velocity: Option<f32>,
        angular_velocity_randomness: Option<f32>,
        position_randomness: Option<Vec2>,
    ) -> Self {
        Self {
            shape: shape.sdf(),
            color_randomness: color_randomness.unwrap_or(Color::BLACK),
            quantity: quantity.unwrap_or(10),
            quantity_randomness: quantity_randomness.unwrap_or(0.0),
            size_randomness: size_randomness.unwrap_or(0.0),
            direction: direction.unwrap_or(0.0),
            direction_randomness: direction_randomness.unwrap_or(0.0),
            rotation_speed: rotation_speed.unwrap_or(0.0),
            rotation_speed_randomness: rotation_speed_randomness.unwrap_or(0.0),
            speed: speed.unwrap_or(5.0),
            speed_randomness: speed_randomness.unwrap_or(0.0),
            acceleration: acceleration.unwrap_or(Vec2::ZERO),
            acceleration_randomness: acceleration_randomness.unwrap_or(Vec2::ZERO),
            lifetime: lifetime.unwrap_or(1.0),
            lifetime_randomness: lifetime_randomness.unwrap_or(0.0),
            end_color: end_color.unwrap_or(Color::BLACK),
            angular_velocity: angular_velocity.unwrap_or(0.0),
            angular_velocity_randomness: angular_velocity_randomness.unwrap_or(0.0),
            position_randomness: position_randomness.unwrap_or(Vec2::ZERO),
        }
    }

    fn generate(&self, vec: &mut Vec<Particle>, pos: Vec2) {
        let quantity = self.quantity as f32 + (rand::<f32>() - 0.5) * self.quantity_randomness;

        for _ in 0..quantity as usize {
            let mut shape = self.shape;
            let direction =
                self.direction + (rand::<f32>() - 0.5) * self.direction_randomness - FRAC_PI_2;
            let speed = self.speed + (rand::<f32>() - 0.5) * self.speed_randomness;
            let rotation_speed =
                self.rotation_speed + (rand::<f32>() - 0.5) * self.rotation_speed_randomness;
            let color = shape.fill_color_a;
            let color = Color::from_rgba(
                color.r + (rand::<f32>() - 0.5) * self.color_randomness.r,
                color.g + (rand::<f32>() - 0.5) * self.color_randomness.g,
                color.b + (rand::<f32>() - 0.5) * self.color_randomness.b,
                color.a + (rand::<f32>() - 0.5) * self.color_randomness.a,
            );
            shape.set_color(color);
            shape.set_position(pos);
            let initial_rotation =
                if self.rotation_speed == 0.0 && self.rotation_speed_randomness == 0.0 {
                    rand::<f32>() * TAU
                } else {
                    0.0
                };
            let acceleration = self.acceleration
                + Vec2::new(
                    (rand::<f32>() - 0.5) * self.acceleration_randomness.x,
                    (rand::<f32>() - 0.5) * self.acceleration_randomness.y,
                );
            let lifetime = self.lifetime + (rand::<f32>() - 0.5) * self.lifetime_randomness;
            let angular_velocity =
                self.angular_velocity + (rand::<f32>() - 0.5) * self.angular_velocity_randomness;
            let pos = pos
                + Vec2::new(
                    (rand::<f32>() - 0.5) * self.position_randomness.x,
                    (rand::<f32>() - 0.5) * self.position_randomness.y,
                );

            let particle = Particle {
                shape: self.shape,
                additional_velocity: Vec2::ZERO,
                direction,
                speed,
                acceleration,
                rotation: initial_rotation,
                rotation_speed,
                spawn_time: time(),
                lifetime,
                end_color: self.end_color,
                start_color: color,
                pos,
                angular_velocity,
            };

            vec.push(particle);
        }
    }
}

#[derive(Clone)]
pub struct ParticleEmitter {
    pub shape: SdfInstance,
    pub color_randomness: Color,
    pub size_randomness: f32,
    pub direction: f32,
    pub direction_randomness: f32,
    pub rotation_speed: f32,
    pub rotation_speed_randomness: f32,
    pub speed: f32,
    pub speed_randomness: f32,
    pub acceleration: Vec2,
    pub acceleration_randomness: Vec2,
    pub lifetime: f32,
    pub lifetime_randomness: f32,
    pub end_color: Color,
    pub angular_velocity: f32,
    pub angular_velocity_randomness: f32,
    pub position_randomness: Vec2,
    pub duration: f32,
    pub duration_randomness: f32,
    pub spawn_interval: f32,
    pub spawn_interval_randomness: f32,
}

#[bon]
impl ParticleEmitter {
    #[allow(clippy::too_many_arguments)]
    #[builder]
    pub fn new<T: Shape2D>(
        shape: T,
        color_randomness: Option<Color>,
        size_randomness: Option<f32>,
        direction: Option<f32>,
        direction_randomness: Option<f32>,
        rotation_speed: Option<f32>,
        rotation_speed_randomness: Option<f32>,
        speed: Option<f32>,
        speed_randomness: Option<f32>,
        acceleration: Option<Vec2>,
        acceleration_randomness: Option<Vec2>,
        lifetime: Option<f32>,
        lifetime_randomness: Option<f32>,
        end_color: Option<Color>,
        angular_velocity: Option<f32>,
        angular_velocity_randomness: Option<f32>,
        position_randomness: Option<Vec2>,
        duration: Option<f32>,
        duration_randomness: Option<f32>,
        spawn_interval: Option<f32>,
        spawn_interval_randomness: Option<f32>,
    ) -> Self {
        Self {
            shape: shape.sdf(),
            color_randomness: color_randomness.unwrap_or(Color::BLACK),
            size_randomness: size_randomness.unwrap_or(0.0),
            direction: direction.unwrap_or(0.0),
            direction_randomness: direction_randomness.unwrap_or(0.0),
            rotation_speed: rotation_speed.unwrap_or(0.0),
            rotation_speed_randomness: rotation_speed_randomness.unwrap_or(0.0),
            speed: speed.unwrap_or(5.0),
            speed_randomness: speed_randomness.unwrap_or(0.0),
            acceleration: acceleration.unwrap_or(Vec2::ZERO),
            acceleration_randomness: acceleration_randomness.unwrap_or(Vec2::ZERO),
            lifetime: lifetime.unwrap_or(1.0),
            lifetime_randomness: lifetime_randomness.unwrap_or(0.0),
            end_color: end_color.unwrap_or(Color::BLACK),
            angular_velocity: angular_velocity.unwrap_or(0.0),
            angular_velocity_randomness: angular_velocity_randomness.unwrap_or(0.0),
            position_randomness: position_randomness.unwrap_or(Vec2::ZERO),
            duration: duration.unwrap_or(1.0),
            duration_randomness: duration_randomness.unwrap_or(0.0),
            spawn_interval: spawn_interval.unwrap_or(0.1),
            spawn_interval_randomness: spawn_interval_randomness.unwrap_or(0.0),
        }
    }

    fn generate(&self, vec: &mut Vec<Particle>, pos: Vec2) {
        let direction = vary(self.direction, self.direction_randomness) - FRAC_PI_2;
        let speed = vary(self.speed, self.speed_randomness);
        let rotation_speed = vary(self.rotation_speed, self.rotation_speed_randomness);
        let mut shape = dyn_clone::clone(&self.shape);
        let color = shape.get_color();
        let color = Color::from_rgba(
            vary(color.r, self.color_randomness.r),
            vary(color.g, self.color_randomness.g),
            vary(color.b, self.color_randomness.b),
            vary(color.a, self.color_randomness.a),
        );
        shape.set_color(color);
        shape.set_position(pos);
        let initial_rotation =
            if self.rotation_speed == 0.0 && self.rotation_speed_randomness == 0.0 {
                rand::<f32>() * TAU
            } else {
                0.0
            };
        let acceleration = self.acceleration
            + Vec2::new(
                vary(self.acceleration.x, self.acceleration_randomness.x),
                vary(self.acceleration.y, self.acceleration_randomness.y),
            );
        let lifetime = self.lifetime + (rand::<f32>() - 0.5) * self.lifetime_randomness;
        let angular_velocity =
            self.angular_velocity + (rand::<f32>() - 0.5) * self.angular_velocity_randomness;
        let pos = pos
            + Vec2::new(
                vary(0.0, self.position_randomness.x),
                vary(0.0, self.position_randomness.y),
            );

        let particle = Particle {
            shape,
            additional_velocity: Vec2::ZERO,
            direction,
            speed,
            acceleration,
            rotation: initial_rotation,
            rotation_speed,
            spawn_time: time(),
            lifetime,
            end_color: self.end_color,
            start_color: color,
            pos,
            angular_velocity,
        };

        vec.push(particle);
    }
}

struct InstantiatedEmitter {
    inner: ParticleEmitter,
    created: f32,
    duration: f32,
    timer: f32,
    next_time: f32,
    pos: Vec2,
}

impl InstantiatedEmitter {
    fn update(&mut self, system: &mut ParticleSystem) {
        self.timer += sge_time::delta_time();

        while self.timer > self.next_time {
            self.timer -= self.next_time;
            self.next_time = self.time_between_spawns();

            self.inner.generate(&mut system.particles, self.pos);
        }
    }

    fn time_between_spawns(&self) -> f32 {
        vary(
            self.inner.spawn_interval,
            self.inner.spawn_interval_randomness,
        )
    }

    fn needs_removing(&self) -> bool {
        time_since(self.created) > self.duration
    }
}

fn vary(base: f32, randomness: f32) -> f32 {
    base + (rand::<f32>() - 0.5) * randomness
}
