use sge::prelude::*;

#[main("Bitmap Fonts")]
async fn main() -> AResult<()> {
    let bytes = include_bytes!("../assets/textures/bitmap_font.png");
    let settings = BitmapFontSettings {
        char_size: uvec2(5, 8),
        advance: 6,
        gaps_in: 0,
        gaps_out: GapsOut::all(0),
        processing: BitmapFontProcessing::FullColor,
        layout: "abcdefghijklmnopqrstuvwxyz 1234567890".to_string(),
    };
    let mut font = load_bitmap_font_sync(bytes, &settings)?;

    loop {
        font.draw_text(
            "the quick brown fox\njumps over the lazy dog\n1234567890".to_string(),
            Vec2::splat(20.0),
            Color::WHITE,
            50,
            1.0,
        );

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
