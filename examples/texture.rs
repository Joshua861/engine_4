use engine_4::prelude::*;

const GRID_SIZE: usize = 50;

fn main() -> anyhow::Result<()> {
    init("3D?")?;

    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);

    let texture = load_texture(
        include_bytes!("../assets/models/beachball.png"),
        ImageFormat::Png,
    )?;
    let material = create_textured_material(texture);
    let data = include_bytes!("../assets/models/beachball.obj");
    let ball = Object3D::from_obj_bytes_with_material(data, material)?;

    loop {
        clear_screen(Color::PURPLE_300);
        let input = get_input();

        orbit_controller.update(input);

        ball.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
