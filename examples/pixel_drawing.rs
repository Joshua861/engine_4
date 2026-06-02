use sge::prelude::*;

#[main("Pixel drawing")]
async fn main() {
    loop {
        draw_pixel_line(vec2(0.0, 0.0), vec2(50.0, 50.0), Color::WHITE);
        draw_pixel_line(vec2(20.0, 0.0), vec2(30.0, 10.0), Color::WHITE);
        draw_pixel(vec2(100.0, 100.0), Color::RED_500);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
