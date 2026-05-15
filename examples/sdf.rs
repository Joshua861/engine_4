use std::f32::consts::PI;

use sge::*;

#[main("SDF")]
fn main() {
    let mut controller = PanningCameraController::new();

    loop {
        clear_screen(Color::SLATE_500);

        controller.update();

        let sdf = SdfInstance::star(vec2(0.0, 0.0), 100.0, 5.0, PI)
            .with_rotation(time() * 2.)
            .with_shadow(
                vec2(
                    oscillate_t(5.0, 20.0, time() * 4.0),
                    oscillate_t(10.0, 40.0, time() * 4.0),
                ),
                0.5,
                Color::SLATE_600,
            )
            .with_corner_radius(15.0)
            .with_fill(
                Color::YELLOW_500,
                Color::YELLOW_400,
                0.3,
                10.0,
                SdfFill::Dots,
            )
            .with_stroke(10.0, Color::YELLOW_200, SdfStroke::Outside);
        draw_sdf_world(sdf);

        let points = [
            vec2(10.0, 10.0),
            vec2(20.0, 40.0),
            vec2(40.0, 40.0),
            vec2(30.0, 20.0),
        ];

        let sdf = SdfInstance::quad(
            vec2(10.0, 10.0),
            vec2(20.0, 40.0),
            vec2(40.0, 40.0),
            vec2(30.0, 20.0),
        )
        .with_stroke(1.0, Color::WHITE, SdfStroke::Centered)
        .with_fill_solid(Color::RED_500);
        draw_sdf_world(sdf);

        for point in points {
            draw_circle_world(point, 1.0, Color::BLACK);
        }

        vignette_screen(Color::SLATE_700, 0.1);

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
