use std::f32::consts::{FRAC_PI_2, TAU};

use sge::prelude::*;

const BASE_WINDOW_SIZE: Vec2 = Vec2::new(3840., 2160.);
const BASE_SCALE: f32 = 3.0;
const PLAYER_ACCEL: f32 = 0.5;
const PLAYER_TURN_SPEED: f32 = 0.1;
const PLAYER_SIZE: f32 = 10.0;
const FG: Color = Color::NEUTRAL_100;
const BG: Color = Color::NEUTRAL_950;
const RED: Color = Color::RED_500;
const PLAYER_LIN_DECEL: f32 = 0.9;
const PLAYER_ANG_DECEL: f32 = 0.9;
const LINE_THICKNESS: f32 = 1.0;
const MIN_ASTEROID_RADIUS: f32 = PLAYER_SIZE;
const MAX_ASTEROID_RADIUS: f32 = MIN_ASTEROID_RADIUS * 5.0;
const MAX_ASTEROID_SPEED: f32 = 1.0;
const MAX_ASTEROID_ANGVEL: f32 = 0.01;
const MAX_ASTEROID_POINT_OFFSET: f32 = 0.5;
const MAX_ASTEROID_POINTS: f32 = 0.5;
const NUM_ASTEROIDS: usize = 10;
const MAX_PLAYER_HEALTH: usize = 3;
const PLAYER_HEALTHBAR_WIDTH: f32 = 400.0;
const PLAYER_HEALTHBAR_HEIGHT: f32 = 50.0;
const PLAYER_HEALTHBAR_PADDING: f32 = 10.0;
const BULLET_SPEED: f32 = 10.0;
const BULLET_RADIUS: f32 = 2.0;
const ASTEROID_BLINK_DURATION: f32 = 0.15;
const PLAYER_INVINCIBILITY_DURATION: f32 = 1.0;

actions! {
    UP, DOWN, LEFT, RIGHT, SHOOT
}

struct State {
    player: Player,
    asteroids: Vec<Asteroid>,
    score: usize,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(),
            asteroids: (0..NUM_ASTEROIDS).map(|_| Asteroid::new()).collect(),
            score: 0,
        }
    }

    fn update(&mut self) {
        if self.player.health != 0 {
            self.game_update();
        }
    }

    fn draw_endscreen(&self) {
        let text = format!("GAME OVER!\n\nScore: {}", self.score);
        let mut params = TextDrawParams {
            font_size: 30,
            line_spacing: 1.5,
            ..Default::default()
        };
        let dimensions = measure_multiline_text_ex(&text, params.font_size, 1.5);
        params.position = window_size() / 2.0 - dimensions.size / 2.0;
        draw_text_custom(text, params);
    }

    fn game_update(&mut self) {
        self.player.update();

        for asteroid in self.asteroids.iter_mut() {
            asteroid.update();
        }

        let mut bullets_to_remove = vec![];
        for (bi, bullet) in self.player.bullets.iter().enumerate() {
            for asteroid in self.asteroids.iter_mut() {
                let dist = bullet.pos.distance(asteroid.pos);
                if dist <= asteroid.radius + BULLET_RADIUS && asteroid.health != 0 {
                    let impact_dir = (bullet.pos - asteroid.pos).normalize_or_zero();
                    spawn_hit_particles(bullet.pos, impact_dir);
                    asteroid.health = asteroid.health.saturating_sub(1);
                    asteroid.last_hit = time();
                    if asteroid.health == 0 {
                        spawn_death_particles(asteroid.pos, asteroid.radius);
                    }
                    bullets_to_remove.push(bi);
                    break;
                }
            }
        }
        bullets_to_remove.sort_unstable();
        bullets_to_remove.dedup();
        for bi in bullets_to_remove.into_iter().rev() {
            self.player.bullets.remove(bi);
        }

        for asteroid in &self.asteroids {
            if asteroid.health == 0 && asteroid.last_hit < time() - ASTEROID_BLINK_DURATION {
                self.score += asteroid.max_health;
            }
        }

        self.asteroids
            .retain(|a| a.health > 0 || a.last_hit > time() - ASTEROID_BLINK_DURATION);

        let now = time();
        if now - self.player.last_hit > PLAYER_INVINCIBILITY_DURATION {
            for asteroid in &self.asteroids {
                let dist = self.player.pos.distance(asteroid.pos);
                if dist <= asteroid.radius + PLAYER_SIZE {
                    self.player.health = self.player.health.saturating_sub(1);
                    self.player.last_hit = now;

                    if self.player.health == 0 {
                        play_sound(get_sounds().game_over);
                    } else {
                        play_sound(get_sounds().player_hit);
                    }
                    break;
                }
            }
        }

        if rand_ratio(1, 100) {
            self.asteroids.push(Asteroid::new());
        }
    }

    fn draw(&self) {
        if self.player.health != 0 {
            draw_text(format!("Score: {}", self.score), Vec2::splat(10.0));
            self.player.draw();

            for asteroid in &self.asteroids {
                asteroid.draw();
            }
        } else {
            self.draw_endscreen();
        }
    }
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
}

