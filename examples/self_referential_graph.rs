use graph_networks::*;
use sge::prelude::*;

#[main("Self referential graphs")]
fn main() {
    let mut network = Network::new();
    network.insert_nodes_with_links(&[&[0, 2], &[0, 3], &[1, 3], &[4], &[1]]);
    network.allow_dragging = true;
    network.use_expensive_algorithms = true;
    network.node_radius = 50.0;

    let mut controller = PanningCameraController::new();

    loop {
        clear_screen(Color::NEUTRAL_100);

        if !network.update(true) {
            controller.update();
            network.calc_positions_by_force(300.0, 10);
        }

        for line in network.iter_connection_lines() {
            for n in [0.0, 0.5] {
                let t = (time() + n) % 1.0;
                if line.start_id == line.end_id {
                    draw_draw_cubic_bezier_arrow_t_world(
                        line.start,
                        line.start - vec2(250.0, 0.0),
                        line.start - vec2(0.0, 250.0),
                        line.start,
                        Color::NEUTRAL_500,
                        5.0,
                        t,
                    );
                } else {
                    draw_solid_arrow_t_world(line.start, line.end, 5.0, Color::NEUTRAL_500, t);
                }
            }
        }

        for node in network.iter_node_positions() {
            let sdf = Sdf::circle(node.pos, node.radius)
                .with_fill(
                    Color::WHITE,
                    Color::NEUTRAL_300,
                    0.0,
                    10.0,
                    SdfFill::RadialGradient,
                )
                .with_shadow(vec2(3.0, 3.0), 10.0, Color::NEUTRAL_900.with_alpha(0.4))
                .with_stroke(7.0, Color::NEUTRAL_800, SdfStroke::Inside);
            draw_sdf_world(sdf);

            let dim = measure_text_ex(
                node.n.to_string(),
                TextDrawParams::builder().font_size(30).build(),
            );
            draw_text_world_ex(node.n, node.pos - dim.size / 2.0, Color::BLACK, 30);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
