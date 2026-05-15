use super::*;

#[derive(Debug)]
pub struct Center {
    vertical: bool,
    horizontal: bool,
    child: Child,
}

impl Center {
    pub fn new(child: Child) -> UiRef {
        Self {
            vertical: true,
            horizontal: true,
            child,
        }
        .to_ref()
    }

    pub fn vertical(child: Child) -> UiRef {
        Self {
            vertical: true,
            horizontal: false,
            child,
        }
        .to_ref()
    }

    pub fn horizontal(child: Child) -> UiRef {
        Self {
            vertical: false,
            horizontal: true,
            child,
        }
        .to_ref()
    }
}

impl UiNode for Center {
    fn preferred_dimensions(&self) -> Vec2 {
        let mut size = self.child.node.preferred_dimensions();

        if self.horizontal {
            size.x = f32::INFINITY;
        }

        if self.vertical {
            size.y = f32::INFINITY;
        }

        size
    }

    fn size(&self, area: Area) -> Vec2 {
        let mut size = Vec2::ZERO;

        if self.horizontal {
            size.x = area.size.x;
        }

        if self.vertical {
            size.y = area.size.y;
        }

        size
    }

    fn draw(&self, mut area: Area, ui: &UiState) -> Vec2 {
        let inner = self.child.node.size(area);
        let diff = (area.size - inner).max(Vec2::ZERO);

        if self.vertical {
            area.top_left.y += diff.y / 2.0;
        }

        if self.horizontal {
            area.top_left.x += diff.x / 2.0;
        }

        let mut size = self.child.node.draw(area, ui);

        if self.horizontal {
            size.x = area.size.x;
        }

        if self.vertical {
            size.y = area.size.y;
        }

        size
    }
}

impl UiRef {
    pub fn center(self) -> UiRef {
        Center::new(self)
    }

    pub fn center_horizontal(self) -> UiRef {
        Center::horizontal(self)
    }

    pub fn center_vertical(self) -> UiRef {
        Center::vertical(self)
    }
}
