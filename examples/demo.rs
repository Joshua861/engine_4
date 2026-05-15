use std::f32::consts::FRAC_PI_3;

use sge::prelude::*;
use ui::*;

#[main("Demo")]
async fn main() -> anyhow::Result<()> {
    let mut is_dark_mode = false;
    let mut camera_controller = PanningCameraController::new();
    let mut screenshake = CameraShakeController::new(50.0, 16.0, 2.0);
    let guy_texture = include_texture!("../assets/textures/guy.jpg");
    let mut show_debug_info = false;

    loop {
        if !ui_consumed_input() {
            camera_controller.update();
        }
        screenshake.update();

        if key_pressed(KeyCode::Space) {
            is_dark_mode = !is_dark_mode;
        }

        if key_pressed(KeyCode::KeyD) && held_control() {
            toggle_wireframe();
        } else if key_pressed(KeyCode::KeyD) {
            show_debug_info = !show_debug_info;
        }

        if is_dark_mode {
            clear_screen(Color::NEUTRAL_900);
        } else {
            clear_screen(Color::NEUTRAL_100);
        }

        draw_poly(Vec2::splat(500.0), 7, 100.0, 5.0, Color::GREEN_500);
        draw_circle(Vec2::new(400.0, 300.0), 100.0, Color::RED_500);
        draw_square(Vec2::splat(200.), 200., Color::AMBER_400);
        draw_square_outline(Vec2::splat(200.), 200., 10., Color::AMBER_500);
        draw_rect(
            Vec2::new(100.0, 0.0),
            Vec2::new(100.0, 200.0),
            Color::SKY_400,
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
        draw_hexagon(Vec2::new(1000.0, 200.0), 200.0, Color::BLUE_500);
        draw_hexagon_pointy(
            Vec2::new(1000.0, 200.0),
            100.0 * 3.0_f32.sqrt(),
            Color::BLUE_400,
        );

        draw_square_world(Vec2::splat(-50.0), 100.0, Color::PINK_400);

        for y in 0..20 {
            for x in 0..20 {
                draw_square_world(
                    Vec2::new(x as f32 * 500.0 - 50.0, y as f32 * 500.0 - 50.0),
                    500.0,
                    if x % 2 == y % 2 {
                        Color::NEUTRAL_200
                    } else {
                        Color::BLACK
                    },
                );
            }
        }

        let cursor_pos = last_cursor_pos();
        for y in 0..100 {
            for x in 0..100 {
                let x = x as f32 * 100.0;
                let y = y as f32 * 100.0;
                let mouse_pos = screen_to_world(cursor_pos);
                let mouse_pos = collision::Point::new(mouse_pos);
                let is_hovered = collision::circle(x, y, 50.0).intersects_with(&mouse_pos);

                let color = if is_hovered {
                    Color::RED_600
                } else {
                    Color::NEUTRAL_800
                };

                let pos = Vec2::new(x, y);
                draw_circle_world(pos, 50.0, color);

                if is_hovered {
                    use ui::prelude::*;
                    let ui = Fit::new(Fill::new(
                        Color::NEUTRAL_900,
                        Padding::all(
                            20.0,
                            Col::new([
                                Text::h2_no_padding("Circle"),
                                Text::mono_nowrap(format!("Id: {}", y as usize + x as usize / 100)),
                                Text::mono_nowrap(format!("Position: ({}, {})", x, y)),
                                Text::mono_nowrap(format!(
                                    "Cursor: ({:.0}, {:.0})",
                                    cursor_pos.x, cursor_pos.y
                                )),
                                EmptyBox::height(15.0),
                                SizedBox::height(20.0, Fill::new(color, EMPTY)),
                            ]),
                        ),
                    ));

                    if cursor().is_some() {
                        draw_ui_unbounded(ui, cursor_pos);
                    }
                }
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
            Color::SKY_400,
            None,
        );

        {
            let points: Vec<Vec2> = (0..10)
                .map(|_| Vec2::new(rand::<f32>() * 300.0 + 400.0, rand::<f32>() * 300.0 + 100.0))
                .collect();
            draw_custom_shape(&points, Color::YELLOW_500);
        }

        if show_debug_info {
            draw_debug_info();
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
