use sge::prelude::*;

#[main("Cubic Bezier")]
async fn main() {
    set_cursor_visible(false);

    loop {
        clear_screen(Color::WHITE);

        let a = vec2(-300.0, 0.0);
        let b = a + vec2(0.0, -300.0);
        let c = vec2(300.0, 0.0);
        let d = screen_to_world(last_cursor_pos());

        draw_cubic_bezier_world(a, b, c, d, Color::BLACK, 4.0);

        for p in [a, b, c, d] {
            draw_circle_world(p, 3.0, Color::BLACK);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
