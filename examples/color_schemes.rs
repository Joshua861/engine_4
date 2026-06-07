use sge::prelude::*;

const COLORS: ColorScheme = ColorScheme::LACKLUSTER;

#[main("Color schemes")]
async fn main() -> anyhow::Result<()> {
    let text = "Hello color schemes!";
    let mut text_params = TextDrawParams {
        color: COLORS.fg0,
        font_size: 50,
        ..Default::default()
    };

    loop {
        clear_screen(COLORS.bg0);

        text_params.position = draw_palette() + Vec2::splat(20.0);

        let dimensions = measure_text_ex(text, MONO, text_params.font_size);
        draw_rect(text_params.position, dimensions.size, COLORS.bg2);
        draw_text_custom(text, text_params);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn draw_palette() -> Vec2 {
    let palette = COLORS.palette();
    let mut cursor = Vec2::ZERO;
    let size = Vec2::splat(50.0);

    for color in palette {
        draw_rect(cursor, size, color);
        cursor.x += size.x;

        if cursor.x >= 500.0 {
            cursor.x = 0.0;
            cursor.y += size.y;
        }
    }

    cursor.x = 0.0;
    cursor.y += size.y;

    cursor
}
