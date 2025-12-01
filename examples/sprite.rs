use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Demo")?;
    use_nearest_filtering();

    let mut is_dark_mode = false;
    let mut controller = PanningCameraController::new();
    let guy_texture = load_texture(
        include_bytes!("../assets/textures/guy.jpg"),
        ImageFormat::Jpeg,
    )?;
    let pasta_texture = load_texture(
        include_bytes!("../assets/textures/pasta.jpg"),
        ImageFormat::Jpeg,
    )?;

    loop {
        controller.update();

        if key_pressed(KeyCode::Space) {
            is_dark_mode = !is_dark_mode;
        }

        if key_pressed(KeyCode::KeyD) {
            show_debug_info();
        }

        if is_dark_mode {
            clear_screen(Color::NEUTRAL_900);
        } else {
            clear_screen(Color::NEUTRAL_100);
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
