use sge_vectors::Vec2;
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

    pub fn to_rect(self) -> glium::Rect {
        let window_size = window_size();

        let bottom_y = window_size.y - (self.top_left.y + self.size.y);

        glium::Rect {
            left: self.top_left.x as u32,
            bottom: bottom_y as u32,
            width: self.size.x as u32,
            height: self.size.y as u32,
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
}

pub fn window_area() -> Area {
    Area {
        top_left: Vec2::ZERO,
        size: window_size(),
    }
}
