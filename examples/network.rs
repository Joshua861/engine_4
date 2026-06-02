use sge::prelude::*;

struct State {
    response: Option<Result<String, net::Error>>,
}

#[main("Network")]
async fn main() {
    storage_init_state(State { response: None });

    start_coroutine(async {
        let text = net::get_text(
            "https://raw.githubusercontent.com/LilyRL/sge/refs/heads/master/README.org",
        )
        .await;

        // pretend the request takes longer so we can see it happening
        std::thread::sleep(Duration::from_millis(500));

        storage_get_state_mut::<State>().response = Some(text);
    });

    loop {
        clear_screen(Color::NEUTRAL_200);

        match &storage_get_state::<State>().response {
            Some(Ok(text)) => {
                draw_colored_text(text, vec2(20.0, 20.0), Color::BLACK);
            }
            Some(Err(e)) => {
                draw_colored_text(format!("Error: {:#?}", e), vec2(20.0, 20.0), Color::RED_400);
            }
            None => {
                draw_colored_text("Loading...", vec2(20.0, 20.0), Color::BLACK);
            }
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
