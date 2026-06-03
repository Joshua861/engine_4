use graph_networks::*;
use sge::prelude::*;

#[main("Graphs")]
async fn main() -> AResult<()> {
    let mut pan = PanningCameraController::new();

    let mut network = Network::new();
    network.use_expensive_algorithms = true;
    init_network(&mut network);

    let mut path = vec![];

    loop {
        pan.update();
        network.update(true);

        network.calc_positions_by_force(100.0, 10);

        if key_pressed(KeyCode::KeyR) {
            network.clear();
            init_network(&mut network);
        }

        if let Some(hovered) = network.hovered() {
            path = network.find_path(hovered, network.nth_node(1).unwrap());
        } else {
            path = vec![];
        }

        draw_text(format!("Path: {:?}", path), vec2(10.0, 10.0));

        let mut hl_lines = vec![];

        for line in network.iter_connection_lines() {
            let on_path = path
                .windows(2)
                .any(|w| w[0] == line.start_id && w[1] == line.end_id);

            if on_path {
                hl_lines.push(line);
            }

            let color = Color::NEUTRAL_600;
            let dir = (line.end - line.start).normalize();
            draw_solid_arrow_world(
                line.start,
                line.end - dir * network.node_radius(),
                2.0,
                color,
            );
        }

        for line in hl_lines {
            let dir = (line.end - line.start).normalize();
            draw_solid_arrow_world(
                line.start,
                line.end - dir * network.node_radius(),
                4.0,
                Color::YELLOW_500,
            );
        }

        for node in network.iter_node_positions() {
            let color = if path.contains(&node.id) {
                Color::YELLOW_500
            } else {
                Color::NEUTRAL_600
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
    ]);
}
