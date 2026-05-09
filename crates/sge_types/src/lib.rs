pub use area::*;
pub mod area;

pub use rendering::*;
use sge_error_union::ErrorUnion;
pub mod rendering;

pub use vertex::*;
pub mod vertex;

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
