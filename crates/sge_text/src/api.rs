#![allow(const_item_mutation)]

use sge_color::Color;
use sge_rendering::{d2::Renderer2D, dq2d, wdq2d};
use sge_types::Area;
use sge_vectors::Vec2;

use crate::{DEFAULT_FONT_SIZE, FontRef, MONO, TextDimensions, TextDrawParams};

pub fn draw_text_to(text: impl ToString, position: Vec2, renderer: Renderer2D) -> TextDimensions {
    MONO.draw_text_to(
        text.to_string(),
        position,
        Color::WHITE,
        DEFAULT_FONT_SIZE,
        1.0,
        renderer,
    )
}

pub fn draw_text(text: impl ToString, position: Vec2) -> TextDimensions {
    draw_text_to(text, position, dq2d())
}

pub fn draw_text_world(text: impl ToString, position: Vec2) -> TextDimensions {
    draw_text_to(text, position, wdq2d())
}

pub fn draw_colored_text_to(
    text: impl ToString,
    position: Vec2,
    color: Color,
    renderer: Renderer2D,
) -> TextDimensions {
    MONO.draw_text_to(
        text.to_string(),
        position,
        color,
        DEFAULT_FONT_SIZE,
        1.0,
        renderer,
    )
}

pub fn draw_colored_text(text: impl ToString, position: Vec2, color: Color) -> TextDimensions {
    draw_colored_text_to(text, position, color, dq2d())
}

pub fn draw_colored_text_world(
    text: impl ToString,
    position: Vec2,
    color: Color,
) -> TextDimensions {
    draw_colored_text_to(text, position, color, wdq2d())
}

pub fn draw_text_ex_to(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
    renderer: Renderer2D,
) -> TextDimensions {
    MONO.draw_text_to(
        text.to_string(),
        position,
        color,
        font_size,
        line_spacing,
        renderer,
    )
}

pub fn draw_text_ex(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
) -> TextDimensions {
    draw_text_ex_to(text, position, color, font_size, 1.0, dq2d())
}

pub fn draw_text_ex_world(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
) -> TextDimensions {
    draw_text_ex_to(text, position, color, font_size, 1.0, wdq2d())
}

pub fn draw_multiline_text_to(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
    renderer: Renderer2D,
) -> TextDimensions {
    MONO.draw_text_to(
        text.to_string(),
        position,
        color,
        font_size,
        line_spacing,
        renderer,
    )
}

pub fn draw_multiline_text(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(text, position, color, font_size, line_spacing, dq2d())
}

pub fn draw_multiline_text_world(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(text, position, color, font_size, line_spacing, wdq2d())
}

pub fn draw_text_custom_to(
    text: impl ToString,
    params: TextDrawParams,
    renderer: Renderer2D,
) -> TextDimensions {
    params.font.unwrap_or(MONO).draw_text_to(
        text.to_string(),
        params.position,
        params.color,
        params.font_size,
        params.line_spacing,
        renderer,
    )
}

pub fn draw_text_custom(text: impl ToString, params: TextDrawParams) -> TextDimensions {
    draw_text_custom_to(text, params, dq2d())
}

pub fn draw_text_custom_world(text: impl ToString, params: TextDrawParams) -> TextDimensions {
    draw_text_custom_to(text, params, wdq2d())
}

pub fn measure_text(text: impl ToString) -> TextDimensions {
    MONO.measure_text(text.to_string(), DEFAULT_FONT_SIZE)
}

pub fn measure_text_ex(text: impl ToString, mut font: FontRef, font_size: usize) -> TextDimensions {
    font.measure_text(text.to_string(), font_size)
}

pub fn measure_multiline_text(text: impl ToString, line_spacing: f32) -> TextDimensions {
    MONO.measure_multiline_text(text.to_string(), DEFAULT_FONT_SIZE, line_spacing)
}

pub fn measure_multiline_text_ex(
    text: impl ToString,
    font_size: usize,
    line_spacing: f32,
) -> TextDimensions {
    MONO.measure_multiline_text(text.to_string(), font_size, line_spacing)
}

pub fn measure_text_custom(text: impl ToString, params: TextDrawParams) -> TextDimensions {
    params.font.unwrap_or(MONO).measure_multiline_text(
        text.to_string(),
        params.font_size,
        params.line_spacing,
    )
}

pub fn measure_wrapped_text(
    text: impl ToString,
    max_width: f32,
    font_size: usize,
    line_spacing: f32,
) -> Vec2 {
    MONO.measure_wrapped_text(text.to_string(), max_width, font_size, line_spacing)
}

pub fn measure_wrapped_text_ex(
    text: impl ToString,
    font_size: usize,
    mut font: FontRef,
    max_width: f32,
    line_spacing: f32,
) -> Vec2 {
    font.measure_wrapped_text(text.to_string(), max_width, font_size, line_spacing)
}

pub fn draw_wrapped_text_to(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
    max_width: f32,
    renderer: Renderer2D,
) -> Vec2 {
    MONO.draw_wrapped_text(
        text.to_string(),
        position,
        color,
        font_size,
        line_spacing,
        max_width,
        renderer,
    )
}

pub fn draw_wrapped_text(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
    max_width: f32,
) -> Vec2 {
    draw_wrapped_text_to(
        text,
        position,
        color,
        font_size,
        line_spacing,
        max_width,
        dq2d(),
    )
}

pub fn draw_wrapped_text_world(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
    max_width: f32,
) -> Vec2 {
    draw_wrapped_text_to(
        text,
        position,
        color,
        font_size,
        line_spacing,
        max_width,
        wdq2d(),
    )
}

pub fn draw_wrapped_text_in_area_to(
    text: impl ToString,
    area: Area,
    color: Color,
    font_size: usize,
    font: FontRef,
    line_spacing: f32,
    renderer: Renderer2D,
) -> Vec2 {
    draw_wrapped_text_ex_to(
        text,
        area.top_left(),
        color,
        font_size,
        font,
        line_spacing,
        area.width(),
        renderer,
    )
}

pub fn draw_wrapped_text_in_area(
    text: impl ToString,
    area: Area,
    color: Color,
    font_size: usize,
    font: FontRef,
    line_spacing: f32,
) -> Vec2 {
    draw_wrapped_text_in_area_to(text, area, color, font_size, font, line_spacing, dq2d())
}

pub fn draw_wrapped_text_in_area_world(
    text: impl ToString,
    area: Area,
    color: Color,
    font_size: usize,
    font: FontRef,
    line_spacing: f32,
) -> Vec2 {
    draw_wrapped_text_in_area_to(text, area, color, font_size, font, line_spacing, wdq2d())
}

pub fn draw_wrapped_text_ex_to(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    mut font: FontRef,
    line_spacing: f32,
    max_width: f32,
    renderer: Renderer2D,
) -> Vec2 {
    font.draw_wrapped_text(
        text.to_string(),
        position,
        color,
        font_size,
        line_spacing,
        max_width,
        renderer,
    )
}

pub fn draw_wrapped_text_ex(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
    max_width: f32,
    font: FontRef,
) -> Vec2 {
    draw_wrapped_text_ex_to(
        text,
        position,
        color,
        font_size,
        font,
        line_spacing,
        max_width,
        dq2d(),
    )
}

pub fn draw_wrapped_text_ex_world(
    text: impl ToString,
    position: Vec2,
    color: Color,
    font_size: usize,
    line_spacing: f32,
    max_width: f32,
    font: FontRef,
) -> Vec2 {
    draw_wrapped_text_ex_to(
        text,
        position,
        color,
        font_size,
        font,
        line_spacing,
        max_width,
        wdq2d(),
    )
}
