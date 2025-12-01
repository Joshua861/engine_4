use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("3D?")?;

    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);

    let material = create_blinn_phong_material(
        Color::RED_900,
        Color::RED_500,
        Color::RED_200,
        Color::PURPLE_300.with_alpha(0.3),
        Vec3::new(2.0, 5.0, 0.0),
    );

    let data = include_bytes!("../assets/models/suzanne_highres.obj");
    let mut model = Object3D::from_obj_bytes_with_material(data, material)?;
    model.compute_smooth_normals();

    loop {
        clear_screen(Color::PURPLE_300);

        orbit_controller.update();

        model.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
