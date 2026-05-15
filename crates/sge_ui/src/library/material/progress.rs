use sge_vectors::vec2;

use crate::base::{self, EMPTY, Fill, SizedBox};

use super::*;

pub struct ProgressBar;

impl ProgressBar {
    pub fn new(width: f32, color: Color, bg: Color, value: f32, max: f32, id: usize) -> UiRef {
        SizedBox::new(
            vec2(width, PROGRESS_HEIGHT),
            base::ProgressBar::new(
                value,
                max,
                Fill::rounded(color, PROGRESS_HEIGHT, EMPTY).padding_right(4.0),
                Fill::rounded(bg, PROGRESS_HEIGHT, EMPTY),
                id,
            ),
        )
    }

    pub fn primary(width: f32, value: f32, max: f32, id: usize) -> UiRef {
        let scheme = scheme();
        Self::new(
            width,
            scheme.primary,
            scheme.secondary_container,
            value,
            max,
            id,
        )
    }
}
