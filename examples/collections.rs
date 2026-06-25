use sge::prelude::*;
use sge_rendering::d2::Collection2D;

#[main("Collections")]
fn main() -> anyhow::Result<()> {
    let collection = arrows();
    let mut controller = PanningCameraController::new();

    loop {
        draw_fps();
        controller.update();

        collection.draw_world();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn arrows() -> Collection2D {
    let mut collection = Collection2D::empty();
    let renderer = collection.renderer();

    let thickness = 5.0;
    draw_circle_to(vec2(-50.0, -60.0), 10.0, Color::RED_400, renderer);
    draw_circle_to(vec2(50.0, -60.0), 10.0, Color::RED_400, renderer);
    draw_zig_zag_ex_to(
        vec2(-50.0, -60.0),
        vec2(50.0, -60.0),
        thickness,
        Color::WHITE,
        5.0,
        10,
        renderer,
    );
    draw_arrow_to(
        vec2(-50.0, -30.0),
        vec2(50.0, -30.0),
        thickness,
        Color::WHITE,
        renderer,
    );
    draw_solid_arrow_to(
        vec2(-50.0, 0.0),
        vec2(50.0, 0.0),
        thickness,
        Color::WHITE,
        renderer,
    );
    draw_sharp_arrow_to(
        vec2(-50.0, 30.0),
        vec2(50.0, 30.0),
        thickness,
        Color::WHITE,
        renderer,
    );

    draw_right_angled_arrow_to(
        vec2(50.0, 50.0),
        vec2(150.0, 300.0),
        thickness,
        Color::WHITE,
        renderer,
    );

    collection
}
