use sge::*;
use sge_utils::ConstantArray;

// cannot exceed 32
const N: usize = 64;

#[main("Metaballs")]
async fn main() -> anyhow::Result<()> {
    let mut balls = Metaballs::new()?;
    let mut velocities: ConstantArray<Vec2, N> = ConstantArray::new();

    let mut cool_stuff = false;

    for i in 0..N {
        velocities[i] = vec2(rand_range(-10.0..10.0), rand_range(-10.0..10.0));
        balls.add_metaball(Metaball::new(vec2(0.0, 0.0), 100.0))?;
    }

    balls.set_color(Color::NEUTRAL_300);

    loop {
        clear_screen(Color::NEUTRAL_900);

        balls.draw_world();

        if key_pressed(KeyCode::Space) {
            cool_stuff = !cool_stuff;
        }

        // random bullshit just wiggle them around and stuff
        for i in 0..N {
            velocities[i][0] += rand_f32();
            velocities[i][0] *= 0.99;
            velocities[i][1] += rand_f32();
            velocities[i][1] *= 0.99;
            balls[i].center[0] += velocities[i][0];
            balls[i].center[1] += velocities[i][1];

            let window_size = window_size() / 1.5;
            let c = balls[i].center;
            let x = c[0];
            let y = c[1];

            if x > window_size.x {
                balls[i].center[0] = -window_size.x;
            } else if x < -window_size.x {
                balls[i].center[0] = window_size.x;
            }

            if y > window_size.y {
                balls[i].center[1] = -window_size.y;
            } else if y < -window_size.y {
                balls[i].center[1] = window_size.y;
            }
        }

        if cool_stuff {
            for _ in 0..3 {
                chromatic_abberation_screen(50.0);
                hue_rotate_screen(180.0);
            }
        }
        draw_fps_bg();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
