use graph_networks::*;
use sge::prelude::*;

#[main("Graphs")]
async fn main() -> AResult<()> {
    let mut pan = PanningCameraController::new();

    let mut network = Network::new();
    network.allow_dragging = true;
    init_network(&mut network);

    loop {
        if !network.update(true) {
            pan.update();
            network.calc_positions_by_force(100.0, 20);
        }

        if key_pressed(KeyCode::KeyR) {
            network.clear();
            init_network(&mut network);
        }

        if key_pressed(KeyCode::KeyA) {
            let len = network.len();
            let n = rand_range(0..len);
            let node = network.nth_node(n).unwrap();
            let c = rand_range(0..len);
            let c = network.nth_node(c).unwrap();
            node.add_connections(&mut network, &[c]);
        }

        for line in network.iter_connection_lines() {
            let alpha = if line.is_hovered { 1.0 } else { 0.8 };
            let dir = (line.end - line.start).normalize();
            draw_solid_arrow_world(
                line.start,
                line.end - dir * network.node_radius(),
                2.0,
                line.color.with_alpha(alpha),
            );
        }

        for node in network.iter_node_positions() {
            let color = if node.is_hovered {
                Color::WHITE
            } else {
                Color::NEUTRAL_400
            };
            draw_circle_world(node.pos, network.node_radius(), color);
            let dim = measure_text(node.n.to_string());
            draw_colored_text_world(node.n.to_string(), node.pos - dim.size / 2.0, Color::BLACK);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn init_network(network: &mut Network) {
    network.insert_nodes_with_links(&[
        &[1, 3],
        &[],
        &[2, 14],
        &[1],
        &[2, 3, 0],
        &[0],
        &[5],
        &[2],
        &[2],
        &[2],
        &[2],
        &[2, 4, 5],
        &[8, 11],
        &[12],
        &[9, 10, 4],
        &[7, 8, 12],
        &[13],
        &[1, 0, 5],
        &[9, 10],
        &[11, 12],
        &[5, 6, 11],
        &[16, 13, 6],
        &[6, 21, 16],
        &[12, 13],
        &[9, 2],
        &[18, 9, 7],
        &[12, 13, 23, 16],
        &[15],
        &[15],
        &[15],
        &[15],
        &[15],
        &[15],
        &[15],
        &[15],
        &[15],
        &[31],
        &[31],
        &[31],
        &[31],
        &[31],
        &[36],
        &[41, 36, 38, 39],
        &[38, 42, 41],
        &[37, 39, 36],
        &[],
        &[45],
        &[45],
        &[45],
        &[45],
        &[45],
        &[45],
        &[45],
        &[45],
        &[45],
        &[46, 59],
        &[47, 58],
        &[48, 62],
        &[49, 61],
        &[50, 60],
        &[51, 57],
        &[52, 55],
        &[53, 63],
        &[54, 56],
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
        &[64],
        &[64],
        &[64],
        &[64],
        &[64],
        &[64],
        &[64],
        &[70],
    ]);
}
