use sge::prelude::*;

const BOUNDS_SIZE: Vec2 = Vec2::new(1000.0, 1000.0);
const BOUNDS_THICKNESS: f32 = 50.0;
const FORCE_RADIUS: f32 = 250.0;
const FORCE_STRENGTH: f32 = 100.0;

#[derive(Clone, Copy, PartialEq)]
enum ShapeType {
    Circle,
    Square,
}

impl ShapeType {
    fn from_index(i: usize) -> Self {
        match i % 2 {
            0 => Self::Circle,
            _ => Self::Square,
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
            Self::Circle => draw_circle(pos, 15.0, color),
            Self::Square => draw_square_rotation(pos - Vec2::splat(15.0), 30.0, color, rotation),
        }
    }
}

fn speed_color(speed: f32) -> Color {
    Color::from_oklch(
        0.8,
        0.1 + (speed / 100.0).clamp(0.0, 0.1),
        142.94 - (speed / 5.0).clamp(0.0, 142.94 - 26.17),
    )
}

#[main("Physics Showcase")]
fn main() {
    let mut world = PhysicsWorld::new();

    let wall_rects = [
        (
            Vec2::new(BOUNDS_THICKNESS * 0.5, BOUNDS_SIZE.y * 0.5),
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
        ),
        (
            Vec2::new(BOUNDS_SIZE.x * 0.5, BOUNDS_THICKNESS * 0.5),
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
        ),
        (
            Vec2::new(BOUNDS_SIZE.x * 0.5, BOUNDS_SIZE.y - BOUNDS_THICKNESS * 0.5),
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
        ),
        (
            Vec2::new(BOUNDS_SIZE.x - BOUNDS_THICKNESS * 0.5, BOUNDS_SIZE.y * 0.5),
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
        ),
    ];
    for (pos, size) in wall_rects {
        world.create_fixed(Bounds::Rect(size)).with_position(pos);
    }

    let ramp_pos = Vec2::new(BOUNDS_SIZE.x * 0.5, BOUNDS_SIZE.y * 0.5 + 200.0);
    world
        .create_fixed(Bounds::Triangle(
            Vec2::new(-120.0, 40.0),
            Vec2::new(120.0, 40.0),
            Vec2::new(-120.0, -40.0),
        ))
        .with_position(ramp_pos);

    let sensor_pos = Vec2::new(BOUNDS_SIZE.x * 0.75, BOUNDS_SIZE.y * 0.25);
    let sensor = world
        .create_fixed_with(Bounds::Circle(80.0), ColliderConfig::default().sensor(true))
        .with_position(sensor_pos);

    let mut objects: Vec<(PhysicsObjectRef, ShapeType)> = Vec::new();

    for i in 0..50 {
        let pos = Vec2::new(
            rand::<f32>() * (BOUNDS_SIZE.x - BOUNDS_THICKNESS * 2.0) + BOUNDS_THICKNESS,
            rand::<f32>() * (BOUNDS_SIZE.y * 0.6) + BOUNDS_THICKNESS,
        );
        let velocity = Vec2::new(rand::<f32>() * 500.0 - 250.0, rand::<f32>() * 500.0 - 250.0);
        let shape_type = ShapeType::from_index(i);

        let collider = world
            .create_dynamic(shape_type.bounds())
            .with_ccd()
            .with_position(pos);

        let mut collider = collider;
        collider.set_velocity(velocity);
        objects.push((collider, shape_type));
    }

    let mut show_colliders = false;

    set_cursor_visible(false);

    loop {
        world.update();
        clear_screen(Color::NEUTRAL_900);

        draw_rect(
            Vec2::ZERO,
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
            Color::NEUTRAL_800,
        );
        draw_rect(
            Vec2::ZERO,
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
            Color::NEUTRAL_800,
        );
        draw_rect(
            Vec2::new(0.0, BOUNDS_SIZE.y - BOUNDS_THICKNESS),
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
            Color::NEUTRAL_800,
        );
        draw_rect(
            Vec2::new(BOUNDS_SIZE.x - BOUNDS_THICKNESS, 0.0),
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
            Color::NEUTRAL_800,
        );

        {
            use ui::prelude::*;
            let ui = Fit::new(Fill::new(
                Color::NEUTRAL_800,
                Padding::all(
                    50.0,
                    Col::new([
                        Text::title_nowrap("Physics Showcase"),
                        Text::mono(format!("Objects: {}", objects.len())),
                        Text::mono(format!("FPS: {:.2}", avg_fps())),
                        Text::h2("Controls"),
                        Text::body("• Left Click: Spawn object"),
                        Text::body("• Right Click (hold): Apply force"),
                        Text::body("• D: Toggle collider debug"),
                    ]),
                ),
            ));
            ui::draw_ui(ui, vec2(0.0, BOUNDS_SIZE.y - BOUNDS_THICKNESS));
        }

        draw_tri(
            ramp_pos + Vec2::new(-120.0, 40.0),
            ramp_pos + Vec2::new(120.0, 40.0),
            ramp_pos + Vec2::new(-120.0, -40.0),
            Color::NEUTRAL_700,
        );

        let sensor_active = sensor.is_colliding();
        let sensor_fill = if sensor_active {
            Color::CYAN_500.with_alpha(0.25)
        } else {
            Color::CYAN_900.with_alpha(0.15)
        };
        let sensor_outline = if sensor_active {
            Color::CYAN_400
        } else {
            Color::CYAN_700
        };

        draw_circle_with_outline(sensor_pos, 80.0, sensor_fill, sensor_outline, 2.5);
        if sensor_active {
            draw_circle_outline(sensor_pos, 90.0, Color::CYAN_300.with_alpha(0.4), 1.0);
        }

        for (collider, shape_type) in &objects {
            let pos = collider.get_position();
            let speed = collider.get_velocity().length();
            let rot = collider.get_rotation();
            shape_type.draw(pos, speed_color(speed), rot);
        }

        if show_colliders {
            world.draw_colliders();
        }

        if let Some(cursor_pos) = cursor() {
            draw_circle_with_outline(cursor_pos, 10.0, Color::CYAN_400, Color::WHITE, 3.0);

            if mouse_held(MouseButton::Right) {
                for (collider, _) in &mut objects {
                    let pos = collider.get_position();
                    let to_cursor = cursor_pos - pos;
                    let distance = to_cursor.length();
                    if distance < FORCE_RADIUS && distance > 0.0 {
                        let strength =
                            (1.0 - distance.powi(2) / FORCE_RADIUS.powi(2)) * FORCE_STRENGTH;
                        collider.add_velocity(to_cursor.normalize() * strength);
                    }
                }

                draw_sdf(
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
                let velocity =
                    Vec2::new(rand::<f32>() * 200.0 - 100.0, rand::<f32>() * 200.0 - 100.0);
                let shape_type = ShapeType::from_index(objects.len());
                let mut collider = world.create_dynamic(shape_type.bounds()).with_ccd();
                collider.set_position(cursor_pos);
                collider.set_velocity(velocity);
                objects.push((collider, shape_type));
            }
        }

        if key_pressed(KeyCode::KeyD) {
            show_colliders = !show_colliders;
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
