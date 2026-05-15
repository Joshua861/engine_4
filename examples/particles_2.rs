use sge::prelude::*;

#[main("Particles 2")]
fn main() {
    let mut ps = ParticleSystem::new();
    let batch = ParticleOneshot::builder()
        .shape(&Rect::new_square(Vec2::ZERO, 20.0, Color::PURPLE_500))
        .size_randomness(5.0)
        .color_randomness(Color::new(0.3, 0.2, 0.3))
        .speed(15.0)
        .speed_randomness(3.0)
        .rotation_speed_randomness(0.5)
        .acceleration(vec2(0.0, 1.0))
        .direction_randomness(9.0)
        .end_color(Color::RED_700)
        .lifetime(1.1)
        .quantity(100)
        .build();

    loop {
        draw_multiline_text(
            format!(
                "Press space\n{} particles\n{:.2} FPS",
                ps.num_particles(),
                avg_fps()
            ),
            Vec2::ZERO,
            1.0,
        );

        for pos in cursor_movements() {
            ps.spawn_oneshot(&batch, pos);
        }

        ps.update();
        ps.draw();

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
