use std::f32::consts::TAU;

use sge_api::shapes_2d::draw_ellipse_sector_outline;
use sge_time::oscillate;

use super::*;

#[derive(Debug)]
pub struct LoadingSpinner(Color);

impl LoadingSpinner {
    pub fn new(color: Color) -> UiRef {
        Self(color).to_ref()
    }
}

impl UiNode for LoadingSpinner {
    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        let area = area.square();
        let thickness = area.size.x / 10.0;

        let a = (time() * 7.0) % TAU;
        let b = a + oscillate(0.5, TAU - 0.5);

        draw_ellipse_sector_outline(
            area.center(),
            area.size / 2.0 - thickness,
            a,
            b,
            self.0,
            thickness,
        );

        area.size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::new(50.0, 50.0)
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }
}