impl Bullet {
    fn new(pos: Vec2, rotation: f32, player_vel: Vec2) -> Self {
        let dir = Vec2::new(rotation.sin(), rotation.cos());
        Self {
            pos,
            vel: dir * BULLET_SPEED + player_vel,
        }
    }

    fn update(&mut self) -> bool {
        self.pos += self.vel;

        let half_w = screen_distance_to_world(window_width()) / 2.0;
        let half_h = screen_distance_to_world(window_height()) / 2.0;

        self.pos.x.abs() <= half_w && self.pos.y.abs() <= half_h
    }

    fn draw(&self) {
        draw_circle_world(self.pos, BULLET_RADIUS, FG);
    }
}

struct Player {
    pos: Vec2,
    rotation: f32,
    health: usize,
    last_hit: f32,
    speed: f32,
    angvel: f32,
    bullets: Vec<Bullet>,
}

impl Player {
    fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            rotation: 0.0,
            health: MAX_PLAYER_HEALTH,
            last_hit: f32::MIN,
            speed: 0.0,
            angvel: 0.0,
            bullets: vec![],
        }
    }

    fn vel(&self) -> Vec2 {
        self.speed * Vec2::new(self.rotation.sin(), self.rotation.cos())
    }

    fn update(&mut self) {
        self.update_input();
        self.update_velocities();
        self.pos = wrap_position(self.pos, PLAYER_SIZE);

        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }
    }

    fn emit_movement_particles(&self) {
        let batch = ParticleOneshot::builder()
            .shape(&Circle::new(
                Vec2::ZERO,
                Vec2::splat(LINE_THICKNESS * 1.5),
                FG,
            ))
            .end_color(RED.with_alpha(0.0))
            .speed(self.speed * 0.8)
            .direction(-self.rotation)
            .direction_randomness(0.2)
            .speed_randomness(self.speed * 0.5)
            .lifetime(0.8)
            .lifetime_randomness(0.4)
            .quantity((self.speed * 5.0).max(1.0) as usize)
            .position_randomness(Vec2::splat(PLAYER_SIZE * 0.3))
            .build();
        get_particles().spawn_oneshot(
            &batch,
            self.pos + vec2(0.0, -PLAYER_SIZE * 0.3).rotated_around_origin(-self.rotation),
        );
    }

    fn update_input(&mut self) {
        if action_held(UP) {
            self.speed += PLAYER_ACCEL;
            self.emit_movement_particles();

            if once_per_n_seconds(0.1) {
                play_sound_ex(get_sounds().engine).volume(0.1).start();
            }
        }

        if action_held(DOWN) {
            self.speed -= PLAYER_ACCEL;
        }

        if action_held(RIGHT) {
            self.angvel -= PLAYER_TURN_SPEED;
        }

        if action_held(LEFT) {
            self.angvel += PLAYER_TURN_SPEED;
        }

        if action_pressed(SHOOT) {
            play_sound(get_sounds().shoot);
            let tip = self.pos + vec2(0.0, PLAYER_SIZE).rotated_around_origin(-self.rotation);
            self.bullets
                .push(Bullet::new(tip, self.rotation, self.vel()));
        }
    }

    fn update_velocities(&mut self) {
        self.pos += self.vel();
        self.rotation += self.angvel;

        self.speed *= PLAYER_LIN_DECEL;
        self.angvel *= PLAYER_ANG_DECEL;

        if self.speed.abs() < 0.1 {
            self.speed = 0.0;
        }

        if self.angvel.abs() < 0.1 {
            self.angvel = 0.0;
        }
    }

    fn draw(&self) {
        for bullet in &self.bullets {
            bullet.draw();
        }

        let c = self.pos;
        let tip = c + vec2(0.0, PLAYER_SIZE);
        let elbow = c + vec2(0.0, -PLAYER_SIZE * 0.5);
        let left_wing = c + vec2(-PLAYER_SIZE, -PLAYER_SIZE);
        let right_wing = c + vec2(PLAYER_SIZE, -PLAYER_SIZE);

        let now = time();
        let time = (now - self.last_hit) * 10.0;
        let blink_on = time as i32 % 2 == 0;
        let currently_invincible = now - self.last_hit < PLAYER_INVINCIBILITY_DURATION;
        let color = if currently_invincible && blink_on {
            FG.with_alpha(0.3)
        } else {
            FG
        };

        if currently_invincible && blink_on {
            vignette_screen(RED, 0.5);
        }

        draw_circle_path_world(
            &[tip, left_wing, elbow, right_wing, tip]
                .map(|p| p.rotated_around_point(c, -self.rotation)),
            LINE_THICKNESS,
            color,
        );

        self.draw_healthbar();
    }

    fn draw_healthbar(&self) {
        let width = PLAYER_HEALTHBAR_WIDTH / MAX_PLAYER_HEALTH as f32;
        let height = PLAYER_HEALTHBAR_HEIGHT;

        for i in 0..self.health {
            let offset = (width + PLAYER_HEALTHBAR_PADDING) * i as f32 + PLAYER_HEALTHBAR_PADDING;
            draw_rect(
                vec2(offset, window_height() - height - PLAYER_HEALTHBAR_PADDING),
                vec2(width, height),
                FG,
            );
        }
    }
}

