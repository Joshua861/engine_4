use sge_color::Color;
use sge_types::Area;
use sge_vectors::Vec2;
use sge_window::dpi_scaling;

use crate::{Font, FontRef, Glyph, MONO, TextDrawParams, draw_text_custom, measure_text_ex};

pub fn wrap_text_to_width(
    text: &str,
    mut max_width: f32,
    font: &mut Font,
    font_size: usize,
    do_dpi_scaling: bool,
) -> Vec<String> {
    // FIXME: please. this is so sad. this makes me sad
    max_width *= 0.97; // sucks to suck

    let dpi_scaling = if do_dpi_scaling { dpi_scaling() } else { 1.0 };
    let font_size_scaled = (font_size as f32 * dpi_scaling).ceil();

    let mut layout = fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);

    layout.clear();
    layout.append(
        &[&font.font],
        &fontdue::layout::TextStyle::new(" ", font_size_scaled, 0),
    );
    let space_width = layout
        .glyphs()
        .last()
        .map(|g| {
            let glyph = Glyph {
                character: g.parent,
                size: font_size_scaled as usize,
            };
            if !font.contains(glyph) {
                font.cache_glyph(glyph);
            }
            g.x + font.characters[&glyph].advance
        })
        .unwrap_or(0.0);

    let estimated_lines = (text.len() / 50).max(1);
    let mut lines = Vec::with_capacity(estimated_lines);

    for paragraph in text.lines() {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }

        let words: Vec<_> = paragraph.split_whitespace().collect();
        let mut current_line = String::with_capacity(50);
        let mut current_width = 0.0;

        for word in words {
            layout.clear();
            layout.append(
                &[&font.font],
                &fontdue::layout::TextStyle::new(word, font_size_scaled, 0),
            );

            let mut word_width = 0.0f32;
            for glyph_pos in layout.glyphs() {
                let glyph = Glyph {
                    character: glyph_pos.parent,
                    size: font_size_scaled as usize,
                };

                if !font.contains(glyph) {
                    font.cache_glyph(glyph);
                }

                let char_info = &font.characters[&glyph];
                let glyph_end = glyph_pos.x + char_info.advance;
                word_width = word_width.max(glyph_end);
            }

            if current_line.is_empty() {
                if word_width <= max_width {
                    current_line.push_str(word);
                    current_width = word_width;
                } else {
                    lines.push(word.to_string());
                }
            } else {
                let test_width = current_width + space_width + word_width;

                if test_width <= max_width {
                    current_line.push(' ');
                    current_line.push_str(word);
                    current_width = test_width;
                } else {
                    lines.push(current_line);
                    current_line = String::with_capacity(50);
                    current_line.push_str(word);
                    current_width = word_width;
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }

    lines
}

pub fn get_space_width(font: &mut Font, font_size: usize) -> f32 {
    let space_glyph = Glyph {
        character: ' ',
        size: font_size,
    };

    if !font.contains(space_glyph) {
        font.cache_glyph(space_glyph);
    }

    font.characters
        .get(&space_glyph)
        .map(|c| c.advance)
        .unwrap_or(0.0)
}

pub fn draw_wrapped_text_in_area(
    text: &str,
    area: Area,
    font: Option<FontRef>,
    font_size: usize,
    color: Color,
    do_dpi_scaling: bool,
    line_spacing: f32,
) -> Vec2 {
    let params = TextDrawParams {
        font,
        font_size,
        color,
        position: Vec2::ZERO,
        do_dpi_scaling,
    };

    let wrapped_lines = wrap_text_to_width(
        text,
        area.width(),
        font.unwrap_or(MONO).get_mut(),
        font_size,
        do_dpi_scaling,
    );

    let mut y_offset = 0.0;
    let line_height = font_size as f32 * line_spacing;
    let mut max_width: f32 = 0.0;

    for line in &wrapped_lines {
        let mut line_params = params;
        line_params.position = area.top_left() + Vec2::new(0.0, y_offset);
        let line_size = draw_text_custom(line, line_params).size;
        max_width = max_width.max(line_size.x);
        y_offset += line_height;
    }

    Vec2::new(max_width, y_offset)
}

pub fn measure_wrapped_text(
    text: &str,
    max_width: f32,
    font: Option<FontRef>,
    font_size: usize,
    do_dpi_scaling: bool,
    line_spacing: f32,
) -> Vec2 {
    let params = TextDrawParams {
        font,
        font_size,
        color: Color::NEUTRAL_100,
        position: Vec2::ZERO,
        do_dpi_scaling,
    };

    let wrapped_lines = wrap_text_to_width(
        text,
        max_width,
        font.unwrap_or(MONO).get_mut(),
        font_size,
        do_dpi_scaling,
    );
    let line_height = font_size as f32 * line_spacing;
    let total_height = wrapped_lines.len() as f32 * line_height;

    let mut max_actual_width = 0.0f32;
    for line in &wrapped_lines {
        let width = measure_text_ex(line, params).size.x;
        max_actual_width = max_actual_width.max(width);
    }

    Vec2::new(max_actual_width, total_height)
}
