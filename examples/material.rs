use engine_4::prelude::*;

pub fn main() -> anyhow::Result<()> {
    init("Custom materials")?;

    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);

    let program = include_program!(
        "./material_shader/vertex.glsl",
        "./material_shader/fragment.glsl"
    )?;
    let texture = load_texture(
        include_bytes!("../assets/textures/space.jpg"),
        ImageFormat::Jpeg,
    )?;
    let material = Material::new(program)
        .with_texture("texture", texture)
        .with_vec3("light_pos", Vec3::Y * 5.0)
        .with_color("light_color", Color::WHITE.with_alpha(0.4))
        .create();
    let object = Object3D::from_obj_bytes_with_material(
        include_bytes!("../assets/models/suzanne.obj"),
        material,
    )?;

    loop {
        clear_screen(Color::BLACK);
        let input = get_input();
        orbit_controller.update(input);

        object.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
