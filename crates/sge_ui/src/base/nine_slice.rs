use sge_rendering::api::draw_nine_slice;
use sge_types::ResizeMethod;

use super::*;

#[derive(Debug)]
pub struct NineSliceFill {
    texture: TextureRef,
    scale: Vec2,
    corner_size: u32,
    resize_method: ResizeMethod,
    child: Child,
}

impl NineSliceFill {
    pub fn new(
        texture: TextureRef,
        scale: Vec2,
        corner_size: u32,
        resize_method: ResizeMethod,
        child: Child,
    ) -> UiRef {
        let padding = corner_size as f32 * scale;
        let child = Padding::xy(padding.x, padding.y, child);
        Self {
            texture,
            scale,
            corner_size,
            resize_method,
            child,
        }
        .to_ref()
    }
}

impl UiNode for NineSliceFill {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.size(area)
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        draw_nine_slice(
            self.texture,
            area.top_left,
            area.size,
            self.scale,
            self.corner_size,
            self.resize_method,
        );

        self.child.draw(area, ui);

        area.size
    }
}