struct Asteroid {
    health: usize,
    max_health: usize,
    last_hit: f32,
    points: Vec<Vec2>,
    pos: Vec2,
    velocity: Vec2,
    rotation: f32,
    angvel: f32,
    radius: f32,
}

impl Asteroid {
    fn new() -> Self {
        let radius = rand_range(MIN_ASTEROID_RADIUS..MAX_ASTEROID_RADIUS);
        let health = (radius / MIN_ASTEROID_RADIUS) as usize * 2;
        let last_hit = f32::MIN;
        let pos = rand_vec2() * BASE_WINDOW_SIZE / 2.0;
        let velocity = rand_vec2() * MAX_ASTEROID_SPEED;
        let rotation = rand_f32() * TAU;
        let angvel = rand_f32() * MAX_ASTEROID_ANGVEL;

        let num_points = (radius * MAX_ASTEROID_POINTS) as usize;
        let max_offset = radius * MAX_ASTEROID_POINT_OFFSET / 2.0;

        let mut points = Vec::with_capacity(num_points + 1);

        for i in 0..num_points {
            let angle = (TAU / num_points as f32) * i as f32;
            let offset = rand_range(-max_offset..max_offset) + radius;
            let pos = Vec2::new(0.0, offset).rotated_around_origin(angle);
            points.push(pos);
        }

        points.push(points[0]);

        Self {
            health,
            last_hit,
            points,
            pos,
            velocity,
            rotation,
            angvel,
            radius,
            max_health: health,
        }
    }

    fn update(&mut self) {
        self.pos += self.velocity;
        self.rotation += self.angvel;
        self.pos = wrap_position(self.pos, self.radius);
    }

    fn draw(&self) {
        let points = &self
            .points
            .iter()
            .map(|p| p.rotated_around_origin(self.rotation) + self.pos)
            .collect::<Vec<_>>();

        let blinking = time() - self.last_hit < ASTEROID_BLINK_DURATION;
        let fill = if blinking { FG } else { BG };

        draw_custom_shape_world(points, fill);
        draw_circle_path_world(points, LINE_THICKNESS, FG);
    }
}

