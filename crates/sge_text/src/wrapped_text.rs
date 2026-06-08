use sge_color::Color;
use sge_rendering::d2::Renderer2D;
use sge_vectors::{Vec2, vec2};

use crate::{CharacterInfo, Font, FontSource, Glyph};

pub struct LayoutWord {
    pub text: String,
    pub space_before: f32,
    pub width: f32,
}

pub struct LayoutLine {
    pub words: Vec<LayoutWord>,
}

impl LayoutLine {
    pub(crate) fn total_width(&self) -> f32 {
        self.words.iter().map(|w| w.space_before + w.width).sum()
    }
}

impl Font {
    pub(crate) fn measure_space(&mut self, font_size: usize) -> CharacterInfo {
        self.font.character_info(Glyph {
            character: ' ',
            size: font_size,
        })
    }

    pub(crate) fn measure_space_width(&mut self, font_size: usize) -> f32 {
        self.measure_space(font_size).advance
    }

    pub fn layout(&mut self, text: String, font_size: usize, max_width: f32) -> Vec<LayoutLine> {
        let mut lines = Vec::new();

        let mut current = LayoutLine { words: Vec::new() };

        let flush = |lines: &mut Vec<LayoutLine>, current: &mut LayoutLine| {
            if !current.words.is_empty() {
                lines.push(LayoutLine {
                    words: std::mem::take(&mut current.words),
                });
            }
        };

        let mut pending_spaces: usize = 0;

        let mut first_paragraph = true;
        for paragraph in text.split('\n') {
            if !first_paragraph {
                flush(&mut lines, &mut current);
                pending_spaces = 0;
            }
            first_paragraph = false;

            for token in tokenize(paragraph) {
                match token {
                    Token::Spaces(count) => {
                        pending_spaces += count;
                    }

                    Token::Text(word) => {
                        let word_width = self.measure_text(word.to_string(), font_size).size.x;
                        let mut space_before = if pending_spaces > 0 && !current.words.is_empty() {
                            self.measure_space_width(font_size) * pending_spaces as f32
                        } else {
                            0.0
                        };

                        let needed = current.total_width() + space_before + word_width;

                        if !current.words.is_empty() && needed > max_width {
                            flush(&mut lines, &mut current);
                            space_before = 0.0;
                        }

                        current.words.push(LayoutWord {
                            text: word.to_string(),
                            space_before: space_before,
                            width: word_width,
                        });

                        pending_spaces = 0;
                    }
                }
            }
        }

        flush(&mut lines, &mut current);
        lines
    }

    pub fn measure_wrapped_text(
        &mut self,
        text: String,
        max_width: f32,
        font_size: usize,
        line_spacing: f32,
    ) -> Vec2 {
        let layout = self.layout(text, font_size, max_width);
        self.measure_layout(layout, font_size, line_spacing)
    }

    pub fn measure_layout(
        &mut self,
        layout: Vec<LayoutLine>,
        font_size: usize,
        line_spacing: f32,
    ) -> Vec2 {
        let mut max_width = 0.0f32;

        let metrics = self.font.font_metrics(font_size);
        let base_line_height = metrics.line_height();
        let line_height = base_line_height * line_spacing;

        let total_height = if layout.is_empty() {
            0.0
        } else {
            line_height * layout.len().saturating_sub(1) as f32 + base_line_height
        };

        for line in layout {
            max_width = max_width.max(line.total_width());
        }

        Vec2::new(max_width, total_height)
    }

    pub fn draw_wrapped_text(
        &mut self,
        text: String,
        position: Vec2,
        color: Color,
        font_size: usize,
        line_spacing: f32,
        max_width: f32,
        renderer: Renderer2D,
    ) -> Vec2 {
        let layout = self.layout(text, font_size, max_width);
        self.draw_layout(layout, position, color, font_size, line_spacing, renderer)
    }

    pub fn draw_layout(
        &mut self,
        layout: Vec<LayoutLine>,
        position: Vec2,
        color: Color,
        font_size: usize,
        line_spacing: f32,
        renderer: Renderer2D,
    ) -> Vec2 {
        let mut max_width = 0.0f32;

        let metrics = self.font.font_metrics(font_size);
        let base_line_height = metrics.line_height();
        let line_height = base_line_height * line_spacing;

        let total_height = if layout.is_empty() {
            0.0
        } else {
            line_height * layout.len().saturating_sub(1) as f32 + base_line_height
        };

        let mut y_offset = position.y;

        for line in layout {
            max_width = max_width.max(line.total_width());

            let mut x = position.x;

            for word in line.words {
                x += word.space_before;
                self.draw_text_to(
                    word.text,
                    vec2(x, y_offset),
                    color,
                    font_size,
                    line_spacing,
                    renderer,
                );
                x += word.width;
            }

            y_offset += line_height;
        }

        Vec2::new(max_width, total_height)
    }
}

pub(crate) enum Token<'a> {
    Text(&'a str),
    Spaces(usize),
}

pub(crate) fn tokenize(text: &str) -> impl Iterator<Item = Token<'_>> {
    let mut chars = text.char_indices().peekable();

    std::iter::from_fn(move || {
        let (start, first) = chars.next()?;

        if first.is_whitespace() {
            let mut count = 1;
            while let Some((_, c)) = chars.peek() {
                if !c.is_whitespace() {
                    break;
                }
                count += 1;
                chars.next();
            }

            return Some(Token::Spaces(count));
        }

        let mut end = start + first.len_utf8();

        while let Some((i, c)) = chars.peek().copied() {
            if c.is_whitespace() {
                break;
            }

            chars.next();
            end = i + c.len_utf8();
        }

        Some(Token::Text(&text[start..end]))
    })
}
