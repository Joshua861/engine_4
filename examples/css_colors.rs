use sge::prelude::*;

#[main("CSS Colors")]
fn main() {
    assert_eq!(
        Color::from_string("forest green"),
        Some(Color::FOREST_GREEN),
    );

    loop {
        for (i, c) in [
            Color::RED,
            Color::REBECCA_PURPLE,
            Color::NAVAJO_WHITE,
            Color::SALMON,
            Color::FIRE_BRICK,
            Color::CORAL,
            Color::FOREST_GREEN,
        ]
        .into_iter()
        .enumerate()
        {
            let p = vec2(10.0, 10.0 + i as f32 * 60.0);
            draw_rect(p, vec2(50.0, 50.0), c);
            draw_text(c.to_hex_string(), p + vec2(60.0, 15.0));
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
