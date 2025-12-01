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
        clear_screen(Color::WHITE);
        orbit_controller.update();

        obj.draw();

        brighten_screen(time().sin().clamp(-0.5, 0.5));
        vignette_screen(Color::BLACK, 0.2);

        draw_circle(window_size() - Vec2::splat(100.0), 50.0, Color::RED_300);

        pixelate_screen(10.0);

        draw_circle(
            Vec2::new(window_size().x - 100.0, 100.0),
            50.0,
            Color::SKY_500,
        );

        run_ui(|_| {});

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
