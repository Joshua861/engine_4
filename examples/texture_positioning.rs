use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Texture positioning")?;

    let sprite = load_texture(
        include_bytes!("../assets/textures/guy.jpg"),
        ImageFormat::Jpeg,
    )?;

    loop {
        draw_sprite(sprite, Vec2::ZERO, 500.0);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
