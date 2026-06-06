use sge::prelude::*;

struct Background {
    round: f32,
    corner_size: u32,
    texture: TextureRef,
    round_offset: f32,
}

#[main("Nine slice")]
fn main() {
    let mut resize_method = ResizeMethod::Tile;
    let mut round = true;
    let mut n = 0;

    let textures = [
        Background {
            round: 20.0,
            corner_size: 3,
            texture: include_texture!("../assets/textures/nine_slice_example.png"),
            round_offset: 10.0,
        },
        Background {
            round: 10.0,
            corner_size: 8,
            texture: include_texture!("../assets/textures/nine_slice_example_2.png"),
            round_offset: 0.0,
        },
    ];

    loop {
        if key_pressed(KeyCode::Space) {
            resize_method = match resize_method {
                ResizeMethod::Tile => ResizeMethod::Stretch,
                ResizeMethod::Stretch => ResizeMethod::Tile,
            }
        }

        if key_pressed(KeyCode::KeyR) {
            round = !round;
        }

        if key_pressed(KeyCode::KeyN) {
            n = (n + 1) % textures.len();
        }

        let size = if round {
            (last_cursor_pos() / textures[n].round).round() * textures[n].round
                + textures[n].round_offset
        } else {
            last_cursor_pos()
        } - Vec2::splat(50.0);
        let size = size.max(Vec2::splat(textures[n].corner_size as f32 * 2.0) * 10.0);

        draw_nine_slice(
            textures[n].texture,
            Vec2::splat(50.0),
            size,
            Vec2::splat(10.0),
            textures[n].corner_size,
            resize_method,
        );

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
