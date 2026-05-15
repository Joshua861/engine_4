use sge::prelude::*;

#[main("Gradients")]
async fn main() {
    loop {
        clear_screen(Color::YELLOW_300);

        let sdf = Sdf::square_tl(Vec2::ZERO, 1000.0)
            .with_fill_gradient(Color::WHITE, Color::BLACK, time())
            .with_stroke(10.0, Color::RED_500, SdfStroke::Inside);
        draw_sdf(sdf);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
