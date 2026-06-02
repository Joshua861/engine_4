use std::{
    f32,
    fmt::{Debug, Display},
};

use nalgebra::{DMatrix, SymmetricEigen};
use sge_camera::screen_to_world;
use sge_color::Color;
use sge_input::last_cursor_pos;
use sge_rng::id;
use sge_vectors::Vec2;

pub struct Network {
    nodes: Vec<Node>,
    hovered: NodeId,
    node_radius: f32,
    pub use_expensive_algorithms: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(usize);

impl Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Node {
    id: NodeId,
    pos: Vec2,
    connections: Vec<NodeId>,
}

impl NodeId {
    pub fn get(self, network: &Network) -> &Node {
        network.get(self)
    }

    pub fn get_mut(self, network: &mut Network) -> &mut Node {
        network.get_mut(self)
    }

    pub fn add_connections(&self, network: &mut Network, connections: &[NodeId]) {
        network.add_connections(*self, connections);
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            hovered: NodeId(usize::MAX),
            node_radius: 20.0,
            use_expensive_algorithms: false,
        }
    }

    pub fn node_radius(&self) -> f32 {
        self.node_radius
    }

    pub fn set_node_radius(&mut self, radius: f32) {
        self.node_radius = radius;
    }

    pub fn with_node_radius(mut self, radius: f32) -> Self {
        self.set_node_radius(radius);
        self
    }

    pub fn hovered(&self) -> Option<NodeId> {
        if self.hovered.0 != usize::MAX {
            Some(self.hovered)
        } else {
            None
        }
    }

    pub fn calc_positions_by_force(
        &mut self,
        desired_connection_length: f32,
        iteration_count: usize,
    ) {
        let l = desired_connection_length;

        for _ in 0..iteration_count {
            let mut displacements = vec![Vec2::ZERO; self.nodes.len()];

            // push all nodes apart
            #[allow(clippy::needless_range_loop)]
            for i in 0..self.nodes.len() {
                for j in 0..self.nodes.len() {
                    if i == j {
                        continue;
                    }

                    let delta = self.nodes[i].pos - self.nodes[j].pos;
                    let dist = delta.length().max(0.01);

                    displacements[i] += (delta / dist) * (l / dist);
                }
            }

            // attract connected ndoes
            for (a, b) in self.iter_connections() {
                let delta = self.get(a).pos - self.get(b).pos;
                let dist = delta.length_squared().max(0.01);
                let force = (delta / dist) * (dist / l);
                displacements[a.0] -= force;
                displacements[b.0] += force;
            }

            // add slight attraction toward center, so they dont just fly off
            #[allow(clippy::needless_range_loop)]
            for i in 0..self.nodes.len() {
                let delta = self.nodes[i].pos;
                let dist = delta.length_squared().max(0.01);
                displacements[i] -= (delta / dist) * (dist * 0.008);
            }

            // take 3 points along graph connection line and add repulsion to all nodes that arent part of the connection
            if self.use_expensive_algorithms {
                for (a, b) in self.iter_connections() {
                    let a_pos = self.get(a).pos;
                    let b_pos = self.get(b).pos;

                    for t in [0.25, 0.5, 0.75] {
                        let point = a_pos * (1.0 - t) + b_pos * t;

                        #[allow(clippy::needless_range_loop)]
                        for i in 0..self.nodes.len() {
                            if self.nodes[i].id == a || self.nodes[i].id == b {
                                continue;
                            }

                            let delta = self.nodes[i].pos - point;
                            let dist = delta.length().max(0.01);
                            displacements[i] += (delta / dist) * (l / dist) * 0.16;
                        }
                    }
                }
            }

            let temp = l * 0.1;
            for node in self.nodes.iter_mut() {
                let d = displacements[node.id.0];
                let len = d.length().max(0.001);
                node.pos += (d / len) * len.min(temp);
            }
        }
    }

    pub fn calc_positions_laplace(&mut self, desired_connection_length: f32) {
        let n = self.nodes.len();

        if n < 2 {
            return;
        }

        let mut laplacian = DMatrix::<f32>::zeros(n, n);

        for node in &self.nodes {
            for &neighbor in &node.connections {
                let i = node.id.0;
                let j = neighbor.0;

                laplacian[(i, j)] -= 1.0;
                laplacian[(j, i)] -= 1.0;

                laplacian[(i, i)] += 1.0;
                laplacian[(j, j)] += 1.0;
            }
        }

        let eigen = SymmetricEigen::new(laplacian);

        let x_vec = eigen.eigenvectors.column(1);
        let y_vec = eigen.eigenvectors.column(2);

        let edge_count = self.iter_connections().count();
        let scale = if edge_count > 0 {
            let mean_eigen_dist: f32 = self
                .iter_connections()
                .map(|(a, b)| {
                    let dx = x_vec[a.0] - x_vec[b.0];
                    let dy = y_vec[a.0] - y_vec[b.0];
                    (dx * dx + dy * dy).sqrt()
                })
                .sum::<f32>()
                / edge_count as f32;

            if mean_eigen_dist > 1e-6 {
                desired_connection_length / mean_eigen_dist
            } else {
                1.0
            }
        } else {
            1.0
        };

        for node in &mut self.nodes {
            let i = node.id.0;
            node.pos = Vec2::new(x_vec[i] * scale, y_vec[i] * scale);
        }
    }

