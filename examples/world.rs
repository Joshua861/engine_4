// improved version of the physics.rs example, that uses the world system to simplify/organise code and improve performance

use sge::prelude::*;

const BOUNDS_SIZE: Vec2 = Vec2::new(1000.0, 1000.0);
const BOUNDS_THICKNESS: f32 = 50.0;
const FORCE_RADIUS: f32 = 250.0;
const FORCE_STRENGTH: f32 = 100.0;
const GRAVITY_STRENGTH: f32 = 1000.0;

#[derive(Clone, Copy, PartialEq)]
enum ShapeType {
    Circle,
    Square,
}

impl ShapeType {
    fn from_index(i: usize) -> Self {
        if i % 2 == 0 {
            Self::Circle
        } else {
            Self::Square
        }
    }
    fn bounds(&self) -> Bounds {
        match self {
            Self::Circle => Bounds::Circle(15.0),
            Self::Square => Bounds::Rect(Vec2::splat(30.0)),
        }
    }
    fn draw(&self, pos: Vec2, color: Color, rotation: f32) {
        match self {
            Self::Circle => {
                let sdf = Sdf::circle(pos, 15.0).with_fill(
                    color.darken_oklch(0.1),
                    color,
                    -rotation,
                    2.0,
                    SdfFill::Checker,
                );
                draw_sdf_world(sdf);
            }
            Self::Square => {
                let sdf = Sdf::square(pos, 30.0).with_rotation(rotation).with_fill(
                    color.darken_oklch(0.1),
                    color,
                    0.0,
                    2.0,
                    SdfFill::Checker,
                );
                draw_sdf_world(sdf);
            }
        }
    }
}

fn speed_color(speed: f32) -> Color {
    Color::from_oklch(
        0.8,
        0.1 + (speed / 100.0).clamp(0.0, 0.1),
        142.94 - (speed / 5.0).clamp(0.0, 116.77),
    )
}

struct ShapeEntity {
    shape_type: ShapeType,
    pos: Vec2,
    rot: f32,
    speed: f32,
}

impl Entity2D for ShapeEntity {
    fn update(&mut self, state: &mut WorldState2D) {
        let Some(mut rb) = state.rigidbody() else {
            return;
        };

        if mouse_held(MouseButton::Right) {
            if let Some(cursor_pos) = cursor_world() {
                let to_cursor = cursor_pos - rb.get_position();
                let dist = to_cursor.length();
                if dist < FORCE_RADIUS && dist > 0.0 {
                    let strength = (1.0 - dist.powi(2) / FORCE_RADIUS.powi(2)) * FORCE_STRENGTH;
                    rb.add_velocity(to_cursor.normalize() * strength);
                }
            }
        }
        self.pos = rb.get_position();
        self.rot = rb.get_rotation();
        self.speed = rb.get_velocity().length();
    }
    fn draw(&self) {
        self.shape_type
            .draw(self.pos, speed_color(self.speed), self.rot);
    }
    fn position(&self, _: Option<&PhysicsObjectRef>) -> Vec2 {
        self.pos
    }
    fn radius(&self) -> f32 {
        20.0
    }

    // increases performance dramatically by not alternating between drawing sdf shapes and vertex based shapes
    // reduces expected draw call count from n/2 to a constant 2, where n is the number of shapes being drawn
    fn z_index(&self) -> i32 {
        match self.shape_type {
            ShapeType::Circle => 1,
            ShapeType::Square => 0,
        }
    }
}

struct Wall {
    pos: Vec2,
    size: Vec2,
}
impl Entity2D for Wall {
    fn update(&mut self, _: &mut WorldState2D) {}

    fn draw(&self) {
        draw_rect_world(self.pos - self.size * 0.5, self.size, Color::NEUTRAL_800);
    }
    fn position(&self, _: Option<&PhysicsObjectRef>) -> Vec2 {
        self.pos
    }
    fn radius(&self) -> f32 {
        (self.size.x.max(self.size.y)) / 2.0
    }
}

struct Ramp {
    pos: Vec2,
    pts: [Vec2; 3],
}

impl Entity2D for Ramp {
    fn update(&mut self, _: &mut WorldState2D) {}
    fn draw(&self) {
        draw_tri_world(
            self.pos + self.pts[0],
            self.pos + self.pts[1],
            self.pos + self.pts[2],
            Color::NEUTRAL_700,
        );
    }
    fn position(&self, _: Option<&PhysicsObjectRef>) -> Vec2 {
        self.pos
    }
    fn radius(&self) -> f32 {
        200.0
    }
}

struct Sensor {
    pos: Vec2,
    active: bool,
}

