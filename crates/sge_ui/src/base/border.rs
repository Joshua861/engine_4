use sge_api::shapes_2d::{draw_dashed_line, draw_quad, draw_rect};
use sge_vectors::vec2;

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct BorderStyle {
    pub thickness: f32,
    pub color: Color,
    pub ty: BorderType,
}

#[derive(Clone, Copy, Debug)]
pub enum BorderType {
    Solid,
    Dashed(f32),
    Dotted,
    Rectangle,
}

impl BorderStyle {}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::none()
    }
}

#[derive(Debug)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl BorderStyle {
    pub fn new(thickness: f32, color: Color) -> Self {
        Self {
            thickness,
            color,
            ty: BorderType::Solid,
        }
    }

    pub const fn none() -> Self {
        Self {
            thickness: 0.0,
            color: Color::TRANSPARENT,
            ty: BorderType::Solid,
        }
    }

    pub fn custom(thickness: f32, color: Color, ty: BorderType) -> Self {
        Self {
            thickness,
            color,
            ty,
        }
    }

    pub fn with_type(mut self, ty: BorderType) -> Self {
        self.ty = ty;
        self
    }

    pub fn draw(&self, a: Vec2, b: Vec2, side: Side) {
        match self.ty {
            BorderType::Solid => {
                let ((x_a, y_a), (x_b, y_b)) = match side {
                    Side::Top => ((1, 1), (-1, 1)),
                    Side::Left => ((1, 1), (1, -1)),
                    Side::Bottom => ((1, -1), (-1, -1)),
                    Side::Right => ((-1, 1), (-1, -1)),
                };

                let mul_a = Vec2::new(x_a as f32, y_a as f32);
                let mul_b = Vec2::new(x_b as f32, y_b as f32);

                let c = a + (self.thickness * mul_a);
                let d = b + (self.thickness * mul_b);

                draw_quad([a, c, d, b], self.color);
            }
            BorderType::Rectangle => {
                let (a, b) = match side {
                    Side::Top => (a, b + Vec2::new(0.0, self.thickness)),
                    Side::Left => (a, b + Vec2::new(self.thickness, 0.0)),
                    Side::Bottom => (a - vec2(0.0, self.thickness), b),
                    Side::Right => (a - vec2(self.thickness, 0.0), b),
                };

                draw_rect(a, b - a, self.color);
            }
            BorderType::Dashed(_) | BorderType::Dotted => {
                let half_thickness = self.thickness / 2.0;
                let (point_a, point_b) = match side {
                    Side::Top => (
                        a + Vec2::new(0.0, half_thickness),
                        b + Vec2::new(0.0, half_thickness),
                    ),
                    Side::Left => (
                        a + Vec2::new(half_thickness, 0.0),
                        b + Vec2::new(half_thickness, 0.0),
                    ),
                    Side::Bottom => (
                        a - Vec2::new(0.0, half_thickness),
                        b - Vec2::new(0.0, half_thickness),
                    ),
                    Side::Right => (
                        a - Vec2::new(half_thickness, 0.0),
                        b - Vec2::new(half_thickness, 0.0),
                    ),
                };

                let len = match self.ty {
                    BorderType::Dashed(len) => len,
                    BorderType::Dotted => self.thickness,
                    _ => unreachable!(),
                };

                draw_dashed_line(point_a, point_b, self.thickness, self.color, len);
            }
        }
    }

    pub const NONE: Self = Self::none();
}

#[derive(Debug)]
pub struct Border {
    top: BorderStyle,
    bottom: BorderStyle,
    left: BorderStyle,
    right: BorderStyle,

    child: Child,
}

impl Border {
    pub fn tblr(
        top: BorderStyle,
        bottom: BorderStyle,
        left: BorderStyle,
        right: BorderStyle,
        child: Child,
    ) -> UiRef {
        Self {
            top,
            bottom,
            left,
            right,
            child,
        }
        .to_ref()
    }

