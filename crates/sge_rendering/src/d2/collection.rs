use crate::api::{draw_collection, draw_collection_to, draw_collection_world};

use super::*;

#[derive(Clone)]
pub struct Collection2D {
    pub(crate) draws: Vec<DrawCommand>,
}

impl Collection2D {
    pub fn empty() -> Self {
        Self { draws: vec![] }
    }

    pub fn clear(&mut self) {
        self.draws.clear();
    }

    pub fn renderer(&mut self) -> Renderer2D {
        Renderer2D {
            draws: &mut self.draws as *mut Vec<DrawCommand>,
            ty: RendererType::Collection,
        }
    }

    pub fn draw(&self) {
        draw_collection(self);
    }

    pub fn draw_world(&self) {
        draw_collection_world(self);
    }

    pub fn draw_to(&self, renderer: Renderer2D) {
        draw_collection_to(self, renderer);
    }
}
