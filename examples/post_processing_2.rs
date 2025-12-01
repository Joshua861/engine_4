use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Post processing")?;

    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);
    let material = create_gouraud_material(Color::SLATE_100, Color::SLATE_500, Vec3::Y);
    let obj = Object3D::from_obj_bytes_with_material(
        include_bytes!("../assets/models/suzanne.obj"),
        material,
    )?;
    show_debug_info();

    loop {
        clear_screen(Color::SKY_500);
        orbit_controller.update();

        obj.draw();
        bloom_screen(0.5, 2.0, 10.0);

        run_ui(|_| {});

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
