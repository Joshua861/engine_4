use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Demo")?;
    use_nearest_filtering();

    let mut is_dark_mode = false;
    let mut cursor_pos = Vec2::ZERO;
    let guy_texture = load_texture(
        include_bytes!("../assets/textures/guy.jpg"),
        ImageFormat::Jpeg,
    )?;
    let pasta_texture = load_texture(
        include_bytes!("../assets/textures/pasta.jpg"),
        ImageFormat::Jpeg,
    )?;

    loop {
        let input = get_input();

        if input.key_pressed(KeyCode::Space) {
            is_dark_mode = !is_dark_mode;
        }

        if input.key_pressed(KeyCode::KeyD) {
            show_debug_info();
        }

        if let Some((x, y)) = input.cursor() {
            cursor_pos = Vec2::new(x, y);
        }

        if is_dark_mode {
            clear_screen(Color::NEUTRAL_900);
        } else {
            clear_screen(Color::NEUTRAL_100);
        }

        if input.mouse_held(MouseButton::Left) {
            let diff: Vec2 = input.mouse_diff().into();

            mutate_camera(|camera| {
                camera.translation -= diff / camera.scale;
            });
        }

        if input.scroll_diff().1 != 0.0 {
            let diff = input.scroll_diff().1;
            let diff = (diff * 0.1) + 1.0;

            camera_zoom_at(cursor_pos, diff);
        }

        let dimensions = Vec2::new(100.0, 100.0);
        for y in 0..1000 {
            for x in 0..300 {
                let texture = if x % 2 == y % 2 {
                    guy_texture
                } else {
                    pasta_texture
                };

                let x = x as f32 * dimensions.x as f32;
                let y = y as f32 * dimensions.y;

                draw_sprite_scaled_world(texture, Vec2::new(x, y), dimensions);
            }
        }

        run_ui(|ctx| {
            egui::Window::new("Hello, world").show(ctx, |ui| {
                ui.label("This is a perfect engine");

                if ui.button("Click me!").clicked() {
                    is_dark_mode = !is_dark_mode;
                }
            });
        });

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
