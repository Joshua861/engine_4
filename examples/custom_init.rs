use sge::prelude::*;

struct PhysicsCircle {
    circle: Circle,
    velocity: Vec2,
}

fn main() -> anyhow::Result<()> {
    let opts = EngineCreationOptions::builder()
        .window_transparent(true)
        .opengl_debug(true)
        .dithering(false)
        .opengl_profile(GlProfile::Core)
        .default_magnify_filter(MagnifySamplerFilter::Nearest)
        .log_verbosity(Verbosity::Medium)
        .window_blur(true)
        // .swap_interval(SwapInterval::DontWait)
        .title("Custom init".to_string())
        .build();

    init_custom(opts)?;

    let mut obj = PhysicsCircle {
        circle: Circle {
            center: Vec2::splat(300.0),
            radius: Vec2::splat(100.0),
            color: Color::RED_400,
        },
        velocity: vec2(100.0, 200.0),
    };

    run_async(async move {
        loop {
            clear_screen(Color::TRANSPARENT);

            if obj.circle.center.x >= window_width() - obj.circle.radius.x
                || obj.circle.center.x <= obj.circle.radius.x
            {
                obj.velocity.x = -obj.velocity.x;
            }

            if obj.circle.center.y >= window_height() - obj.circle.radius.y
                || obj.circle.center.y <= obj.circle.radius.y
            {
                obj.velocity.y = -obj.velocity.y;
            }

            obj.circle.center += obj.velocity * physics_delta_time();
            dbg!(obj.circle.center);

            draw(&obj.circle);
            draw_fps();

            if should_quit() {
                break;
            }

            next_frame().await;
        }
    });

    Ok(())
}
