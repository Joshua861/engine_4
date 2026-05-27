use sge::*;

#[main("Sectors")]
async fn main() {
    loop {
        clear_screen(Color::SLATE_800);

        let a = (time() * 7.0) % TAU;
        let b = a + oscillate(0.5, TAU - 0.5);

        draw_ring_world(Vec2::ZERO, 200.0, 30.0, a, b, Color::SKY_500);

        // let sdf = SdfInstance::arc(vec2(0., 0.), Vec2::splat(200.0), 15.0, a, b)
        //     .with_fill_solid(Color::SKY_500);
        // draw_sdf_world(sdf);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
