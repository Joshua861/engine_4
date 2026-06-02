use sge::prelude::*;

#[main("SDF")]
fn main() {
    let mut controller = PanningCameraController::new();

    loop {
        clear_screen(Color::SLATE_500);

        controller.update();

        let mut cursor = Vec2::ZERO;

        for (i, f) in [
            |c| Sdf::rect(c, vec2(100.0, 50.0)),
            |c| Sdf::square(c, 100.0),
            |c| Sdf::circle(c, 50.0),
            |c| Sdf::ellipse(c, vec2(50.0, 25.0)),
            |c| {
                Sdf::quad(
                    c + vec2(-40.0, -30.0),
                    c + vec2(50.0, -35.0),
                    c + vec2(40.0, 45.0),
                    c + vec2(-30.0, 44.0),
                )
            },
            |c| {
                Sdf::arc(
                    c,
                    Vec2::splat(50.0),
                    20.0,
                    time(),
                    time() + oscillate(0.5, 5.0),
                )
            },
            |c| {
                Sdf::ring(
                    c,
                    Vec2::splat(50.0),
                    20.0,
                    time(),
                    time() + oscillate(0.5, 5.0),
                )
            },
            |c| Sdf::sector(c, Vec2::splat(50.0), time(), time() + oscillate(0.5, 5.0)),
            |c| {
                Sdf::triangle(
                    c + vec2(0.0, -50.0),
                    c + vec2(-50.0, 50.0),
                    c + vec2(50.0, 50.0),
                )
            },
            |c| Sdf::pentagon(c, 50.0),
            |c| Sdf::hexagon(c, 50.0),
            |c| Sdf::octogon(c, 50.0),
            |c| Sdf::hexagram(c, 50.0),
            |c| Sdf::pentagram(c, 50.0),
            |c| Sdf::star(c, 50.0, 12.0, 6.0),
            |c| Sdf::moon(c, 50.0, 25.0, 40.0),
            |c| Sdf::heart(c, 50.0),
            |c| {
                Sdf::quadratic_bezier(
                    c + vec2(-50.0, 50.0),
                    c + vec2(0.0, -50.0),
                    c + vec2(50.0, 50.0),
                )
            },
            |c| {
                Sdf::cubic_bezier(
                    c + vec2(-50.0, 50.0),
                    c + vec2(-50.0, -50.0),
                    c + vec2(50.0, -50.0),
                    c + vec2(50.0, 50.0),
                )
            },
            |c| Sdf::quadratic_circle(c, 50.0),
            |c| Sdf::segment(c + vec2(-50.0, -50.0), c + vec2(50.0, 50.0)),
            |c| Sdf::oriented_box(c + vec2(-50.0, -50.0), c + vec2(50.0, 50.0), 30.0),
        ]
        .iter()
        .enumerate()
        {
            let palette = Palette::PALETTES[i % 22];
            let main = palette.v400;
            let alt = palette.v500;
            let outline = palette.v200;

            let sdf = f(cursor);
            let thickness = if sdf.shape_type == SdfShape::Ring {
                0.0
            } else {
                5.0
            };
            let radius = if sdf.shape_type == SdfShape::Ring
                || sdf.shape_type == SdfShape::Heart
                || sdf.shape_type == SdfShape::QuadraticCircle
            {
                0.0
            } else {
                10.0
            };
            let sdf = sdf
                .with_fill(main, alt, 0.0, 5.0, unsafe {
                    std::mem::transmute((i + 1) as i32)
                })
                .with_stroke(thickness, outline, SdfStroke::Outside)
                .with_corner_radius(oscillate(0.0, radius))
                .with_shadow(vec2(10.0, 20.0), 0.5, Color::SLATE_600);

            cursor += vec2(200.0, 0.0);

            if cursor.x > 1000.0 {
                cursor.x = 0.0;
                cursor.y += 200.0;
            }

            draw_sdf_world(sdf);
        }

        vignette_screen(Color::SLATE_700, 0.1);

        draw_fps_bg();

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
