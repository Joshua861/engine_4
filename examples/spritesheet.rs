use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Title for the window")?;

    let mut sheet = create_spritesheet()?;
    let guy_image = load_image(
        include_bytes!("../assets/textures/guy.jpg"),
        ImageFormat::Jpeg,
    )?;
    let space_image = load_image(
        include_bytes!("../assets/textures/space.jpg"),
        ImageFormat::Jpeg,
    )?;
    let pasta_image = load_image(
        include_bytes!("../assets/textures/pasta.jpg"),
        ImageFormat::Jpeg,
    )?;

    let guy = sheet.cache_sprite(&guy_image);
    let space = sheet.cache_sprite(&space_image);
    let pasta = sheet.cache_sprite(&pasta_image);

    loop {
        clear_screen(Color::from_oklch(0.6067, 0.0987, time() * 20.0));

        sheet.draw(guy, Vec2::splat(20.0), 400.0);
        sheet.draw(space, vec2(440.0, 20.0), 400.0);
        sheet.draw(pasta, vec2(860.0, 20.0), 400.0);

        draw_rect_outline(
            vec2(20.0, 500.0) - Vec2::splat(5.0),
            sheet.texture.normalized_dimensions * 1000.0 + Vec2::splat(5.0),
            10.0,
            Color::WHITE,
        );
        draw_texture(sheet.texture, vec2(20.0, 500.0), 1000.0);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
