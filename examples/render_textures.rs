use sge::prelude::*;
use sge_vectors::UVec2;

#[main("Render textures")]
fn main() -> anyhow::Result<()> {
    use_nearest_filtering();

    let size = UVec2::new(50, 50);
    let fsize = size.as_vec2();
    let render_texture = create_empty_render_texture(size.x, size.y)?;

    loop {
        start_rendering_to_texture(render_texture);
        clear_screen(Color::WHITE);
        draw_ellipse(fsize / 2.0, fsize / 4.0, Color::RED_500);
        draw_tri(
            Vec2::new(0.0, fsize.y),
            fsize / 2.0,
            fsize,
            Color::GREEN_500,
        );
        draw_square(Vec2::ZERO, 10.0, Color::SKY_400);
        end_rendering_to_texture();

        draw_fullscreen_texture(render_texture.color_texture);
        draw_fps_bg();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
