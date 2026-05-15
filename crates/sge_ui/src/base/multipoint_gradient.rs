use sge_api::shapes_2d::{GradientPoint, draw_multipoint_gradient};
use sge_types::Orientation;

use super::*;

#[derive(Debug)]
pub struct MultipointGradientFill {
    points: Vec<GradientPoint>,
    orientation: Orientation,
    child: Child,
}

impl MultipointGradientFill {
    pub fn vertical(points: Vec<GradientPoint>, child: Child) -> UiRef {
        Self {
            points,
            child,
            orientation: Orientation::Vertical,
        }
        .to_ref()
    }

    pub fn horizontal(points: Vec<GradientPoint>, child: Child) -> UiRef {
        Self {
            points,
            child,
            orientation: Orientation::Horizontal,
        }
        .to_ref()
    }
}

impl UiNode for MultipointGradientFill {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        draw_multipoint_gradient(
            area.top_left,
            area.size,
            self.points.clone(),
            self.orientation,
        );
        self.child.draw(area, ui);

        area.size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.size(area)
    }
}