struct Sounds {
    shoot: SoundRef,
    asteroid_hit: SoundRef,
    asteroid_death: SoundRef,
    player_hit: SoundRef,
    game_over: SoundRef,
    engine: SoundRef,
}

impl Sounds {
    fn new() -> Result<Self, SoundLoadError> {
        Ok(Self {
            shoot: include_sound!("../assets/sounds/Gun.wav")?,
            asteroid_hit: include_sound!("../assets/sounds/Click.wav")?,
            asteroid_death: include_sound!("../assets/sounds/Explosion.wav")?,
            player_hit: include_sound!("../assets/sounds/Hurt.wav")?,
            game_over: include_sound!("../assets/sounds/Powerdown.wav")?,
            engine: include_sound!("../assets/sounds/Crunch.wav")?,
        })
    }
}

#[main("Space game")]
fn main() -> anyhow::Result<()> {
    storage_store_state(ParticleSystem::new());
    storage_store_state(Sounds::new()?);

    bind! {
        UP => KeyCode::ArrowUp;
        UP => KeyCode::KeyW;
        DOWN => KeyCode::ArrowDown;
        DOWN => KeyCode::KeyS;
        LEFT => KeyCode::ArrowLeft;
        LEFT => KeyCode::KeyA;
        RIGHT => KeyCode::ArrowRight;
        RIGHT => KeyCode::KeyD;
        SHOOT => KeyCode::Space;
    }

    let mut state = State::new();

    loop {
        clear_screen(BG);
        let scale = window_size() / BASE_WINDOW_SIZE * BASE_SCALE;
        let scale = scale.x.min(scale.y);
        get_camera_2d_mut().set_scale(scale);

        state.update();

        get_particles().update();
        get_particles().draw_world();
        state.draw();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn get_particles() -> &'static mut ParticleSystem {
    storage_get_state_mut()
}

fn get_sounds() -> &'static Sounds {
    storage_get_state()
}

fn spawn_hit_particles(pos: Vec2, impact_dir: Vec2) {
    play_sound(get_sounds().asteroid_hit);
    let batch = ParticleOneshot::builder()
        .shape(&Circle::new(
            Vec2::ZERO,
            Vec2::splat(LINE_THICKNESS * 1.5),
            FG.with_alpha(0.5),
        ))
        .end_color(FG.with_alpha(0.0))
        .speed(3.0)
        .direction(impact_dir.to_angle() + FRAC_PI_2)
        .direction_randomness(1.5)
        .speed_randomness(2.0)
        .lifetime(0.4)
        .lifetime_randomness(0.2)
        .quantity(8)
        .build();
    get_particles().spawn_oneshot(&batch, pos);
}

fn spawn_death_particles(pos: Vec2, radius: f32) {
    play_sound(get_sounds().asteroid_death);
    let batch = ParticleOneshot::builder()
        .shape(&Circle::new(
            Vec2::ZERO,
            Vec2::splat(LINE_THICKNESS * 2.0),
            FG.with_alpha(0.7),
        ))
        .end_color(FG.with_alpha(0.0))
        .speed(radius * 0.1)
        .direction(0.0)
        .direction_randomness(std::f32::consts::TAU)
        .speed_randomness(radius * 0.2)
        .lifetime(0.8)
        .lifetime_randomness(0.4)
        .quantity((radius * 1.5) as usize)
        .position_randomness(Vec2::splat(radius * 0.5))
        .build();
    get_particles().spawn_oneshot(&batch, pos);
}

fn wrap_position(pos: Vec2, size: f32) -> Vec2 {
    let half_w = screen_distance_to_world(window_width()) / 2.0 + size;
    let half_h = screen_distance_to_world(window_height()) / 2.0 + size;

    let x = if pos.x > half_w {
        -half_w
    } else if pos.x < -half_w {
        half_w
    } else {
        pos.x
    };

    let y = if pos.y > half_h {
        -half_h
    } else if pos.y < -half_h {
        half_h
    } else {
        pos.y
    };

    Vec2::new(x, y)
}
