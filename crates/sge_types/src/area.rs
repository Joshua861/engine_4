use sge_vectors::{Vec2, vec2};
use sge_window::window_size;

use crate::Orientation;

#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub top_left: Vec2,
    pub size: Vec2,
}

impl Area {
    pub const ZERO: Self = Self {
        top_left: Vec2::ZERO,
        size: Vec2::ZERO,
    };

    pub fn new(top_left: Vec2, size: Vec2) -> Self {
        Self { top_left, size }
    }

    pub fn from_glium_rect(rect: glium::Rect) -> Self {
        rect.into()
    }

    pub fn to_glium_rect(self) -> glium::Rect {
        let window_size = window_size();

        let bottom_y = window_size.y - (self.top_left.y + self.size.y);

        glium::Rect {
            left: self.top_left.x.round() as u32,
            bottom: bottom_y.round() as u32,
            width: self.size.x.round() as u32,
            height: self.size.y.round() as u32,
        }
    }

    pub fn square(self) -> Self {
        let size = self.size.min_element();
        Self {
            top_left: self.top_left,
            size: Vec2::splat(size),
        }
    }

    pub fn half_size(&self) -> Vec2 {
        self.size / 2.0
    }

    pub fn resize(&self, new_size: Vec2) -> Self {
        Self {
            top_left: self.top_left,
            size: new_size,
        }
    }

    /// create area that encompasses this and another
    pub fn merge(&self, other: Self) -> Self {
        let left = self.left().min(other.left());
        let right = self.right().max(other.right());
        let top = self.top().min(other.top());
        let bottom = self.bottom().max(other.bottom());

        Self {
            top_left: Vec2::new(left, top),
            size: Vec2::new(right - left, bottom - top),
        }
    }

    pub fn top(&self) -> f32 {
        self.top_left.y
    }

    pub fn bottom(&self) -> f32 {
        self.top_left.y + self.size.y
    }

    pub fn left(&self) -> f32 {
        self.top_left.x
    }

    pub fn right(&self) -> f32 {
        self.top_left.x + self.size.x
    }

    pub fn top_left(&self) -> Vec2 {
        self.top_left
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.top_left + self.size
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.top_left.x, self.top_left.y + self.size.y)
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.top_left.x + self.size.x, self.top_left.y)
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn shrink(self, amount: f32) -> Self {
        Self {
            top_left: self.top_left + Vec2::splat(amount),
            size: self.size - Vec2::splat(amount * 2.0),
        }
    }

    pub fn shrink_vec2(self, amount: Vec2) -> Self {
        Self {
            top_left: self.top_left + amount,
            size: self.size - amount * 2.0,
        }
    }

    pub fn with_padding(self, padding: f32) -> Self {
        Self {
            top_left: self.top_left + Vec2::splat(padding),
            size: self.size - Vec2::splat(padding * 2.0),
        }
    }

    pub fn with_left_padding(self, padding: f32) -> Self {
        Self {
            top_left: self.top_left + Vec2::new(padding, 0.0),
            size: self.size - Vec2::new(padding, 0.0),
        }
    }

    pub fn with_right_padding(self, padding: f32) -> Self {
        Self {
            top_left: self.top_left,
            size: self.size - Vec2::new(padding, 0.0),
        }
    }

    pub fn with_top_padding(self, padding: f32) -> Self {
        Self {
            top_left: self.top_left + Vec2::new(0.0, padding),
            size: self.size - Vec2::new(0.0, padding),
        }
    }

    pub fn with_bottom_padding(self, padding: f32) -> Self {
        Self {
            top_left: self.top_left,
            size: self.size - Vec2::new(0.0, padding),
        }
    }

    pub fn split_at(self, split: f32, orientation: Orientation) -> (Self, Self) {
        match orientation {
            Orientation::Horizontal => {
                let left = Self {
                    top_left: self.top_left,
                    size: Vec2::new(split, self.size.y),
                };
                let right = Self {
                    top_left: Vec2::new(self.top_left.x + split, self.top_left.y),
                    size: Vec2::new(self.size.x - split, self.size.y),
                };
                (left, right)
            }
            Orientation::Vertical => {
                let top = Self {
                    top_left: self.top_left,
                    size: Vec2::new(self.size.x, split),
                };
                let bottom = Self {
                    top_left: Vec2::new(self.top_left.x, self.top_left.y + split),
                    size: Vec2::new(self.size.x, self.size.y - split),
                };
                (top, bottom)
            }
        }
    }

    #[inline]
    pub fn min(self) -> Vec2 {
        self.top_left
    }

    #[inline]
    pub fn max(self) -> Vec2 {
        self.bottom_right()
    }

    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self {
            top_left: min,
            size: max - min,
        }
    }

    pub fn from_corners(a: Vec2, b: Vec2) -> Self {
        Self::from_min_max(a.min(b), a.max(b))
    }

    pub fn from_rect(rect: sge_vectors::Rect) -> Self {
        Self::from_min_max(rect.min, rect.max)
    }

    pub fn to_rect(self) -> sge_vectors::Rect {
        self.into()
    }
}

pub fn window_area() -> Area {
    Area {
        top_left: Vec2::ZERO,
        size: window_size(),
    }
}

impl From<sge_vectors::Rect> for Area {
    fn from(value: sge_vectors::Rect) -> Self {
        Self::from_rect(value)
    }
}

impl From<Area> for sge_vectors::Rect {
    fn from(value: Area) -> Self {
        Self {
            min: value.min(),
            max: value.max(),
        }
    }
}

impl From<glium::Rect> for Area {
    fn from(value: glium::Rect) -> Self {
        let bl = vec2(value.bottom as f32, value.left as f32);
        let size = vec2(value.width as f32, value.height as f32);
        let tl = bl - vec2(0.0, size.y);
        Self::new(tl, size)
    }
}

impl From<Area> for glium::Rect {
    fn from(value: Area) -> Self {
        value.to_glium_rect()
    }
}
