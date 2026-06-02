use sge::prelude::{
    ui::{Center, SizedBox, Text, draw_ui},
    *,
};

#[derive(Default)]
struct State {
    guy: LoadingTexture,
    space: LoadingTexture,
    pasta: LoadingTexture,
}

#[main("Async asset loading")]
async fn main() {
    storage_init_state(State::default());

    start_coroutine(async {
        let state = storage_get_state_mut::<State>();

        // join! lets you run all 3 simultaniously
        join!(
            async { state.guy = Some(load_texture("assets/textures/guy.jpg").await) },
            async { state.space = Some(load_texture("assets/textures/space.jpg").await) },
            async { state.pasta = Some(load_texture("assets/textures/pasta.jpg").await) },
        );
    });

    loop {
        clear_screen(Color::NEUTRAL_200);
        let state = storage_get_state_mut::<State>();

        draw_loading_texture(&state.guy, vec2(20.0, 20.0), vec2(600.0, 400.0));
        draw_loading_texture(&state.space, vec2(20.0, 440.0), vec2(600.0, 400.0));
        draw_loading_texture(&state.pasta, vec2(640.0, 20.0), vec2(600.0, 820.0));

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}

fn draw_loading_texture(loading: &LoadingTexture, pos: Vec2, scale: Vec2) {
    match loading {
        Some(Ok(tex)) => draw_texture_scaled(*tex, pos, scale),
        Some(Err(e)) => draw_box(format!("{:#?}", e), pos, scale),
        None => draw_box("Loading...", pos, scale),
    }
}

fn draw_box(text: impl ToString, pos: Vec2, scale: Vec2) {
    draw_ui(
        SizedBox::new(scale, Center::new(Text::new(text)).fill(Color::NEUTRAL_800)),
        pos,
    );
}
