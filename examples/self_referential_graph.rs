use graph_networks::*;
use sge::prelude::*;

#[main("Self referential graphs")]
fn main() {
    let mut network = Network::new();
    network.insert_nodes_with_links(&[&[0, 2], &[0, 3], &[1, 3], &[4], &[1]]);

    let mut controller = PanningCameraController::new();

    loop {
        clear_screen(Color::NEUTRAL_100);

        controller.update();

        network.update(true);
        network.calc_positions_by_force(300.0, 10);

        for line in network.iter_connection_lines() {
            if line.start_id == line.end_id {
                let p = line.start - vec2(90.0, 97.5);
                draw_cubic_bezier_world(
                    line.start,
                    line.start - vec2(250.0, 0.0),
                    line.start - vec2(0.0, 250.0),
                    line.start,
                    Color::NEUTRAL_500,
                    5.0,
                );
                draw_tri_world(
                    p,
                    p + vec2(5.0, -15.0),
                    p + vec2(15.0, 0.0),
                    Color::NEUTRAL_500,
                );
            } else {
                draw_solid_arrow_middle_world(line.start, line.end, 5.0, Color::NEUTRAL_500);
            }
        }

        for node in network.iter_node_positions() {
            let sdf = Sdf::circle(node.pos, 50.0)
                .with_fill(
                    Color::NEUTRAL_100,
                    Color::NEUTRAL_200,
                    FRAC_PI_4,
                    5.0,
                    SdfFill::Lines,
                )
                .with_shadow(vec2(3.0, 3.0), 2.0, Color::NEUTRAL_400)
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
