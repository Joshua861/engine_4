use sge::prelude::*;

#[main("Debug textures")]
async fn main() -> anyhow::Result<()> {
    let mut n: usize = 0;

    loop {
        clear_screen(Color::NEUTRAL_900);

        draw_text_ex(n, Vec2::ZERO, Color::WHITE, 50);

        if key_pressed(KeyCode::ArrowLeft) {
            n = n.wrapping_sub(1);
            n %= num_registered_textures();
        }

        if key_pressed(KeyCode::ArrowRight) {
            n += 1;
            n %= num_registered_textures();
        }

        let r = unsafe { TextureRef::new_indexed(n) };
        draw_text_ex(r.dimensions, vec2(0.0, 50.0), Color::WHITE, 50);
        let dim = r.normalized_dimensions * 500.0;
        draw_rect(Vec2::splat(200.0), dim, Color::NEUTRAL_500);
        draw_texture(r, Vec2::splat(200.0), 500.0);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
