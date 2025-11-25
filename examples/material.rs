use engine_4::prelude::*;

pub fn main() -> anyhow::Result<()> {
    init("Custom materials")?;

    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);
    let mut current_material = 0;
    let mut materials = vec![];
    let light_pos = Vec3::new(0.0, 5.0, 2.0);

    {
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
            .with_vec3("light_pos", light_pos)
            .with_color("light_color", Color::WHITE.with_alpha(0.4))
            .create();
        materials.push(material);
    }

    // Holographic material
    {
        let program = include_program!(
            "./material_shader/holographic_vertex.glsl",
            "./material_shader/holographic_fragment.glsl"
        )?;
        let material = Material::new(program)
            .with_float("time", 0.0) // will be updated each frame
            .with_vec3("camera_pos", Vec3::ZERO) // will be updated each frame
            .create();
        materials.push(material);
    }

    // Dissolve material
    {
        let program = include_program!(
            "./material_shader/dissolve_vertex.glsl",
            "./material_shader/dissolve_fragment.glsl"
        )?;
        let material = Material::new(program)
            .with_float("time", 0.0) // will be updated each frame
            .with_float("dissolve_threshold", 0.5) // adjust as needed
            .with_color("edge_color", Color::RED_500)
            .with_float("edge_width", 0.05)
            .create();
        materials.push(material);
    }

    // Outline material
    {
        let program = include_program!(
            "./material_shader/outline_vertex.glsl",
            "./material_shader/outline_fragment.glsl"
        )?;
        let material = Material::new(program)
            .with_color("outline_color", Color::PURPLE_100)
            .with_float("outline_width", 0.3)
            .create();
        materials.push(material);
    }

    let mut object = Object3D::from_obj_bytes_with_material(
        include_bytes!("../assets/models/suzanne_highres.obj"),
        materials[0],
    )?;
    object.compute_smooth_normals();

    loop {
        clear_screen(Color::hex(0x000001));
        let input = get_input();
        let time = time();
        orbit_controller.update(input);

        if let Some(mat) = materials.get_mut(3) {
            let threshold = (time * 0.2).sin() * 0.5 + 0.5;
            mat.set_float("dissolve_threshold", threshold);
        }

        if input.key_pressed(KeyCode::KeyN) {
            current_material += 1;
            current_material %= materials.len();
            object.material = materials[current_material];
        }

        object.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
