use std::f32::consts::FRAC_PI_2;

use sge::prelude::*;

#[main("Screenshot")]
async fn main() -> anyhow::Result<()> {
    start_coroutine(draw_pattern());

    let mut textures = vec![];

    loop {
        if key_pressed(KeyCode::Space) {
            let screenshot = take_screenshot();
            textures.push(screenshot);
            info!("Screenshot taken");
        }

        draw_screenshots(&textures);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

// can ignore this if you only care about screenshots

fn draw_screenshots(screenshots: &[TextureRef]) {
    use ui::*;

    let gap = min_window_dimension() / 50.0;
    let ui = Padding::all(
        gap,
        Grid::with_gap(
            4,
            4,
            gap,
            screenshots
                .iter()
                .map(|t| {
                    AspectRatio::new(
                        t.normalized_dimensions.x / t.normalized_dimensions.y,
                        ImageNode::from_texture(*t),
                    )
                })
                .collect::<Vec<_>>(),
        ),
    );

    draw_ui_in_area(ui, window_area());
}

async fn draw_pattern() {
    let mut hue = 0.0;
    let mut offset = vec2(0.0, 0.0);

    loop {
        hue += 20.0 * delta_time();
        hue %= 360.0;
        let color = Color::from_oklch(0.727, 0.1219, hue);
        let alt_color = color.darken_oklch(0.05);

        offset -= vec2(0.5, 0.25) * delta_time();

        draw_sdf(
            Sdf::rect_tl(vec2(0.0, 0.0), window_size())
                .with_fill(color, alt_color, FRAC_PI_2, 5.0, SdfFill::Truchet)
                .with_fill_offset(offset),
        );

        let r = min_window_dimension() / 5.0;

        draw_circle(window_center(), r, Color::BLACK);
        push_scissor(Area::new(vec2(window_center().x, 0.0), window_size()).to_rect());
        draw_circle(window_center(), r, Color::WHITE);
        pop_scissor();

        next_frame().await;
    }
}
