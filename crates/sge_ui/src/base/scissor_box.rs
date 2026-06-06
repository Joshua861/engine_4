use sge_rendering::scissor::{pop_scissor, push_scissor};

use super::*;

#[derive(Debug)]
pub struct ScissorBox {
    child: Child,
}

impl ScissorBox {
    pub fn new(child: Child) -> UiRef {
        ScissorBox { child }.to_ref()
    }
}

impl UiRef {
    pub fn scissored(self) -> UiRef {
        ScissorBox::new(self)
    }
}

impl UiNode for ScissorBox {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        push_scissor(area.to_glium_rect());

        let dimensions = self.child.node.draw(area, ui);

        pop_scissor();

        dimensions
    }
}
