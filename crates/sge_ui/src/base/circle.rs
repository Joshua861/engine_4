use sge_api::Drawable;
use sge_shapes::d2::{Circle, CircleWithOutline};
use sge_vectors::vec2;

use super::*;

#[derive(Debug)]
pub struct CircleFill {
    color: Color,
}

impl CircleFill {
    pub fn new(color: Color) -> UiRef {
        Self { color }.to_ref()
    }
}

impl UiNode for CircleFill {
    fn preferred_dimensions(&self) -> Vec2 {
        vec2(50.0, 50.0)
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        Circle::from_top_left(area.top_left, area.size / 2.0, self.color).draw();

        area.size
    }
}

#[derive(Debug)]
pub struct CircleOutlineFill {
    outline_color: Color,
    fill_color: Color,
    thickness: f32,
}

impl CircleOutlineFill {
    pub fn new(outline_color: Color, fill_color: Color, thickness: f32) -> UiRef {
        Self {
            outline_color,
            fill_color,
            thickness,
        }
        .to_ref()
    }
}

impl UiNode for CircleOutlineFill {
    fn preferred_dimensions(&self) -> Vec2 {
        vec2(50.0, 50.0)
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        CircleWithOutline::from_top_left(
            area.top_left,
            area.size / 2.0,
            self.outline_color,
            self.thickness,
            self.fill_color,
        )
        .draw();

        area.size
    }
}
