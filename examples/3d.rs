use engine_4::prelude::*;

const GRID_SIZE: usize = 50;

fn main() -> anyhow::Result<()> {
    init("3D?")?;

    let mut clear_color = Color::PURPLE_200;

    mutate_camera_3d(|camera| {
        // camera.isometric = true;
    });

    let mut show_many = false;
    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);

    let grid_width = (GRID_SIZE - 1) as f32 * 3.0;
    let grid_center = grid_width / 2.0;

    let material =
        create_gouraud_material(Color::SLATE_300, Color::SLATE_400, Vec3::new(2.0, 5.0, 0.0));
    let data = include_bytes!("../assets/models/suzanne.obj");
    let mut suzanne = Object3D::from_obj_bytes_with_material(data, material)?;

    show_debug_info();

    let mut transforms = Vec::with_capacity(GRID_SIZE * GRID_SIZE);
    for x in 0..GRID_SIZE {
        for z in 0..GRID_SIZE {
            let vector = Vec3::new(x as f32 * 3.0, 0.0, z as f32 * 3.0);
            let transform = Transform3D::from_translation(vector);
            transforms.push(transform);
        }
    }

    loop {
        let input = get_input();

        if input.key_pressed(KeyCode::KeyM) {
            show_many = !show_many;
            orbit_controller.set_enabled(!show_many);
        }

        if input.key_pressed(KeyCode::KeyY) {
            let mat = suzanne.material();

            mat.set_color("regular_color", Color::YELLOW_300);
            mat.set_color("dark_color", Color::YELLOW_400);
            clear_color = Color::YELLOW_200;
        }

        if input.key_pressed(KeyCode::KeyG) {
            let mat = suzanne.material();

            mat.set_color("regular_color", Color::SLATE_300);
            mat.set_color("dark_color", Color::SLATE_400);
            clear_color = Color::PURPLE_200;
        }

        if input.key_pressed(KeyCode::KeyB) {
            let mat = suzanne.material();

            mat.set_color("regular_color", Color::BLUE_300.hue_rotate(-10.0));
            mat.set_color("dark_color", Color::BLUE_400.desaturate(0.5));
            clear_color = Color::EMERALD_300;
        }

        if input.key_held(KeyCode::KeyR) {
            clear_color = clear_color.hue_rotate_oklch(5.0);
        }

        clear_screen(clear_color);
        orbit_controller.update(input);

        if show_many {
            suzanne.draw_many(transforms.clone());
        } else {
            suzanne.draw();
        }

        if show_many {
            mutate_camera_3d(|camera| {
                camera.eye = Vec3::new(
                    grid_center,
                    grid_center * 0.4,
                    grid_center + grid_width * 0.6,
                );
                camera.target = Vec3::new(grid_center, 0.0, grid_center);
            });
        }

        if should_quit() {
            break;
        }

        run_ui(|_| {});
        next_frame();
    }

    Ok(())
}
