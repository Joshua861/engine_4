use sge::*;

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
    let tile_size = Vec2::splat(100.0);

    loop {
        hue += 20.0 * delta_time();
        hue %= 360.0;
        let color = Color::from_oklch(0.727, 0.1219, hue);
        let alt_color = color.darken_oklch(0.05);

        offset += vec2(100.0, 50.0) * delta_time();
        offset %= tile_size * 2.0;

        let width = (window_width() / tile_size.x).ceil() as isize;
        let height = (window_height() / tile_size.y).ceil() as isize;

        for x in -2..width {
            for y in -2..height {
                let pos = Vec2::new(x as f32 * tile_size.x, y as f32 * tile_size.y) + offset;
                let color = if (x + y) % 2 == 0 { color } else { alt_color };

                draw_rect(pos, tile_size, color);
            }
        }

        let r = min_window_dimension() / 5.0;
        draw_circle(window_center(), r, Color::BLACK);
        push_scissor(Area::new(vec2(window_center().x, 0.0), window_size()).to_rect());
        draw_circle(window_center(), r, Color::WHITE);
        pop_scissor();

        next_frame().await;
    }
}
