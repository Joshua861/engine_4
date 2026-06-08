use sge::prelude::*;

#[main("Post processing 3")]
fn main() -> anyhow::Result<()> {
    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);
    let material = create_gouraud_material(Color::SLATE_200, Color::SLATE_600, Vec3::Y);
    let obj = Object3D::from_obj_bytes_with_material(
        include_bytes!("../assets/models/suzanne.obj"),
        material,
    )?;

    loop {
        clear_screen(Color::NEUTRAL_950);
        orbit_controller.update();

        obj.draw();

        quantization_screen(oscillate(2.0, 20.0));
        wobbly_screen(time(), 5.0, 0.025, 10.0);
        fish_eye_screen(5.0);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
