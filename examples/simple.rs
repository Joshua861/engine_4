use sge::prelude::*;

#[main("Simple example")]
async fn main() -> anyhow::Result<()> {
    loop {
        clear_screen(Color::NEUTRAL_200);

        draw_circle(vec2(40.0, 80.0), 30.0, Color::YELLOW_400);
        draw_square(vec2(100.0, 40.0), 20.0, Color::RED_300);
        draw_colored_text(
            format!("{} x {}", window_width(), window_height()),
            vec2(40.0, 80.0),
            Color::BLACK,
        );
        draw_fps_black();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
