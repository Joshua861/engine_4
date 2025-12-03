use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Title for the window")?;

    let inter = load_font(include_bytes!("../assets/fonts/inter.ttf"))?;

    loop {
        inter.draw_text("Hello world, from Inter", Vec2::splat(100.0), 100);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
