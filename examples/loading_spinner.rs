use std::f32::consts::TAU;

use sge::*;
use sge_types::Sdf;

#[main("Sectors")]
async fn main() {
    loop {
        clear_screen(Color::SLATE_800);

        let a = (time() * 7.0) % TAU;
        let b = a + oscillate(0.5, TAU - 0.5);

        let sdf = Sdf::ring(vec2(0., 0.), Vec2::splat(200.0), 30.0, a, b)
            .with_fill_solid(Color::SKY_500);
        draw_sdf_world(sdf);

        // let sdf = SdfInstance::arc(vec2(0., 0.), Vec2::splat(200.0), 15.0, a, b)
        //     .with_fill_solid(Color::SKY_500);
        // draw_sdf_world(sdf);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
