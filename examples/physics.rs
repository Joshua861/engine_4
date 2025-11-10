use engine_4::prelude::*;

const BOUNDS_SIZE: Vec2 = Vec2::new(1000.0, 1000.0);
const BOUNDS_THICKNESS: f32 = 50.0;
const FORCE_RADIUS: f32 = 250.0;
const FORCE_STRENGTH: f32 = 50.0;

#[derive(Clone, Copy)]
enum ShapeType {
    Circle,
    Square,
    Rectangle,
}

impl ShapeType {
    fn from_index(i: usize) -> Self {
        match i % 3 {
            0 => Self::Circle,
            1 => Self::Square,
            _ => Self::Rectangle,
        }
    }

    fn create_collider(&self) -> ColliderBuilder {
        match self {
            Self::Circle => ColliderBuilder::ball(15.0).restitution(0.8).density(1.0),
            Self::Square => ColliderBuilder::cuboid(15.0, 15.0)
                .restitution(0.8)
                .density(2.0),
            Self::Rectangle => ColliderBuilder::cuboid(20.0, 10.0)
                .restitution(0.8)
                .density(1.5),
        }
    }

    fn draw(&self, pos: Vec2, color: Color) {
        match self {
            Self::Circle => draw_circle(pos, 15.0, color),
            Self::Square => draw_square(pos - Vec2::splat(15.0), 30.0, color),
            Self::Rectangle => draw_rect(pos - Vec2::new(20.0, 10.0), Vec2::new(40.0, 20.0), color),
        }
    }
}

fn speed_color(speed: f32) -> Color {
    if speed > 200.0 {
        Color::RED_500
    } else if speed > 100.0 {
        Color::ORANGE_500
    } else if speed > 50.0 {
        Color::YELLOW_500
    } else {
        Color::EMERALD_500
    }
}

fn create_boundary_walls(world: &mut PhysicsWorld) -> [[Vec2; 2]; 4] {
    let walls = [
        [Vec2::ZERO, Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y)],
        [Vec2::ZERO, Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS)],
        [
            Vec2::new(0.0, BOUNDS_SIZE.y - BOUNDS_THICKNESS),
            BOUNDS_SIZE,
        ],
        [
            Vec2::new(BOUNDS_SIZE.x - BOUNDS_THICKNESS, 0.0),
            BOUNDS_SIZE,
        ],
    ];

    for [a, b] in &walls {
        let center = a.midpoint(*b);
        let size = *b - *a;

        let body =
            world.insert_rigid_body(RigidBodyBuilder::fixed().translation(center.into()).build());

        world.collider_set.insert_with_parent(
            ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
                .restitution(1.0)
                .build(),
            body,
            &mut world.rigid_body_set,
        );
    }

    walls
}

fn spawn_object(
    world: &mut PhysicsWorld,
    pos: Vec2,
    velocity: nalgebra::Vector2<f32>,
    shape_type: ShapeType,
) -> (physics::RigidBodyHandle, physics::ColliderHandle) {
    let body = world.insert_rigid_body(
        RigidBodyBuilder::dynamic()
            .translation(pos.into())
            .linvel(velocity)
            .build(),
    );

    let collider = world.collider_set.insert_with_parent(
        shape_type.create_collider().build(),
        body,
        &mut world.rigid_body_set,
    );

    (body, collider)
}

fn main() -> anyhow::Result<()> {
    init("Physics Showcase")?;

    let mut cursor_pos = Vec2::ZERO;
    let mut world = PhysicsWorld::new().with_custom_gravity(Vec2::new(0.0, 1000.0));
    let mut objects: Vec<(physics::RigidBodyHandle, physics::ColliderHandle, ShapeType)> =
        Vec::new();

    let walls = create_boundary_walls(&mut world);

    for i in 0..500 {
        let pos = Vec2::new(
            rand::<f32>() * (BOUNDS_SIZE.x - BOUNDS_THICKNESS * 2.0) + BOUNDS_THICKNESS,
            rand::<f32>() * (BOUNDS_SIZE.y - BOUNDS_THICKNESS * 2.0) + BOUNDS_THICKNESS,
        );
        let velocity = vector![rand::<f32>() * 500.0 - 250.0, rand::<f32>() * 500.0 - 250.0];
        let shape_type = ShapeType::from_index(i);

        let (body, collider) = spawn_object(&mut world, pos, velocity, shape_type);
        objects.push((body, collider, shape_type));
    }

    loop {
        clear_screen(Color::NEUTRAL_900);
        let input = get_input();

        if let Some((x, y)) = input.cursor() {
            cursor_pos = Vec2::new(x, y);
        }

        if input.key_pressed(KeyCode::KeyD) {
            show_debug_info();
        }

        if input.mouse_pressed(MouseButton::Left) {
            let velocity = vector![rand::<f32>() * 100.0 - 50.0, rand::<f32>() * 100.0 - 50.0];
            let shape_type = ShapeType::from_index(objects.len());
            let (body, collider) = spawn_object(&mut world, cursor_pos, velocity, shape_type);
            objects.push((body, collider, shape_type));
        }

        if input.mouse_held(MouseButton::Right) {
            for (body_handle, _, _) in &objects {
                if let Some(body) = world.get_rigid_body_mut(*body_handle) {
                    let pos = body.translation();
                    let to_cursor = vector![cursor_pos.x - pos.x, cursor_pos.y - pos.y];
                    let distance = to_cursor.norm();

                    if distance < FORCE_RADIUS {
                        let strength = (1.0 - distance / FORCE_RADIUS) * FORCE_STRENGTH;
                        let mut vel = *body.linvel();
                        vel += to_cursor.normalize() * strength;
                        body.set_linvel(vel, true);
                    }
                }
            }

            draw_circle(cursor_pos, FORCE_RADIUS, Color::CYAN_700);
            draw_circle(cursor_pos, FORCE_RADIUS - 3.0, Color::CYAN_800);
            draw_circle(cursor_pos, 10.0, Color::CYAN_300);
        }

        for [a, b] in &walls {
            draw_rect(*a, *b - *a, Color::NEUTRAL_950);
        }

        for (body_handle, _, shape_type) in &objects {
            if let Some(body) = world.get_rigid_body(*body_handle) {
                let pos: Vec2 = (*body.translation()).into();
                let speed = body.linvel().norm();
                let color = speed_color(speed);
                shape_type.draw(pos, color);
            }
        }

        run_ui(|ctx| {
            egui::Window::new("Physics Showcase").show(ctx, |ui| {
                ui.label(format!("Objects: {}", objects.len()));
                ui.separator();
                ui.label("Controls:");
                ui.label("• Left Click: Spawn object");
                ui.label("• Right Click (hold): Apply force");
            });
        });

        if should_quit() {
            break;
        }

        world.step();
        next_frame();
    }

    Ok(())
}
