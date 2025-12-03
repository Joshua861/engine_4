use std::f32::consts::FRAC_PI_3;

use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Demo")?;

    let mut is_dark_mode = false;
    let mut camera_controller = PanningCameraController::new();
    let guy_texture = load_texture(
        include_bytes!("../assets/textures/guy.jpg"),
        ImageFormat::Jpeg,
    )?;

    loop {
        camera_controller.update();

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

        draw_poly(Vec2::splat(500.0), 7, 100.0, 5.0, Color::EMERALD_500);
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
        draw_hexagon(Vec2::new(1000.0, 200.0), 200.0, Color::BLUE_400);
        draw_hexagon_pointy(
            Vec2::new(1000.0, 200.0),
            100.0 * 3.0_f32.sqrt(),
            Color::BLUE_200,
        );

        draw_square_world(Vec2::splat(-50.0), 100.0, Color::PINK_300);

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
                let x = x as f32 * 100.0;
                let y = y as f32 * 100.0;
                let mouse_pos = screen_to_world(cursor_pos());
                let mouse_pos = collisions::Point::new(mouse_pos);
                let color = if collisions::circle(x, y, 50.0).intersects_with(&mouse_pos) {
                    Color::RED_500
                } else {
                    Color::NEUTRAL_500
                };

                draw_circle_world(Vec2::new(x, y), 50.0, color);
            }
        }

        draw_texture_world(guy_texture, Vec2::new(0.0, 0.0), 50.0);
        draw_texture_world_ex(
            guy_texture,
            Transform2D::from_scale_rotation_translation(
                Vec2::new(100.0, 30.0),
                FRAC_PI_3,
                Vec2::new(100.0, 0.0),
            ),
            Color::SKY_300,
            None,
        );

        {
            let points: Vec<Vec2> = (0..10)
                .map(|_| Vec2::new(rand::<f32>() * 300.0 + 400.0, rand::<f32>() * 300.0 + 100.0))
                .collect();
            draw_custom_shape(points, Color::YELLOW_500);
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
