use sge::prelude::*;

#[main("Radial")]
fn main() {
    loop {
        clear_screen(Color::NEUTRAL_100);

        draw_sdf(
            Sdf::circle(Vec2::splat(200.0), 200.0)
                .with_fill_radial_gradient(Color::RED_500, Color::NEUTRAL_100),
        );

        draw_sdf(
            Sdf::circle(Vec2::new(600.0, 200.0), 200.0)
                .with_fill_radial_gradient(Color::SKY_500, Color::NEUTRAL_100)
                .with_fill_offset(vec2(-100.0, 0.0)),
        );

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
