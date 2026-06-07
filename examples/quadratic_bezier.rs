use sge::prelude::*;

#[main("Quadratic Bezier")]
async fn main() {
    set_cursor_visible(false);

    loop {
        clear_screen(Color::WHITE);

        let a = vec2(-300.0, 0.0);
        let b = screen_to_world(last_cursor_pos());
        let c = vec2(300.0, 0.0);

        for n in [0.0, 0.5] {
            let t = (time() + n) % 1.0;
            let p = point_on_quadratic_bezier(a, b, c, t);
            draw_circle_world(p, 8.0, Color::BLACK);
        }

        for p in [a, c] {
            draw_line_world(b, p, 4.0, Color::BLACK);
        }

        draw_quadratic_bezier_world(a, b, c, Color::BLACK, 4.0);

        let m = (a + c) / 2.0;

        draw_circle_outline_world((a + m) / 2.0, 150.0, Color::BLACK, 2.0);
        draw_circle_outline_world((c + m) / 2.0, 150.0, Color::BLACK, 2.0);

        draw_dashed_line_world(a, c, 3.0, Color::BLACK, 9.0);

        for p in [a, b, c, m] {
            draw_circle_world(p, 5.0, Color::BLACK);
        }

        draw_text_ex_world("a", a + vec2(-40.0, -30.0), Color::BLACK, 40);
        if b.y < 0.0 {
            draw_text_ex_world("b", b + vec2(-10.0, -60.0), Color::BLACK, 40);
        } else {
            draw_text_ex_world("b", b + vec2(-10.0, 0.0), Color::BLACK, 40);
        }
        draw_text_ex_world("c", c + vec2(40.0, -30.0), Color::BLACK, 40);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