    fn next_id(&self) -> NodeId {
        NodeId(self.nodes.len())
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn insert_nodes_with_links(&mut self, nodes: &[&[usize]]) {
        let base_n = self.nodes.len();

        for connections in nodes {
            let connections = connections
                .iter()
                .map(|&id| NodeId(id + base_n))
                .collect::<Vec<_>>();
            self.insert(connections);
        }
    }

    pub fn randomize_positions(&mut self) {
        for node in &mut self.nodes {
            node.pos = sge_rng::rand_vec2() * 100.0;
        }
    }

    pub fn nth_node(&self, n: usize) -> Option<NodeId> {
        self.nodes.get(n).map(|node| &node.id).copied()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn insert(&mut self, connections: impl Into<Vec<NodeId>>) -> NodeId {
        let id = self.next_id();
        self.nodes.push(Node {
            id,
            pos: sge_rng::rand_vec2() * 100.0,
            connections: connections.into(),
        });
        id
    }

    pub fn get(&self, id: NodeId) -> &Node {
        &self.nodes[id.0]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0]
    }

    pub fn update(&mut self, world: bool) {
        let mut cursor = last_cursor_pos();
        if world {
            cursor = screen_to_world(cursor);
        }
        let radius_squared = self.node_radius * self.node_radius;
        for node in &self.nodes {
            let pos = node.pos;
            let delta = cursor - pos;
            let dist_squared = delta.length_squared();

            if dist_squared < radius_squared {
                self.hovered = node.id;
                return;
            }
        }

        self.hovered = NodeId(usize::MAX);
    }

    pub fn iter_connections<'a>(&'a self) -> ConnectionIterator<'a> {
        ConnectionIterator {
            network: self,
            node: 0,
            conn: 0,
        }
    }

    pub fn iter_node_positions<'a>(&'a self) -> NodePositionIterator<'a> {
        NodePositionIterator {
            network: self,
            node: 0,
        }
    }

    pub fn iter_connection_lines<'a>(&'a self) -> ConnectionLineIterator<'a> {
        ConnectionLineIterator {
            network: self,
            node: 0,
            conn: 0,
        }
    }

    pub fn add_connections(&mut self, id: NodeId, connections: &[NodeId]) {
        self.get_mut(id).connections.extend_from_slice(connections);
    }

    pub fn find_path(&self, a: NodeId, b: NodeId) -> Vec<NodeId> {
        let mut weights = vec![usize::MAX; self.nodes.len()];
        weights[a.0] = 0;

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(a);

        while let Some(current) = queue.pop_front() {
            if current == b {
                break;
            }
            let next_weight = weights[current.0] + 1;
            for &conn in &self.get(current).connections {
                if weights[conn.0] == usize::MAX {
                    weights[conn.0] = next_weight;
                    queue.push_back(conn);
                }
            }
        }

        self.trace_path(a, b, &weights)
    }

    fn trace_path(&self, a: NodeId, b: NodeId, weights: &[usize]) -> Vec<NodeId> {
        if weights[b.0] == usize::MAX {
            return vec![];
        }

        let mut path = vec![b];
        let mut current = b;

        while current != a {
            let prev = self.nodes.iter().find(|node| {
                node.connections.contains(&current) && weights[node.id.0] == weights[current.0] - 1
            });

            match prev {
                Some(node) => {
                    path.push(node.id);
                    current = node.id;
                }
                None => break,
            }
        }

        path.reverse();
        path
    }

    pub fn make_all_connections_bidirectional(&mut self) {
        let nodes = self.nodes.as_mut_ptr();
        let len = self.nodes.len();
        unsafe {
            for i in 0..len {
                let node = &*nodes.add(i);
                for j in 0..node.connections.len() {
                    let conn = node.connections[j];
                    let conn_node = &mut *nodes.add(conn.0);
                    if !conn_node.connections.contains(&node.id) {
                        conn_node.connections.push(node.id);
                    }
                }
            }
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConnectionIterator<'a> {
    network: &'a Network,
    node: usize,
    conn: usize,
}

impl<'a> Iterator for ConnectionIterator<'a> {
    type Item = (NodeId, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.network.nodes.get(self.node)?;

        if let Some(&conn) = node.connections.get(self.conn) {
            self.conn += 1;
            Some((node.id, conn))
        } else {
            self.node += 1;
            self.conn = 0;
            self.next()
        }
    }
}

pub struct NodePositionIterator<'a> {
    network: &'a Network,
    node: usize,
}

#[derive(Clone, Copy)]
pub struct NodePosition {
    pub pos: Vec2,
    pub n: usize,
    pub is_hovered: bool,
    pub id: NodeId,
}

impl<'a> Iterator for NodePositionIterator<'a> {
    type Item = NodePosition;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.network.nodes.get(self.node)?;

        self.node += 1;

        Some(NodePosition {
            pos: node.pos,
            n: node.id.0,
            is_hovered: node.id == self.network.hovered,
            id: node.id,
        })
    }
}

pub struct ConnectionLineIterator<'a> {
    network: &'a Network,
    node: usize,
    conn: usize,
}

#[derive(Clone, Copy)]
pub struct ConnectionLine {
    pub start: Vec2,
    pub start_id: NodeId,
    pub end: Vec2,
    pub end_id: NodeId,
    pub is_hovered: bool,
    pub color: Color,
}

impl<'a> Iterator for ConnectionLineIterator<'a> {
    type Item = ConnectionLine;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.network.nodes.get(self.node)?;

        if let Some(&conn) = node.connections.get(self.conn) {
            self.conn += 1;
            Some(ConnectionLine {
                start: node.pos,
                start_id: node.id,
                end: self.network.get(conn).pos,
                end_id: conn,
                is_hovered: node.id == self.network.hovered || conn == self.network.hovered,
                color: Color::from_usize_no_alpha(id!(conn.0, node.id.0)),
            })
        } else {
            self.node += 1;
            self.conn = 0;
            self.next()
        }
    }
}
