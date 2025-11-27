use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("3D render textures")?;

    let mut camera_controller = OrbitCameraController::new(Vec3::ZERO);

    let render_texture = create_empty_render_texture(1000, 1000)?;
    let textured_material = create_textured_material(render_texture.color_texture);
    let textured_cube = Object3D::from_obj_bytes_with_material(
        include_bytes!("../assets/models/cube.obj"),
        textured_material,
    )?;

    mutate_camera_3d(|c| c.isometric = true);

    let flat_material = create_gouraud_material(Color::SKY_500, Color::SKY_300, Vec3::Y);
    let mut flat_cube = Object3D::from_mesh_and_material(textured_cube.mesh, flat_material);

    loop {
        clear_screen(Color::WHITE);

        camera_controller.update(get_input());

        flat_cube
            .transform
            .rotate_by(Quat::from_xyzw(0.005, 0.026, 0.009, 1.000));

        start_rendering_to_texture(render_texture);
        clear_screen(Color::TEAL_500);
        flat_cube.draw();
        end_rendering_to_texture();

        textured_cube.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
