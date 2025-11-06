use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Demo")?;

    let mut is_dark_mode = false;
    let mut cursor_pos = Vec2::ZERO;

    loop {
        let input = get_input();

        if input.key_pressed(KeyCode::Space) {
            is_dark_mode = !is_dark_mode;
        }

        if let Some((x, y)) = input.cursor() {
            cursor_pos = Vec2::new(x, y);
        }

        if is_dark_mode {
            clear_screen(Color::NEUTRAL_900);
        } else {
            clear_screen(Color::NEUTRAL_100);
        }

        draw_circle(Vec2::new(400.0, 300.0), 100.0, Color::RED_500);
        draw_square(Vec2::splat(200.), 200., Color::AMBER_300);
        draw_square_outline(Vec2::splat(200.), 200., 10., Color::AMBER_400);
        draw_rect(
            Vec2::new(100.0, 0.0),
            Vec2::new(100.0, 200.0),
            Color::SKY_300,
        );
        draw_tri(
            Vec2::splat(300.),
            Vec2::new(400.0, 300.0),
            Vec2::new(500.0, 600.0),
            Color::ROSE_700,
        );
        draw_tri_outline(
            Vec2::splat(300.),
            Vec2::new(400.0, 300.0),
            Vec2::new(500.0, 600.0),
            10.0,
            Color::ROSE_500,
        );

        draw_square_world(Vec2::splat(-50.0), 100.0, Color::PINK_300);

        // draw_poly(Vec2::splat(500.0), 7, 100.0, 5.0, Color::EMERALD_500);

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

        for y in 0..20 {
            for x in 0..20 {
                draw_square_world(
                    Vec2::new(x as f32 * 500.0 - 50.0, y as f32 * 500.0 - 50.0),
                    500.0,
                    if x % 2 == y % 2 {
                        Color::NEUTRAL_100
                    } else {
                        Color::NEUTRAL_900
                    },
                );
            }
        }

        for y in 0..100 {
            for x in 0..100 {
                draw_circle_world(
                    Vec2::new(x as f32 * 100.0, y as f32 * 100.0),
                    50.0,
                    Color::NEUTRAL_500,
                );
            }
        }

        run_ui(|ctx| {
            egui::Window::new("Hello, world")
                .collapsible(false)
                .movable(false)
                .resizable(false)
                .hscroll(false)
                .vscroll(false)
                .show(ctx, |ui| {
                    ui.label("This is a perfect engine");

                    if ui.button("Click me!").clicked() {
                        is_dark_mode = !is_dark_mode;
                    }

                    ui.label(&format!("FPS: {:.1}", avg_fps()));
                });
        });

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
