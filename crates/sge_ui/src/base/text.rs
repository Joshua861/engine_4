use core::f32;

use sge_color::Color;
use sge_vectors::Vec2;

use super::*;
use sge_text::{
    FontRef, MONO, SANS, SANS_BOLD, SANS_BOLD_ITALIC, SANS_DISPLAY, SANS_ITALIC, TextDrawParams,
    draw_wrapped_text_in_area, measure_wrapped_text_ex,
};

#[derive(Debug)]
pub struct Text {
    text: String,
    font: FontRef,
    font_size: usize,
    color: Color,
    line_spacing: f32,
    wrap: bool,
}

impl Default for Text {
    fn default() -> Self {
        let params = TextDrawParams::default();
        Self {
            text: String::new(),
            font_size: params.font_size,
            color: params.color,
            font: SANS,
            line_spacing: 1.0,
            wrap: true,
        }
    }
}

impl From<Text> for TextDrawParams {
    fn from(value: Text) -> Self {
        TextDrawParams {
            font: Some(value.font),
            font_size: value.font_size,
            color: value.color,
            position: Vec2::ZERO,
            ..Default::default()
        }
    }
}

impl From<&Text> for TextDrawParams {
    fn from(value: &Text) -> Self {
        TextDrawParams {
            font: Some(value.font),
            font_size: value.font_size,
            color: value.color,
            position: Vec2::ZERO,
            ..Default::default()
        }
    }
}

#[bon::bon]
impl Text {
    pub fn new(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
        .to_ref()
    }

    pub fn nowrap(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            wrap: false,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn nowrap_with_color(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            color,
            wrap: false,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn body(text: impl ToString) -> UiRef {
        Self::new(text)
    }

    pub fn mono(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn mono_sized(text: impl ToString, font_size: usize) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            font_size,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn mono_nowrap(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            wrap: false,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn mono_colored(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn title(text: impl ToString) -> UiRef {
        Padding::tblr(
            2.0,
            10.0,
            0.0,
            0.0,
            Center::horizontal(
                Self {
                    text: text.to_string(),
                    font_size: 48,
                    font: SANS_DISPLAY,
                    ..Default::default()
                }
                .to_ref(),
            ),
        )
    }

    pub fn title_nowrap(text: impl ToString) -> UiRef {
        Padding::tblr(
            2.0,
            10.0,
            0.0,
            0.0,
            Center::horizontal(
                Self {
                    text: text.to_string(),
                    font_size: 48,
                    font: SANS_DISPLAY,
                    wrap: false,
                    ..Default::default()
                }
                .to_ref(),
            ),
        )
    }

    pub fn h1_no_padding(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 40,
            font: SANS_DISPLAY,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn h1(text: impl ToString) -> UiRef {
        Padding::tblr(20.0, 10.0, 0.0, 0.0, Self::h1_no_padding(text))
    }

    pub fn h2_no_padding(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 32,
            font: SANS_DISPLAY,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn h2(text: impl ToString) -> UiRef {
        Padding::tblr(20.0, 10.0, 0.0, 0.0, Self::h2_no_padding(text))
    }

    pub fn h3_no_padding(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 26,
            font: SANS_DISPLAY,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn h3(text: impl ToString) -> UiRef {
        Padding::tblr(20.0, 10.0, 0.0, 0.0, Self::h3_no_padding(text))
    }

    pub fn small(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 12,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn italic(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_ITALIC,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn italic_with_color(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_ITALIC,
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn bold(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_BOLD,
            font_size: 18,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn bold_with_color(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_BOLD,
            font_size: 18,
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn bold_italic(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_BOLD_ITALIC,
            font_size: 18,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn bold_italic_with_color(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_BOLD_ITALIC,
            font_size: 18,
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_with_color(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_with_size_color(text: impl ToString, font_size: usize, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            font_size,
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_with_size(text: impl ToString, font_size: usize) -> UiRef {
        Self {
            text: text.to_string(),
            font_size,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_full(
        text: impl ToString,
        font: FontRef,
        font_size: usize,
        color: Color,
        line_spacing: f32,
        wrap: bool,
    ) -> UiRef {
        Self {
            text: text.to_string(),
            font,
            font_size,
            color,
            line_spacing,
            wrap,
        }
        .to_ref()
    }

    #[builder]
    pub fn builder(
        text: impl ToString,
        font: Option<FontRef>,
        font_size: Option<usize>,
        color: Option<Color>,
        line_spacing: Option<f32>,
        wrap: Option<bool>,
    ) -> UiRef {
        Self {
            text: text.to_string(),
            font: font.unwrap_or(SANS),
            font_size: font_size.unwrap_or(TextDrawParams::default().font_size),
            color: color.unwrap_or(TextDrawParams::default().color),
            line_spacing: line_spacing.unwrap_or(1.0),
            wrap: wrap.unwrap_or(true),
        }
        .to_ref()
    }

    pub fn from_params(text: impl ToString, params: TextDrawParams, wrap: bool) -> UiRef {
        Self {
            text: text.to_string(),
            font: params.font.unwrap_or(SANS),
            font_size: params.font_size,
            color: params.color,
            line_spacing: 1.0,
            wrap,
        }
        .to_ref()
    }
}

impl UiNode for Text {
    fn preferred_dimensions(&self) -> sge_vectors::Vec2 {
        let mut font = self.font.clone();
        font.measure_multiline_text(self.text.clone(), self.font_size, self.line_spacing)
            .size
    }

    fn size(&self, area: Area) -> Vec2 {
        if self.wrap {
            measure_wrapped_text_ex(
                &self.text,
                self.font_size,
                self.font,
                area.width(),
                self.line_spacing,
            )
        } else {
            self.preferred_dimensions()
        }
    }

    fn draw(&self, area: super::Area, _: &UiState) -> Vec2 {
        if self.wrap {
            draw_wrapped_text_in_area(
                &self.text,
                area,
                self.color,
                self.font_size,
                self.font,
                self.line_spacing,
            )
        } else {
            self.font
                .get_mut()
                .draw_text(
                    self.text.clone(),
                    area.top_left,
                    self.color,
                    self.font_size,
                    self.line_spacing,
                )
                .size
        }
    }
}

#[derive(Debug)]
pub struct RichTextNode {
    text: sge_text::rich_text::RichText,
    line_spacing: f32,
}

impl RichTextNode {
    pub fn new(text: sge_text::rich_text::RichText) -> UiRef {
        Self {
            text,
            line_spacing: 1.1,
        }
        .to_ref()
    }

    pub fn with_line_spacing(text: sge_text::rich_text::RichText, line_spacing: f32) -> UiRef {
        Self { text, line_spacing }.to_ref()
    }
}

impl UiNode for RichTextNode {
    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        self.text.draw(area, self.line_spacing).size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.text.measure(f32::INFINITY, self.line_spacing)
    }

    fn size(&self, area: Area) -> Vec2 {
        self.text.measure(area.width(), self.line_spacing)
    }
}
