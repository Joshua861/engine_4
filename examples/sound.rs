use engine_4::prelude::*;
use tunes::{consts::*, prelude::*};

fn main() -> anyhow::Result<()> {
    init("Sound")?;

    let mut circle_size: f32 = 0.0;

    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5)
        .repeat(5);
    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .repeat(5);

    // realtime is probably what you want, otherwise the function blocks until the
    // audio is finished playing and does not open the window
    audio().play_mixer_realtime(&comp.into_mixer())?;

    loop {
        let input = get_input();

        if input.key_pressed(KeyCode::Space) {
            audio().play_sample("assets/sounds/vine-boom.mp3");
            circle_size += 300.0;
        }

        circle_size = (circle_size - 20.).clamp(0.0, 1000.0);

        if circle_size != 0.0 {
            draw_circle_with_outline_world(
                Vec2::ZERO,
                circle_size,
                Color::RED_500,
                Color::RED_500.darken(0.5),
                circle_size / 5.0,
            );
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
