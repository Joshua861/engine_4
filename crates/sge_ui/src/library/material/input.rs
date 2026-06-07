use sge_text::SANS;

use crate::base;

use super::*;

pub struct TextInput;

impl TextInput {
    fn new(id: usize, prompt: Option<String>, width: f32, bg: Color, fg: Color) -> UiRef {
        base::TextInput::new(id, prompt, Some(SANS), rem(1.0), fg, INPUT_PADDING)
            .width(width)
            .rounded_fill(bg, INPUT_RADIUS)
            .fit()
    }

    pub fn primary(id: usize, prompt: Option<String>, width: f32) -> UiRef {
        let scheme = scheme();
        Self::new(
            id,
            prompt,
            width,
            scheme.primary_container,
            scheme.on_primary_container,
        )
    }

    pub fn surface(id: usize, prompt: Option<String>, width: f32) -> UiRef {
        let scheme = scheme();
        Self::new(
            id,
            prompt,
            width,
            scheme.surface_container,
            scheme.on_surface,
        )
    }
}
