use bevy_math::{UVec2, VectorSpace};
use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Render textures")?;
    use_nearest_filtering();

    let size = UVec2::new(50, 50);
    let fsize = size.as_vec2();
    let render_texture = create_empty_render_texture(size.x, size.y)?;

    loop {
        clear_screen(Color::BLACK);

        start_rendering_to_texture(render_texture);
        clear_screen(Color::WHITE);
        draw_circle_world(Vec2::ZERO, 25.0, Color::PINK_500);
        draw_tri(
            Vec2::new(0.0, fsize.y),
            fsize / 2.0,
            fsize,
            Color::GREEN_500,
        );
        draw_square(Vec2::ZERO, 10.0, Color::SKY_400);
        end_rendering_to_texture();

        // draw_sprite_scaled_world(render_texture.color_texture, Vec2::ZERO, fsize * 20.0);
        draw_fullscreen_texture(render_texture.color_texture);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
