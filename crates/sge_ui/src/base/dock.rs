use sge_api::shapes_2d::Orientation;
use sge_input::{cursor, mouse_pressed, mouse_released};
use sge_window::{use_ew_resize_cursor_icon, use_ns_resize_cursor_icon};

use super::*;

#[derive(Debug)]
pub struct Dock {
    a: Child,
    b: Child,
    border: BorderStyle,
    hovered_border: BorderStyle,
    orientation: Orientation,
    default_split: Split,
    state: State<DockState>,
}

#[derive(Clone, Copy, Debug)]
pub enum Split {
    FixedSizeA(f32),
    FixedSizeB(f32),
    Proportion(f32),
}

impl Split {
    pub fn split_area(&self, area: Area, orientation: Orientation) -> (Area, Area) {
        let (a, _) = self.sizes(orientation.main(area.size));
        area.split_at(a, orientation)
    }

    pub fn sizes(self, width: f32) -> (f32, f32) {
        match self {
            Self::FixedSizeA(size_a) => {
                if width > size_a {
                    (size_a, width - size_a)
                } else {
                    (width, 0.0)
                }
            }
            Self::FixedSizeB(size_b) => {
                if width > size_b {
                    (width - size_b, size_b)
                } else {
                    (0.0, width)
                }
            }
            Self::Proportion(proportion) => {
                debug_assert!((0.0..=1.0).contains(&proportion));
                let p_a = proportion;
                let p_b = 1.0 - proportion;
                (width * p_a, width * p_b)
            }
        }
    }
}

#[derive(Debug)]
pub struct DockState {
    split: Split,
    captured: bool,
}

impl Dock {
    pub fn new(
        border: BorderStyle,
        hovered_border: BorderStyle,
        orientation: Orientation,
        default_split: Split,
        id: usize,
        a: Child,
        b: Child,
    ) -> UiRef {
        Self {
            a,
            b,
            border,
            hovered_border,
            orientation,
            default_split,
            state: State::from_id(id),
        }
        .to_ref()
    }
}

impl UiNode for Dock {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::INFINITY
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state.get_or_create_mut(|| DockState {
            split: self.default_split,
            captured: false,
        });

        if mouse_released(MouseButton::Left) {
            state.captured = false;
        }

        if state.captured {
            if let Some(cursor_pos) = cursor() {
                let total = self.orientation.main(area.size);
                let relative = self.orientation.main(cursor_pos - area.top_left);

                match state.split {
                    Split::FixedSizeA(_) => {
                        let size_a = relative.clamp(10.0, total - 10.0);
                        state.split = Split::FixedSizeA(size_a);
                    }
                    Split::FixedSizeB(_) => {
                        let size_b = (total - relative).clamp(10.0, total - 10.0);
                        state.split = Split::FixedSizeB(size_b);
                    }
                    Split::Proportion(_) => {
                        let proportion = (relative / total).clamp(0.05, 0.95);
                        state.split = Split::Proportion(proportion);
                    }
                }
            }
        }

        let (area_a, area_b) = state.split.split_area(area, self.orientation);

        let border_thickness = self.border.thickness.max(10.0);
        let border_area = match self.orientation {
            Orientation::Horizontal => Area {
                top_left: Vec2::new(area_a.right() - border_thickness / 2.0, area.top_left.y),
                size: Vec2::new(border_thickness, area.size.y),
            },
            Orientation::Vertical => Area {
                top_left: Vec2::new(area.top_left.x, area_a.bottom() - border_thickness / 2.0),
                size: Vec2::new(area.size.x, border_thickness),
            },
        };

        let is_hovered = ui.is_hovered(border_area);

        if is_hovered || state.captured {
            match self.orientation {
                Orientation::Horizontal => {
                    use_ew_resize_cursor_icon();
                }
                Orientation::Vertical => {
                    use_ns_resize_cursor_icon();
                }
            }
        }

        if mouse_pressed(MouseButton::Left) && is_hovered {
            state.captured = true;
        }

        let border = if is_hovered || state.captured {
            &self.hovered_border
        } else {
            &self.border
        };

        match self.orientation {
            Orientation::Horizontal => {
                let x = area_a.right();
                let top = Vec2::new(x, area.top_left.y);
                let bottom = Vec2::new(x, area.bottom_right().y);
                border.draw(top, bottom, Side::Right);
            }
            Orientation::Vertical => {
                let y = area_a.bottom();
                let left = Vec2::new(area.top_left.x, y);
                let right = Vec2::new(area.bottom_right().x, y);
                border.draw(left, right, Side::Bottom);
            }
        }

        self.a.draw(area_a, ui);
        self.b.draw(area_b, ui);

        area.size
    }
}
