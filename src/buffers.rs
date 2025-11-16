use glium::{IndexBuffer, VertexBuffer};

use crate::{
    EngineDisplay,
    shapes_2d::QUAD_INDICES,
    textures::{TexturedVertex, UNIT_SQUARE},
};

pub struct Buffers {
    pub unit_square_tex: VertexBuffer<TexturedVertex>,
    pub unit_indices_tex: IndexBuffer<u32>,
}

impl Buffers {
    pub(crate) fn new(facade: &EngineDisplay) -> anyhow::Result<Self> {
        Ok(Self {
            unit_square_tex: VertexBuffer::new(facade, &UNIT_SQUARE)?,
            unit_indices_tex: IndexBuffer::new(
                facade,
                glium::index::PrimitiveType::TrianglesList,
                &QUAD_INDICES,
            )?,
        })
    }
}