impl Entity2D for Sensor {
    fn update(&mut self, state: &mut WorldState2D) {
        if let Some(rb) = state.rigidbody() {
            self.active = rb.is_colliding();
        }
    }
    fn draw(&self) {
        let (fill, outline) = if self.active {
            (Color::CYAN_500.with_alpha(0.25), Color::CYAN_400)
        } else {
            (Color::CYAN_900.with_alpha(0.15), Color::CYAN_700)
        };
        draw_circle_with_outline_world(self.pos, 80.0, fill, outline, 2.5);
        if self.active {
            draw_circle_outline_world(self.pos, 90.0, Color::CYAN_300.with_alpha(0.4), 1.0);
        }
    }
    fn position(&self, _: Option<&PhysicsObjectRef>) -> Vec2 {
        self.pos
    }
    fn radius(&self) -> f32 {
        80.0
    }
}

#[main("Physics Showcase")]
fn main() {
    let mut world = World2D::new(100.0);
    world.physics.set_gravity(GRAVITY_STRENGTH);
    let mut controller = PanningCameraController::new();
    controller.set_pan_button(Button(MouseButton::Middle));

    get_camera_2d_mut().translate_by(BOUNDS_SIZE / 2.0);

    let min = Vec2::ZERO;
    let max = BOUNDS_SIZE;
    let t = BOUNDS_THICKNESS;
    let t2 = t * 0.5;

    let walls = [
        (
            Vec2::new((min.x + max.x) * 0.5, min.y + t2),
            Vec2::new(max.x - min.x, t),
        ),
        (
            Vec2::new((min.x + max.x) * 0.5, max.y - t2),
            Vec2::new(max.x - min.x, t),
        ),
        (
            Vec2::new(min.x + t2, (min.y + max.y) * 0.5),
            Vec2::new(t, max.y - min.y),
        ),
        (
            Vec2::new(max.x - t2, (min.y + max.y) * 0.5),
            Vec2::new(t, max.y - min.y),
        ),
    ];
    for (pos, size) in walls {
        world
            .spawn_fixed(Wall { pos, size }, Bounds::Rect(size))
            .set_position(pos);
    }

    let ramp_pos = Vec2::new(max.x * 0.5, max.y * 0.5 + 200.0);
    let pts = [
        Vec2::new(-120.0, 40.0),
        Vec2::new(120.0, 40.0),
        Vec2::new(-120.0, -40.0),
    ];
    world
        .spawn_fixed(
            Ramp { pos: ramp_pos, pts },
            Bounds::Triangle(pts[0], pts[1], pts[2]),
        )
        .set_position(ramp_pos);

    let sensor_pos = Vec2::new(max.x * 0.75, max.y * 0.25);
    world
        .spawn_fixed_with_config(
            Sensor {
                pos: sensor_pos,
                active: false,
            },
            Bounds::Circle(80.0),
            ColliderConfig::default().sensor(true),
        )
        .set_position(sensor_pos);

    let mut shape_counter = 0;
    for _ in 0..5 {
        let pos = Vec2::new(rand::<f32>() * (max.x - t * 4.0) + t * 2.0, t);
        let velocity = Vec2::new(rand_f32() * 50.0, rand_f32() * 50.0);
        let shape_type = ShapeType::from_index(shape_counter);
        shape_counter += 1;

        let mut rb = world.spawn_dynamic(
            ShapeEntity {
                shape_type,
                pos,
                rot: 0.0,
                speed: 0.0,
            },
            shape_type.bounds(),
        );
        rb.set_position(pos);
        rb.set_velocity(velocity);
    }

    set_cursor_visible(false);

    loop {
        clear_screen(Color::NEUTRAL_900);

        world.update();
        controller.update();
        draw_simple_debug_info();

        if let Some(cursor_pos) = cursor_world() {
            draw_circle_with_outline_world(cursor_pos, 10.0, Color::CYAN_400, Color::WHITE, 3.0);

            if mouse_held(MouseButton::Right) {
                draw_sdf_world(
                    Sdf::circle(cursor_pos, FORCE_RADIUS)
                        .with_fill(
                            Color::CYAN_500.with_alpha(0.2),
                            Color::CYAN_500.with_alpha(0.15),
                            0.0,
                            1.0,
                            SdfFill::RadialGradient,
                        )
                        .with_stroke(3.0, Color::WHITE, SdfStroke::Inside),
                );
            }

            if mouse_pressed(MouseButton::Left) {
                let shape_type = ShapeType::from_index(shape_counter);
                shape_counter += 1;

                let mut rb = world.spawn_dynamic(
                    ShapeEntity {
                        shape_type,
                        pos: cursor_pos,
                        rot: 0.0,
                        speed: 0.0,
                    },
                    shape_type.bounds(),
                );
                rb.set_position(cursor_pos);
                rb.set_velocity(Vec2::new(
                    rand::<f32>() * 200.0 - 100.0,
                    rand::<f32>() * 200.0 - 100.0,
                ));
            }
        }

        if key_held(KeyCode::KeyD) {
            world.debug_entities();
        }

        if should_quit() {
            break;
        }
        next_frame().await;
    }
}
