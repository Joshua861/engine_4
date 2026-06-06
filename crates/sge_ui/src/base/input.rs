use glium::winit::keyboard::{Key, NamedKey};
use sge_api::shapes_2d::draw_rect;
use sge_input::input_text;
use sge_rendering::scissor::{pop_scissor, push_scissor};
use sge_text::{FontRef, TextDrawParams, draw_text_custom, measure_text};
use sge_time::toggle_every_n_seconds;
use sge_vectors::vec2;
use sge_window::use_text_cursor_icon;

use super::*;

pub(crate) const CURSOR_WIDTH: f32 = 2.0;

#[derive(Debug)]
pub struct TextInput {
    state: State<InputState>,

    prompt: String,
    font: FontRef,
    font_size: usize,
    color: Color,
    /// scale the font size by the DPI scaling of your monitor
    do_dpi_scaling: bool,
    padding: f32,
}

impl TextInput {
    pub fn new(
        id: usize,
        prompt: Option<String>,
        font: Option<FontRef>,
        font_size: usize,
        color: Color,
        padding: f32,
        do_dpi_scaling: bool,
    ) -> UiRef {
        let state = State::from_id(id);
        TextInput {
            state,
            prompt: prompt.unwrap_or("Enter text".to_string()),
            font: font.unwrap_or_default(),
            font_size,
            color,
            do_dpi_scaling,
            padding,
        }
        .to_ref()
    }

    fn extra_size(&self) -> Vec2 {
        Vec2::splat(self.padding * 2.0)
    }

    fn offset(&self) -> Vec2 {
        Vec2::splat(self.padding)
    }
}

impl TextInput {
    fn display_value(&self) -> &str {
        let state = self.state.get_or_default();
        if state.value.is_empty() {
            &self.prompt
        } else {
            &state.value
        }
    }
}

#[derive(Default, Debug)]
pub struct InputState {
    pub value: String,
    pub is_active: bool,
    pub changed: bool,
}

impl UiNode for TextInput {
    fn preferred_dimensions(&self) -> Vec2 {
        vec2(
            f32::INFINITY,
            (measure_text(self.display_value()).size + vec2(CURSOR_WIDTH, 0.0)).y,
        ) + self.extra_size()
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state.get_or_default();
        state.changed = false;

        if ui.is_hovered(area) {
            use_text_cursor_icon();
        }

        push_scissor(area.to_glium_rect());

        if state.is_active {
            for key in input_text() {
                if let Key::Character(s) = key {
                    state.value.push_str(s.as_str());
                    state.changed = true;
                }

                if let Key::Named(NamedKey::Space) = key {
                    state.value.push(' ');
                    state.changed = true;
                }

                // if let Key::Named(NamedKey::Enter) = key {
                //     state.value.push('\n');
                // }

                if let Key::Named(NamedKey::Backspace) = key {
                    state.value.pop();
                    state.changed = true;
                }
            }
        }

        let text = self.display_value();

        let mut inner = area;
        inner.size -= self.extra_size();
        inner.top_left += self.offset();

        let dimensions = if state.value.is_empty() {
            draw_text_custom(
                &self.prompt,
                TextDrawParams {
                    color: self.color.with_alpha(0.5),
                    do_dpi_scaling: self.do_dpi_scaling,
                    font: Some(self.font),
                    font_size: self.font_size,
                    position: inner.top_left,
                    ..Default::default()
                },
            )
        } else {
            draw_text_custom(
                text,
                TextDrawParams {
                    color: self.color,
                    do_dpi_scaling: self.do_dpi_scaling,
                    font: Some(self.font),
                    font_size: self.font_size,
                    position: inner.top_left,
                    ..Default::default()
                },
            )
        };

        if state.is_active && toggle_every_n_seconds(0.5) {
            draw_rect(
                if state.value.is_empty() {
                    inner.top_left
                } else {
                    inner.top_left + vec2(dimensions.size.x, 0.0)
                },
                Vec2::new(CURSOR_WIDTH, dimensions.size.y),
                self.color,
            );
        }

        let mut size = dimensions.size + vec2(CURSOR_WIDTH, 0.0);
        size.x = area.width();
        let inhabited_area = Area::new(area.top_left, size + vec2(0.0, self.extra_size().y));

        state.is_active = ui.is_hovered(inhabited_area);

        pop_scissor();

        size + self.extra_size()
    }
}

pub fn text_input_value(id: usize) -> &'static str {
    let state: State<InputState> = State::from_id(id);
    &state.get_or_default().value
}

pub fn text_input_changed(id: usize) -> bool {
    let state: State<InputState> = State::from_id(id);
    state.get_or_default().changed
}

pub fn text_input_state(id: usize) -> &'static mut InputState {
    let state: State<InputState> = State::from_id(id);
    state.get_or_default()
}
