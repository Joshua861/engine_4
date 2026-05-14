use std::f32::consts::TAU;

use sge::*;

#[main("Sectors")]
async fn main() {
    loop {
        clear_screen(Color::SLATE_800);

        let a = (time() * 7.0) % TAU;
        let b = a + oscillate(0.5, TAU - 0.5);

        draw_sector_outline_world(vec2(0.0, 0.0), 200.0, a, b, Color::SKY_500, 30.0);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
