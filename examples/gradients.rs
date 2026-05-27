use sge::prelude::*;

#[main("Gradients")]
async fn main() {
    let mut controller = PanningCameraController::new();

    loop {
        controller.update();

        clear_screen(Color::YELLOW_300);

        let sdf = Sdf::square_tl(Vec2::ZERO, 1000.0)
            .with_fill_gradient(Color::WHITE, Color::BLACK, time())
            .with_stroke(10.0, Color::RED_500, SdfStroke::Inside)
            .with_corner_radius(50.0)
            .with_shadow(vec2(0.0, 0.0), 20., Color::BLACK);
        draw_sdf_world(sdf);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