    pub fn top_bottom(top: BorderStyle, bottom: BorderStyle, child: Child) -> UiRef {
        Self {
            top,
            bottom,
            left: BorderStyle::NONE,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn left_right(left: BorderStyle, right: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: BorderStyle::NONE,
            left,
            right,
            child,
        }
        .to_ref()
    }

    pub fn top(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: style,
            bottom: BorderStyle::NONE,
            left: BorderStyle::NONE,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn bottom(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: style,
            left: BorderStyle::NONE,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn left(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: BorderStyle::NONE,
            left: style,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn right(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: BorderStyle::NONE,
            left: BorderStyle::NONE,
            right: style,
            child,
        }
        .to_ref()
    }

    pub fn all_style(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            child,
        }
        .to_ref()
    }

    pub fn all(thickness: f32, color: Color, child: Child) -> UiRef {
        let style = BorderStyle {
            thickness,
            color,
            ty: BorderType::Rectangle,
        };
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            child,
        }
        .to_ref()
    }

    fn extra_size(&self) -> Vec2 {
        Vec2::new(
            self.left.thickness + self.right.thickness,
            self.top.thickness + self.bottom.thickness,
        )
    }

    fn offset(&self) -> Vec2 {
        Vec2::new(self.left.thickness, self.top.thickness)
    }
}

impl UiNode for Border {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions() + self.extra_size()
    }

    fn size(&self, area: Area) -> Vec2 {
        (self.child.node.size(area) + self.extra_size()).min(area.size)
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        self.top.draw(area.top_left(), area.top_right(), Side::Top);
        self.left
            .draw(area.top_left(), area.bottom_left(), Side::Left);
        self.bottom
            .draw(area.bottom_left(), area.bottom_right(), Side::Bottom);
        self.right
            .draw(area.top_right(), area.bottom_right(), Side::Right);

        area.size -= self.extra_size();
        area.top_left += self.offset();

        let dimensions = self.child.node.draw(area, state);
        dimensions + self.extra_size()
    }
}

impl UiRef {
    pub fn border(self, color: Color, thickness: f32) -> Self {
        Border::all(thickness, color, self)
    }
}

#[derive(Debug)]
pub struct Outline {
    top: BorderStyle,
    bottom: BorderStyle,
    left: BorderStyle,
    right: BorderStyle,

    child: Child,
}

impl Outline {
    pub fn tblr(
        top: BorderStyle,
        bottom: BorderStyle,
        left: BorderStyle,
        right: BorderStyle,
        child: Child,
    ) -> UiRef {
        Self {
            top,
            bottom,
            left,
            right,
            child,
        }
        .to_ref()
    }

    pub fn top_bottom(top: BorderStyle, bottom: BorderStyle, child: Child) -> UiRef {
        Self {
            top,
            bottom,
            left: BorderStyle::NONE,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn left_right(left: BorderStyle, right: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: BorderStyle::NONE,
            left,
            right,
            child,
        }
        .to_ref()
    }

    pub fn top(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: style,
            bottom: BorderStyle::NONE,
            left: BorderStyle::NONE,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn bottom(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: style,
            left: BorderStyle::NONE,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn left(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: BorderStyle::NONE,
            left: style,
            right: BorderStyle::NONE,
            child,
        }
        .to_ref()
    }

    pub fn right(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: BorderStyle::NONE,
            bottom: BorderStyle::NONE,
            left: BorderStyle::NONE,
            right: style,
            child,
        }
        .to_ref()
    }

    pub fn all_style(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            child,
        }
        .to_ref()
    }

    pub fn all(thickness: f32, color: Color, child: Child) -> UiRef {
        let style = BorderStyle {
            thickness,
            color,
            ty: BorderType::Solid,
        };
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            child,
        }
        .to_ref()
    }
}

impl UiNode for Outline {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.node.size(area)
    }

    fn draw(&self, area: Area, state: &UiState) -> Vec2 {
        let dimensions = self.child.node.draw(area, state);

        self.top.draw(area.top_left(), area.top_right(), Side::Top);
        self.left
            .draw(area.top_left(), area.bottom_left(), Side::Left);
        self.bottom
            .draw(area.bottom_left(), area.bottom_right(), Side::Bottom);
        self.right
            .draw(area.top_right(), area.bottom_right(), Side::Right);

        dimensions
    }
}

impl UiRef {
    pub fn outline(self, color: Color, thickness: f32) -> Self {
        Outline::all(thickness, color, self)
    }
}
