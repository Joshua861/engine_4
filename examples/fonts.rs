use sge::prelude::*;

#[main("Fonts")]
async fn main() -> anyhow::Result<()> {
    // this font is included in the SANS constant, if you use the extra-fonts feauture
    let mut inter = load_font_sync(include_bytes!("../crates/sge_text/assets/inter.ttf"))?;

    loop {
        inter.draw_text(
            "Hello world, from Inter".to_string(),
            Vec2::splat(100.0),
            Color::WHITE,
            100,
            1.0,
        );

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
