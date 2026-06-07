use sge::prelude::*;

actions! {
    PAUSE, DEBUG, SHOW_TEXTURE
}

#[main("Text")]
fn main() -> anyhow::Result<()> {
    bind! {
        PAUSE => KeyCode::Space;
        DEBUG => KeyCode::KeyD;
        SHOW_TEXTURE => KeyCode::KeyT;
    }

    let mut show_texture = false;
    let mut text = String::new();
    let mut show_debug_info = false;

    loop {
        clear_screen(Color::GRAY_900);
        draw_fps();

        if action_pressed(PAUSE) && held_control() {
            toggle_physics_timer();
        }

        if action_pressed(DEBUG) && held_control() {
            show_debug_info = !show_debug_info;
        }

        if action_pressed(SHOW_TEXTURE) && held_control() {
            show_texture = !show_texture;
        }

        if show_texture {
            let texture = default_font().texture();
            let scale = window_height().min(window_width()) / 2.0;
            let dimensions = texture.normalized_dimensions * scale;
            draw_rect_outline_world(-dimensions / 2.0, dimensions, 2.0, Color::GRAY_800);
            draw_texture_world(texture, -dimensions / 2.0, scale);
        } else {
            for key in input_text() {
                if let Key::Character(s) = key {
                    text.push_str(s.as_str());
                }

                if let Key::Named(NamedKey::Space) = key {
                    text.push(' ');
                }

                if let Key::Named(NamedKey::Backspace) = key {
                    text.pop();
                }

                if let Key::Named(NamedKey::Enter) = key {
                    text.push('\n');
                }
            }

            let position = vec2(10.0, 30.0);
            if text.is_empty() {
                draw_text_custom(
                    "Start typing...",
                    TextDrawParams {
                        position,
                        color: Color::GRAY_400,
                        ..Default::default()
                    },
                );
            } else {
                let dimensions = draw_text(&text, position);
                if toggle_every_n_seconds(1.0) {
                    draw_rect(
                        position + vec2(dimensions.size.x, 0.0),
                        Vec2::new(2.0, dimensions.size.y),
                        Color::NEUTRAL_200,
                    );
                }
            }

            draw_circle_world(Vec2::ZERO, 20.0, Color::RED_300);

            let text = "Hello world";
            let mut params = TextDrawParams {
                font: None,
                position: Vec2::new(
                    physics_time().sin() * min_window_dimension() / 4.0,
                    physics_time().cos() * min_window_dimension() / 4.0,
                ),
                font_size: 100,
                color: Color::PINK_300,
                line_spacing: 1.0,
            };
            let dimensions = measure_text_ex(text, MONO, 100);
            params.position -= dimensions.size / 2.0;
            draw_rect_world(params.position, dimensions.size, Color::GRAY_800);
            draw_text_custom_world(text, params);
        }

        if should_quit() {
            break;
        }

        if show_debug_info {
            draw_debug_info();
        }

        next_frame().await;
    }

    Ok(())
}
