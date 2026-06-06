use crate::{FontRef, FontType, MONO_TYPEFACE, Typeface};
use parse::RichTextParser;
use sge_color::Color;
use sge_rendering::{d2::Renderer2D, dq2d, wdq2d};
use sge_types::Area;
use sge_vectors::Vec2;

use super::TextDimensions;

mod draw;
mod layout;
mod parse;

pub use parse::{RichTextParseError, RichTextParseErrorKind};

#[derive(Debug, Clone)]
pub struct RichText {
    pub blocks: Vec<RichTextBlock>,
    pub line_spacing: f32,
}

#[derive(Debug, Clone)]
pub struct RichTextBlock {
    pub text: String,
    pub style: RichTextStyle,
}

impl RichTextBlock {
    pub fn from_color(text: impl ToString, color: Color) -> Self {
        Self {
            text: text.to_string(),
            style: RichTextStyle {
                color,
                ..Default::default()
            },
        }
    }

    pub fn underlined(text: impl ToString, color: Color) -> Self {
        Self {
            text: text.to_string(),
            style: RichTextStyle {
                underline: Some(color),
                ..Default::default()
            },
        }
    }

    pub fn with_typeface(mut self, typeface: Typeface) -> Self {
        self.style.typeface = typeface;
        self
    }

    pub fn with_mono(mut self) -> Self {
        self.style.typeface = MONO_TYPEFACE;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RichTextStyle {
    pub color: Color,
    pub typeface: Typeface,
    pub font_size: usize,
    pub font_type: FontType,
    pub underline: Option<Color>,
    pub strikethrough: Option<Color>,
    pub href: Option<String>,
    pub highlight: Option<Color>,
    pub outline: Option<Color>,
}

impl RichTextStyle {
    pub fn font(&self) -> FontRef {
        self.typeface.get_font(self.font_type)
    }
}

impl Default for RichTextStyle {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            typeface: Typeface::default(),
            font_size: 16,
            font_type: FontType::Regular,
            underline: None,
            strikethrough: None,
            href: None,
            highlight: None,
            outline: None,
        }
    }
}

pub struct RichTextParams {
    pub line_spacing: f32,
    pub do_dpi_scaling: bool,
}

impl Default for RichTextParams {
    fn default() -> Self {
        Self {
            line_spacing: 1.1,
            do_dpi_scaling: false,
        }
    }
}

pub fn rich_text(input: impl AsRef<str>) -> Result<RichText, Vec<RichTextParseError>> {
    RichText::parse(input)
}

pub fn rich_text_blocks(blocks: Vec<RichTextBlock>) -> RichText {
    RichText::new(blocks)
}

impl RichText {
    pub fn new(blocks: Vec<RichTextBlock>) -> Self {
        Self {
            blocks,
            line_spacing: 1.1,
        }
    }

    pub fn with_line_spacing(mut self, line_spacing: f32) -> Self {
        self.line_spacing = line_spacing;
        self
    }

    pub fn parse(input: impl AsRef<str>) -> Result<RichText, Vec<RichTextParseError>> {
        RichTextParser::new(input.as_ref()).run()
    }

    pub fn measure(&self, max_width: f32, line_spacing: f32) -> Vec2 {
        let lines = self.layout(max_width, true);
        Self::measure_layout(&lines, line_spacing)
    }

    pub fn measure_layout(layout: &[layout::LayoutLine], line_spacing: f32) -> Vec2 {
        let mut total_width = 0.0f32;
        let mut y = 0.0f32;

        for line in layout {
            total_width = total_width.max(line.total_width());
            y += line.max_font_size as f32 * line_spacing;
        }

        Vec2::new(total_width, y)
    }

    pub fn draw_to(&self, area: Area, line_spacing: f32, renderer: Renderer2D) -> TextDimensions {
        let lines = self.layout(area.width(), true);
        let params = RichTextParams {
            line_spacing,
            do_dpi_scaling: false,
        };
        self.draw_layout_to(&lines, area.top_left(), params, renderer)
    }

    pub fn draw(&self, area: Area, line_spacing: f32) -> TextDimensions {
        self.draw_to(area, line_spacing, dq2d())
    }

    pub fn draw_world(&self, area: Area, line_spacing: f32) -> TextDimensions {
        self.draw_to(area, line_spacing, wdq2d())
    }

    pub fn draw_to_ex(
        &self,
        area: Area,
        params: RichTextParams,
        renderer: Renderer2D,
    ) -> TextDimensions {
        let lines = self.layout(area.width(), params.do_dpi_scaling);
        self.draw_layout_to(&lines, area.top_left(), params, renderer)
    }

    pub fn draw_ex(&self, area: Area, params: RichTextParams) -> TextDimensions {
        self.draw_to_ex(area, params, dq2d())
    }

    pub fn draw_world_ex(&self, area: Area, params: RichTextParams) -> TextDimensions {
        self.draw_to_ex(area, params, wdq2d())
    }

    pub fn draw_layout(
        &self,
        lines: &[layout::LayoutLine],
        origin: Vec2,
        params: RichTextParams,
    ) -> TextDimensions {
        self.draw_layout_to(lines, origin, params, dq2d())
    }

    pub fn draw_layout_world(
        &self,
        lines: &[layout::LayoutLine],
        origin: Vec2,
        params: RichTextParams,
    ) -> TextDimensions {
        self.draw_layout_to(lines, origin, params, wdq2d())
    }

    pub fn print_to_stdout(&self) {
        use owo_colors::OwoColorize;

        for block in &self.blocks {
            let mut text = if block.style.color == Color::WHITE {
                block.text.to_string()
            } else {
                let (r, g, b) = block.style.color.to_u8();
                block.text.truecolor(r, g, b).to_string()
            };

            if block.style.underline.is_some() {
                text = text.underline().to_string();
            }

            if block.style.strikethrough.is_some() {
                text = text.strikethrough().to_string();
            }

            if let Some(c) = block.style.highlight {
                let (r, g, b) = c.to_u8();
                text = text.on_truecolor(r, g, b).to_string();
            }

            print!("{}", text);
        }
        println!();
    }
}
