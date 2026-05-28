use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    let opts = EngineCreationOptions::builder()
        .title("Logging".to_string())
        .min_log_level(LevelFilter::max())
        .log_verbosity(Verbosity::Medium)
        .build();

    init_custom(opts)?;
    let mut controller = PanningCameraController::new();
    set_max_drawn_log_lines(50);

    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");
    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");
    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");
    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");

    run_async(async move {
        loop {
            controller.update();
            draw_logs();

            if once_per_n_seconds(4.0) {
                info!("im so alone");
            } else if once_per_n_seconds(2.0) {
                warn!("haha");
            } else if once_per_second() {
                error!("BAHAHAHA");
            } else if once_per_n_seconds(0.5) {
                debug!("BAHAHAHAHAHA");
            }

            let mut font = default_font();
            let texture = font.texture();
            draw_texture_world(texture, Vec2::ZERO, 1000.0);
            draw_square_outline_world(Vec2::ZERO, 1000.0, 2.0, Color::NEUTRAL_500);

            if should_quit() {
                break;
            }

            next_frame().await;
        }
    });

    Ok(())
}
