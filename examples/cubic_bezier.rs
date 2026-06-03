use sge::prelude::*;

#[main("Cubic Bezier")]
async fn main() {
    set_cursor_visible(false);

    loop {
        clear_screen(Color::RED_200);

        let a = vec2(-300.0, 0.0);
        let b = a + vec2(0.0, -300.0);
        let c = vec2(300.0, 0.0);
        let d = screen_to_world(last_cursor_pos());

        for p in [b, c] {
            draw_circle_world(p, 3.0, Color::RED_300);
        }

        let sdf = Sdf::cubic_bezier(a, b, c, d)
            .with_fill(
                Color::RED_500,
                Color::RED_400,
                FRAC_PI_4,
                7.5,
                SdfFill::Gradient,
            )
            .with_corner_radius(50.0)
            .with_shadow(vec2(5.0, 5.0), 5.0, Color::RED_700.with_alpha(0.3));
        draw_sdf_world(sdf);

        for n in [0.0, 0.5] {
            let t = (time() + n) % 1.0;
            let p = point_on_cubic_bezier(a, b, c, d, t);
            let d = tangent_to_cubic_bezier(a, b, c, d, t);
            draw_circle_world(p, 8.0, Color::RED_600);
            draw_arrow_world(p, p + d * 100.0, 4.0, Color::RED_600);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
