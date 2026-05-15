use crate::prelude::*;

pub struct Button;

impl Button {
    pub fn new(bg: Color, hover: Color, id: usize, child: Child) -> UiRef {
        Fit::new(
            Fill::builder()
                .color(bg)
                .hover_color(hover)
                .active_color(bg)
                .child(base::Button::new(
                    id,
                    Padding::tblr(10., 15., 40., 40., child),
                ))
                .build(),
        )
    }

    pub fn primary(id: usize, child: Child) -> UiRef {
        Self::new(super::BG1, super::BG2, id, child)
    }

    pub fn text(bg: Color, hover: Color, id: usize, text: impl ToString) -> UiRef {
        Self::new(bg, hover, id, Text::nowrap(text))
    }

    pub fn primary_text(id: usize, text: impl ToString) -> UiRef {
        Self::text(super::BG1, super::BG2, id, text)
    }

    pub fn danger(id: usize, child: Child) -> UiRef {
        Self::new(super::BG1, super::ERROR, id, child)
    }

    pub fn danger_text(id: usize, text: impl ToString) -> UiRef {
        Self::text(super::BG1, super::ERROR, id, text)
    }
}
