use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Text")?;

    let mut controller = PanningCameraController::new();

    show_debug_info();

    loop {
        clear_screen(Color::GRAY_900);
        controller.update();

        draw_circle_world(Vec2::ZERO, 20.0, Color::RED_300);

        if key_pressed(KeyCode::Space) {
            toggle_physics_timer();
        }

        let text = "Hello world";
        let params = TextDrawParams {
            font: None,
            position: Vec2::new(physics_time().sin() * 1000.0, physics_time().cos() * 500.0),
            font_size: 100,
            color: Color::PINK_300,
            do_dpi_scaling: true,
        };

        let dimensions = measure_text_ex(text, params);
        draw_rect_world(params.position, dimensions.size, Color::GRAY_800);
        draw_text_world_ex(text, params);

        if should_quit() {
            break;
        }

        run_ui(|_| {});

        next_frame();
    }

    Ok(())
}
