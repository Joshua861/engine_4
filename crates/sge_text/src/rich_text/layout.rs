use sge_vectors::Vec2;

use crate::{TextDrawParams, Token, measure_text_custom, tokenize};

use super::{RichText, RichTextStyle};

pub struct LayoutWord {
    pub text: String,
    pub style: RichTextStyle,
    pub space_before: f32,
    pub width: f32,
}

pub struct LayoutLine {
    pub words: Vec<LayoutWord>,
    pub max_font_size: usize,
}

impl LayoutLine {
    pub(crate) fn total_width(&self) -> f32 {
        self.words.iter().map(|w| w.space_before + w.width).sum()
    }
}

impl RichText {
    fn measure_word(word: &str, style: &RichTextStyle) -> f32 {
        let params = TextDrawParams {
            font: Some(style.font()),
            font_size: style.font_size,
            color: style.color,
            position: Vec2::ZERO,
            line_spacing: 1.0,
        };
        measure_text_custom(word, params).size.x
    }

    fn measure_space(style: &RichTextStyle) -> f32 {
        let font = style.font().get_mut();
        font.measure_space_width(style.font_size)
    }

    pub fn layout(&self, max_width: f32) -> Vec<LayoutLine> {
        let mut lines = Vec::new();

        let mut current = LayoutLine {
            words: Vec::new(),
            max_font_size: 0,
        };

        let flush = |lines: &mut Vec<LayoutLine>, current: &mut LayoutLine| {
            if !current.words.is_empty() {
                lines.push(LayoutLine {
                    words: std::mem::take(&mut current.words),
                    max_font_size: current.max_font_size,
                });
                current.max_font_size = 0;
            }
        };

        let mut pending_spaces: usize = 0;

        for block in &self.blocks {
            let mut first_paragraph = true;

            for paragraph in block.text.split('\n') {
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
                            let word_width = Self::measure_word(word, &block.style);

                            let space_before = if pending_spaces > 0 && !current.words.is_empty() {
                                Self::measure_space(&block.style) * pending_spaces as f32
                            } else {
                                0.0
                            };

                            let needed = current.total_width() + space_before + word_width;

                            if !current.words.is_empty() && needed > max_width {
                                flush(&mut lines, &mut current);
                            }

                            current.words.push(LayoutWord {
                                text: word.to_string(),
                                style: block.style.clone(),
                                space_before: if current.words.is_empty() {
                                    0.0
                                } else {
                                    space_before
                                },
                                width: word_width,
                            });

                            current.max_font_size =
                                current.max_font_size.max(block.style.font_size);

                            pending_spaces = 0;
                        }
                    }
                }
            }
        }

        flush(&mut lines, &mut current);
        lines
    }
}
