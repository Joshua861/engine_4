use sge::prelude::*;

const TEXT: &str = "corporis fuga rerum aut accusantium similique corporis vero excepturi voluptas itaque et adipisci quae dicta aliquid aliquid itaque quisquam neque";

#[main("Wrapped text")]
fn main() {
    loop {
        clear_screen(Color::NEUTRAL_950);

        let w = oscillate(200.0, window_width() - 100.0);
        draw_rect(
            Vec2::splat(50.0),
            vec2(w, window_height() - 100.0),
            Color::NEUTRAL_900,
        );

        let size = measure_wrapped_text(TEXT, w, 24, 1.2);
        draw_rect(Vec2::splat(50.0), size, Color::NEUTRAL_100.with_alpha(0.1));

        let drawn_size = draw_wrapped_text(TEXT, Vec2::splat(50.0), Color::WHITE, 24, 1.2, w);
        draw_rect(
            Vec2::splat(50.0),
            drawn_size,
            Color::NEUTRAL_100.with_alpha(0.1),
        );

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
