use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Demo")?;
    use_linear_filtering();

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

        draw_fullscreen_texture(guy_texture);
        vignette_screen(Color::BLACK, 0.2);

        if key_pressed(KeyCode::KeyD) {
            toggle_debug_info();
        }

        let dimensions = Vec2::new(100.0, 100.0);
        for y in 0..100 {
            for x in 0..100 {
                let texture = if x % 2 == y % 2 {
                    guy_texture
                } else {
                    pasta_texture
                };

                let x = x as f32 * dimensions.x;
                let y = y as f32 * dimensions.y;

                draw_texture_scaled_world(texture, Vec2::new(x, y), dimensions);
            }
        }

        run_ui(|_| {});

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
