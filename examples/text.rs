use engine_4::prelude::*;

actions! {
    PAUSE, DEBUG, SHOW_TEXTURE
}

fn main() -> anyhow::Result<()> {
    init("Text")?;

    bind! {
        PAUSE => KeyCode::Space;
        DEBUG => KeyCode::KeyD;
        SHOW_TEXTURE => KeyCode::KeyT;
    }

    let mut show_texture = false;
    let mut text = String::new();

    loop {
        clear_screen(Color::GRAY_900);
        draw_fps();

        if action_pressed(PAUSE) && held_control() {
            toggle_physics_timer();
        }

        if action_pressed(DEBUG) && held_control() {
            toggle_debug_info();
        }

        if action_pressed(SHOW_TEXTURE) && held_control() {
            show_texture = !show_texture;
        }

        if show_texture {
            let texture = default_font().texture();
            let scale = window_height().min(window_width()) / 2.0;
            let dimensions = texture.normalized_dimensions * scale;
            draw_rect_outline_world(-dimensions / 2.0, dimensions, 2.0, Color::GRAY_800);
            draw_sprite_world(texture, -dimensions / 2.0, scale);
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
            }

            let position = vec2(10.0, 40.0);
            if text.is_empty() {
                draw_text_ex(
                    "Start typing...",
                    TextDrawParams {
                        position,
                        color: Color::GRAY_400,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(&text, position);
            }

            draw_circle_world(Vec2::ZERO, 20.0, Color::RED_300);

            let text = "Hello world";
            let mut params = TextDrawParams {
                font: None,
                position: Vec2::new(
                    physics_time().sin() * window_width() / 4.0,
                    physics_time().cos() * window_height() / 4.0,
                ),
                font_size: 100,
                color: Color::PINK_300,
                do_dpi_scaling: true,
            };
            let dimensions = measure_text_ex(text, params);
            params.position -= dimensions.size / 2.0;
            draw_rect_world(params.position, dimensions.size, Color::GRAY_800);
            draw_text_world_ex(text, params);
        }

        if should_quit() {
            break;
        }

        run_ui(|_| {});

        next_frame();
    }

    Ok(())
}
