use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Ellipse & Circle Outline Demo")?;

    let mut time = 0.0;

    loop {
        clear_screen(Color::NEUTRAL_100);
        let input = get_input();
        time += delta_time();

        draw_circle(Vec2::new(100.0, 100.0), 50.0, Color::RED_500);
        draw_circle(Vec2::new(250.0, 100.0), 40.0, Color::BLUE_500);

        draw_circle_with_outline(
            Vec2::new(400.0, 100.0),
            50.0,
            Color::GREEN_500,
            Color::GREEN_900,
            5.0,
        );

        draw_circle_outline(Vec2::new(550.0, 100.0), 50.0, Color::YELLOW_500, 8.0);

        draw_ellipse(
            Vec2::new(100.0, 250.0),
            Vec2::new(80.0, 40.0),
            Color::PURPLE_500,
        );

        draw_ellipse(
            Vec2::new(250.0, 250.0),
            Vec2::new(40.0, 80.0),
            Color::PINK_500,
        );

        draw_ellipse_with_outline(
            Vec2::new(400.0, 250.0),
            Vec2::new(70.0, 45.0),
            Color::CYAN_500,
            Color::CYAN_900,
            5.0,
        );

        draw_ellipse_outline(
            Vec2::new(550.0, 250.0),
            Vec2::new(60.0, 40.0),
            Color::ORANGE_500,
            6.0,
        );

        let angle = time * 2.0;
        let rx = 60.0 + (angle * 1.5).sin() * 20.0;
        let ry = 40.0 + (angle * 2.0).cos() * 15.0;
        draw_ellipse_with_outline(
            Vec2::new(100.0, 400.0),
            Vec2::new(rx, ry),
            Color::EMERALD_500,
            Color::EMERALD_900,
            4.0,
        );

        let pulse = (time * 3.0).sin() * 10.0 + 50.0;
        let outline_pulse = (time * 3.0).cos() * 2.0 + 5.0;
        draw_circle_with_outline(
            Vec2::new(250.0, 400.0),
            pulse,
            Color::ROSE_500,
            Color::ROSE_900,
            outline_pulse,
        );

        for i in 0..3 {
            let radius = 20.0 + i as f32 * 15.0;
            let hue = i as f32 * 120.0;
            let color = Color::hsl(hue, 0.7, 0.5);
            draw_circle_outline(Vec2::new(400.0, 400.0), radius, color, 3.0);
        }

        draw_circle_world(Vec2::new(-100.0, -100.0), 30.0, Color::SLATE_500);
        draw_ellipse_world(Vec2::new(0.0, 0.0), Vec2::new(50.0, 30.0), Color::SLATE_300);
        draw_circle_with_outline_world(
            Vec2::new(100.0, 0.0),
            40.0,
            Color::AMBER_500,
            Color::AMBER_900,
            5.0,
        );

        if input.mouse_held(MouseButton::Left) {
            let diff: Vec2 = input.mouse_diff().into();
            mutate_camera_2d(|camera| {
                camera.translation -= diff / camera.scale;
            });
        }

        if input.scroll_diff().1 != 0.0 {
            let cursor_pos = input
                .cursor()
                .map(|(x, y)| Vec2::new(x, y))
                .unwrap_or(Vec2::ZERO);
            let diff = input.scroll_diff().1;
            let diff = (diff * 0.1) + 1.0;
            camera2d_zoom_at(cursor_pos, diff);
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
