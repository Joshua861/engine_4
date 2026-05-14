use std::f32::consts::TAU;

use sge::*;

#[main("Sectors")]
async fn main() {
    loop {
        clear_screen(Color::NEUTRAL_200);

        let a = (time() * 2.0) % TAU;
        let b = a + oscillate(0.0, TAU);

        draw_ellipse_sector_world(vec2(0.0, 0.0), vec2(400.0, 200.0), a, b, Color::RED_300);
        draw_sector_world(vec2(0.0, 0.0), 200.0, a, b, Color::RED_500);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
