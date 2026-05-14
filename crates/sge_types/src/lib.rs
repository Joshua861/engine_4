pub use area::*;
pub mod area;

pub use rendering::*;
use sge_error_union::ErrorUnion;
pub mod rendering;

use sge_vectors::{Vec2, vec2};
pub use vertex::*;
pub mod vertex;

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Orientation {
    pub fn main(self, vec2: Vec2) -> f32 {
        match self {
            Orientation::Vertical => vec2.y,
            Orientation::Horizontal => vec2.x,
        }
    }

    pub fn cross(self, vec2: Vec2) -> f32 {
        match self {
            Orientation::Vertical => vec2.x,
            Orientation::Horizontal => vec2.y,
        }
    }

    pub fn create_vec2(self, main: f32, cross: f32) -> Vec2 {
        match self {
            Orientation::Vertical => vec2(cross, main),
            Orientation::Horizontal => vec2(main, cross),
        }
    }
}

pub enum Verbosity {
    Low,
    Medium,
    High,
}

#[derive(ErrorUnion, Debug)]
pub enum BufferError {
    Vertex(glium::vertex::BufferCreationError),
    Index(glium::index::BufferCreationError),
}
