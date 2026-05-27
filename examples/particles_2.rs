use sge::prelude::*;

fn main() {
    let opts = Opts::builder()
        .title("Particles 2".to_string())
        .swap_interval(SwapInterval::DontWait)
        .build();
    init_custom(opts).unwrap();

    let mut ps = ParticleSystem::new();
    let batch = ParticleOneshot::builder()
        .shape(&Rect::new_square(Vec2::ZERO, 20.0, Color::GRAY))
        .size_randomness(5.0)
        .color_randomness(Color::GRAY)
        .speed(15.0)
        .speed_randomness(3.0)
        .rotation_speed_randomness(0.5)
        .acceleration(vec2(0.0, 1.0))
        .direction_randomness(9.0)
        .end_color(Color::BLACK)
        .lifetime(1.1)
        .quantity(100)
        .build();

    run_async(async move {
        loop {
            dont_clear_screen();

            draw_fps_bg();

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
    });
}
