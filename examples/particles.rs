use sge::prelude::*;

#[main("Particles")]
fn main() {
    let mut ps = ParticleSystem::new();
    let batch = ParticleOneshot::builder()
        .shape(&Rect::new_square(Vec2::ZERO, 20.0, Color::YELLOW_500))
        .size_randomness(5.0)
        .color_randomness(Color::new(0.3, 0.1, 0.1))
        .direction_randomness(0.5)
        .speed(40.0)
        .speed_randomness(3.0)
        .rotation_speed_randomness(0.2)
        .end_color(Color::RED_700)
        .acceleration(vec2(0.0, 1.0))
        .acceleration_randomness(vec2(0.0, 0.2))
        .lifetime(2.0)
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
            Color::WHITE,
            16,
            1.0,
        );

        if once_per_n_seconds(0.1) {
            ps.spawn_oneshot(&batch, last_cursor_pos());
        }

        ps.update();
        ps.draw();

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
