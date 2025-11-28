use bevy_math::VectorSpace;
use engine_4::prelude::*;

struct MovingCircle {
    circle: Circle,
    velocity: Vec2,
}

impl MovingCircle {
    pub fn random() -> Self {
        Self {
            circle: Circle {
                center: Vec2::new(rand::<f32>() * 1000.0, rand::<f32>() * 1000.0),
                radius: Vec2::splat(rand::<f32>() * 30.0),
                color: Color::from_rgba(rand(), rand(), rand(), 1.0),
            },
            velocity: Vec2::ZERO,
        }
    }
}

fn main() -> anyhow::Result<()> {
    init("3D render textures")?;

    let mut camera_controller = OrbitCameraController::new(Vec3::ZERO);

    let render_texture = create_empty_render_texture(1000, 1000)?;
    let textured_material = create_textured_material(render_texture.color_texture);
    let textured_cube = Object3D::from_obj_bytes_with_material(
        include_bytes!("../assets/models/cube.obj"),
        textured_material,
    )?;

    let mut circles: Vec<_> = (0..500).map(|_| MovingCircle::random()).collect();

    loop {
        clear_screen(Color::WHITE);

        camera_controller.update(get_input());

        start_rendering_to_texture(render_texture);
        clear_screen(Color::TEAL_500);
        for circle in circles.iter_mut() {
            let r = || (rand::<f32>() - 0.5) * 1.0;
            circle.velocity += Vec2::new(r(), r());
            circle.circle.center += circle.velocity;
            circle.circle.center %= 1000.0;
            draw_shape(&circle.circle);
        }
        end_rendering_to_texture();

        textured_cube.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
