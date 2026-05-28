use glium::winit::keyboard::{Key, NamedKey};
use sge_api::shapes_2d::draw_rect;
use sge_input::input_text;
use sge_rendering::scissor::{pop_scissor, push_scissor};
use sge_text::{FontRef, TextDrawParams, draw_text_custom, measure_text};
use sge_time::toggle_every_n_seconds;
use sge_vectors::vec2;
use sge_window::use_text_cursor_icon;
use std::str::FromStr;

use super::*;

#[derive(Debug)]
pub struct DataInput<T: DataInputValue + 'static> {
    state: State<DataInputState<T>>,
    prompt: String,
    font: FontRef,
    font_size: usize,
    color: Color,
    /// scale the font size by the DPI scaling of your monitor
    do_dpi_scaling: bool,
    padding: f32,
}

pub trait DataInputValue: FromStr<Err: Debug> + Debug + 'static {}

impl<T> DataInputValue for T
where
    T: FromStr + Debug + 'static,
    <T as FromStr>::Err: Debug,
{
}

#[derive(Debug)]
pub struct DataInputState<T: DataInputValue> {
    pub text: String,
    pub value: Option<T>,
    pub err: Option<<T as FromStr>::Err>,
    pub is_active: bool,
    pub changed: bool,
}

impl<T: DataInputValue> Default for DataInputState<T> {
    fn default() -> Self {
        Self {
            text: String::new(),
            value: None,
            err: None,
            is_active: false,
            changed: false,
        }
    }
}

impl<T: DataInputValue + 'static> DataInput<T> {
    pub fn new(
        id: usize,
        prompt: Option<String>,
        font: Option<FontRef>,
        font_size: usize,
        color: Color,
        padding: f32,
        do_dpi_scaling: bool,
    ) -> UiRef {
        Self {
            state: State::from_id(id),
            font: font.unwrap_or_default(),
            font_size,
            prompt: prompt.unwrap_or("Enter data".to_string()),
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

    fn display_value(&self) -> &str {
        let state = self.state.get_or_default();
        if state.text.is_empty() {
            &self.prompt
        } else {
            &state.text
        }
    }
}

impl<T: DataInputValue> DataInputState<T> {
    fn do_parsing(&mut self) {
        if !self.changed {
            return;
        }

        let result = T::from_str(&self.text);

        match result {
            Ok(v) => {
                self.value = Some(v);
                self.err = None;
            }
            Err(e) => self.err = Some(e),
        }
    }
}

impl<T: DataInputValue> UiNode for DataInput<T> {
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

        push_scissor(area.to_rect());

        if state.is_active {
            for key in input_text() {
                if let Key::Character(s) = key {
                    state.text.push_str(s.as_str());
                    state.changed = true;
                }

                if let Key::Named(NamedKey::Space) = key {
                    state.text.push(' ');
                    state.changed = true;
                }

                // if let Key::Named(NamedKey::Enter) = key {
                //     state.value.push('\n');
                // }

                if let Key::Named(NamedKey::Backspace) = key {
                    state.text.pop();
                    state.changed = true;
                }
            }
        }

        state.do_parsing();

        let mut inner = area;
        inner.size -= self.extra_size();
        inner.top_left += self.offset();

        let dimensions = if state.text.is_empty() {
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
                &state.text,
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
                if state.text.is_empty() {
                    inner.top_left
                } else {
                    inner.top_left + vec2(dimensions.size.x, 0.0)
                },
                Vec2::new(CURSOR_WIDTH, dimensions.size.y),
                Color::NEUTRAL_200,
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

pub fn data_input_state<T: DataInputValue>(id: usize) -> &'static mut DataInputState<T> {
    State::from_id(id).get_or_default()
}
