use engine_4::prelude::*;
use palette::encoding::Linear;

fn random_rect() -> Rect {
    let sf = window_height().min(window_width());
    let top_left: Vec2 = rand::<Vec2>() * sf;
    let size: Vec2 = rand::<Vec2>() * 300.0;
    let color: Color = random_color();

    Rect {
        top_left,
        size,
        color,
    }
}

fn main() -> anyhow::Result<()> {
    init("Animation")?;

    let mut animation_controller =
        AnimationController::new(random_rect(), random_rect(), 0.5, LinearEasingFunction);

    loop {
        draw_shape(&animation_controller.value());

        if animation_controller.is_complete() {
            animation_controller.now_animate_towards(random_rect());
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
