use core::f32;

use sge_vectors::vec2;

use super::*;

/// be careful with what order you place sized boxes and other nodes
///
/// for example, always place a border node inside a sized box and not the other way around
#[derive(Debug)]
pub struct SizedBox {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub child: Child,
}

impl SizedBox {
    pub fn new(dimensions: Vec2, child: Child) -> UiRef {
        Self {
            x: Some(dimensions.x),
            y: Some(dimensions.y),
            child,
        }
        .to_ref()
    }

    pub fn zero(child: Child) -> UiRef {
        Self::new(Vec2::ZERO, child)
    }

    pub fn square(size: f32, child: Child) -> UiRef {
        Self::new(Vec2::splat(size), child)
    }

    pub fn wh(width: f32, height: f32, child: Child) -> UiRef {
        Self {
            x: Some(width),
            y: Some(height),
            child,
        }
        .to_ref()
    }

    pub fn grow(child: Child) -> UiRef {
        Self {
            x: Some(f32::INFINITY),
            y: Some(f32::INFINITY),
            child,
        }
        .to_ref()
    }

    pub fn width(width: f32, child: Child) -> UiRef {
        Self {
            x: Some(width),
            y: None,
            child,
        }
        .to_ref()
    }

    pub fn height(height: f32, child: Child) -> UiRef {
        Self {
            x: None,
            y: Some(height),
            child,
        }
        .to_ref()
    }
}

impl UiRef {
    pub fn sized_wh(self, width: f32, height: f32) -> UiRef {
        SizedBox::wh(width, height, self)
    }

    pub fn square(self, size: f32) -> UiRef {
        SizedBox::wh(size, size, self)
    }

    pub fn sized(self, dimensions: Vec2) -> UiRef {
        SizedBox::new(dimensions, self)
    }

    pub fn height_infinite_width(self, height: f32) -> UiRef {
        SizedBox::height(height, self)
    }

    pub fn expand_x(self) -> UiRef {
        SizedBox::width(f32::INFINITY, self)
    }

    pub fn expand_y(self) -> UiRef {
        SizedBox::height(f32::INFINITY, self)
    }

    pub fn width_infinite_height(self, width: f32) -> UiRef {
        SizedBox::width(width, self)
    }

    pub fn width(self, width: f32) -> UiRef {
        SizedBox::width(width, self)
    }

    pub fn height(self, height: f32) -> UiRef {
        SizedBox::height(height, self)
    }
}

impl UiNode for SizedBox {
    fn preferred_dimensions(&self) -> Vec2 {
        let area = vec2(
            self.x.unwrap_or(f32::INFINITY),
            self.y.unwrap_or(f32::INFINITY),
        );
        let mut size = self.child.size(Area::new(Vec2::ZERO, area));

        if let Some(x) = self.x {
            size.x = x;
        }

        if let Some(y) = self.y {
            size.y = y;
        }

        size
    }

    fn size(&self, area: Area) -> Vec2 {
        Vec2::new(
            match self.x {
                Some(x) if x.is_finite() => x,
                Some(_) | None => area.width(),
            },
            match self.y {
                Some(y) if y.is_finite() => y,
                Some(_) | None => area.height(),
            },
        )
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        area.size = self.size(area);
        let dimensions = self.child.node.draw(area, state);
        Vec2::new(
            match self.x {
                Some(x) if x.is_finite() => x,
                Some(_) | None => dimensions.x,
            },
            match self.y {
                Some(y) if y.is_finite() => y,
                Some(_) | None => dimensions.y,
            },
        )
    }
}

#[derive(Debug)]
pub struct ConstrainedBox {
    max_size: Vec2,
    min_size: Vec2,
    child: Child,
}

impl ConstrainedBox {
    pub fn max_height(max_height: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, max_height),
            min_size: Vec2::ZERO,
            child,
        }
        .to_ref()
    }

    pub fn max_width(max_width: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(max_width, f32::INFINITY),
            min_size: Vec2::ZERO,
            child,
        }
        .to_ref()
    }

    pub fn min_height(min_height: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(0.0, min_height),
            child,
        }
        .to_ref()
    }

    pub fn min_width(min_width: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(min_width, 0.0),
            child,
        }
        .to_ref()
    }

    pub fn min_size(min_size: Vec2, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size,
            child,
        }
        .to_ref()
    }

    pub fn max_size(max_size: Vec2, child: Child) -> UiRef {
        Self {
            max_size,
            min_size: Vec2::ZERO,
            child,
        }
        .to_ref()
    }

    pub fn new(min_size: Vec2, max_size: Vec2, child: Child) -> UiRef {
        Self {
            max_size,
            min_size,
            child,
        }
        .to_ref()
    }

    fn transform_size(&self, size: Vec2) -> Vec2 {
        size.clamp(self.min_size, self.max_size)
    }

    pub fn grow_x(child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(f32::INFINITY, 0.0),
            child,
        }
        .to_ref()
    }

    pub fn grow_y(child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(0.0, f32::INFINITY),
            child,
        }
        .to_ref()
    }
}

impl UiNode for ConstrainedBox {
    fn preferred_dimensions(&self) -> Vec2 {
        self.transform_size(self.child.node.preferred_dimensions())
    }

    fn size(&self, area: Area) -> Vec2 {
        self.transform_size(self.child.node.size(area))
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        area.size = self.transform_size(area.size);
        self.transform_size(self.child.node.draw(area, state))
    }
}

impl UiRef {
    pub fn min_height(self, min_height: f32) -> UiRef {
        ConstrainedBox::min_height(min_height, self)
    }

    pub fn min_width(self, min_width: f32) -> UiRef {
        ConstrainedBox::min_width(min_width, self)
    }

    pub fn max_height(self, max_height: f32) -> UiRef {
        ConstrainedBox::max_height(max_height, self)
    }

    pub fn max_width(self, max_width: f32) -> UiRef {
        ConstrainedBox::max_width(max_width, self)
    }

    pub fn min_size(self, min_size: Vec2) -> UiRef {
        ConstrainedBox::min_size(min_size, self)
    }

    pub fn max_size(self, max_size: Vec2) -> UiRef {
        ConstrainedBox::max_size(max_size, self)
    }

    pub fn grow_x(self) -> UiRef {
        ConstrainedBox::grow_x(self)
    }

    pub fn grow_y(self) -> UiRef {
        ConstrainedBox::grow_y(self)
    }
}

#[derive(Debug)]
pub struct EmptyBox {
    height: f32,
    width: f32,
}

impl EmptyBox {
    pub fn new(width: f32, height: f32) -> UiRef {
        Self { width, height }.to_ref()
    }

    pub fn height(height: f32) -> UiRef {
        Self { width: 0.0, height }.to_ref()
    }

    pub fn width(width: f32) -> UiRef {
        Self { width, height: 0.0 }.to_ref()
    }
}

impl UiNode for EmptyBox {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    fn size(&self, area: Area) -> Vec2 {
        self.preferred_dimensions().max(area.size)
    }

    fn draw(&self, _: Area, _: &UiState) -> Vec2 {
        self.preferred_dimensions()
    }
}
