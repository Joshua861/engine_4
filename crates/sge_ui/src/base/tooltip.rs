use std::cell::Cell;

use sge_input::last_cursor_pos;
use sge_vectors::vec2;

use super::*;

#[derive(Debug)]
pub struct Tooltip {
    inner: Child,
    overlay: Child,
    positioning: TooltipPosition,
    offset: f32,
    overlay_area: Cell<Area>,
}

#[derive(Debug, Clone, Copy)]
pub enum TooltipPosition {
    Top,
    Left,
    Bottom,
    Right,
    BelowCursorRight,
    AboveCursorRight,
    BelowCursorLeft,
    AboveCursorLeft,
}

impl Tooltip {
    pub fn new(inner: Child, overlay: Child, offset: f32, positioning: TooltipPosition) -> UiRef {
        Self {
            inner: inner,
            overlay: overlay,
            positioning,
            offset,
            overlay_area: Area::new(Vec2::ZERO, Vec2::ZERO).into(),
        }
        .to_ref()
    }

    pub(crate) fn actually_draw(&self) {
        draw_ui_in_area(self.overlay, self.overlay_area.get());
    }
}

impl UiNode for Tooltip {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let size = self.inner.draw(area, ui);

        let child_area = Area::new(area.top_left, size);
        if ui.is_hovered(child_area) {
            let size = self.overlay.preferred_dimensions();
            let pos = match self.positioning {
                TooltipPosition::Top => Vec2::new(
                    child_area.top_left.x + (child_area.width() - size.x) / 2.0,
                    child_area.top() - size.y - self.offset,
                ),
                TooltipPosition::Left => Vec2::new(
                    child_area.left() - size.x - self.offset,
                    child_area.top_left.y + (child_area.height() - size.y) / 2.0,
                ),
                TooltipPosition::Bottom => Vec2::new(
                    child_area.top_left.x + (child_area.width() - size.x) / 2.0,
                    child_area.bottom() + self.offset,
                ),
                TooltipPosition::Right => Vec2::new(
                    child_area.right() + self.offset,
                    child_area.top_left.y + (child_area.height() - size.y) / 2.0,
                ),
                TooltipPosition::AboveCursorRight => last_cursor_pos() - vec2(0.0, size.y),
                TooltipPosition::BelowCursorRight => last_cursor_pos(),
                TooltipPosition::AboveCursorLeft => last_cursor_pos() - size,
                TooltipPosition::BelowCursorLeft => last_cursor_pos() - vec2(size.x, 0.0),
            };

            self.overlay_area.set(Area::new(pos, size));
            get_ui_storage().tooltips.push(self as *const Self);
        }

        size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.inner.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.inner.size(area)
    }
}
