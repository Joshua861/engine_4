use sge_animation::lerp;
use sge_vectors::vec2;

use super::*;

#[derive(Debug)]
pub struct ProgressBar {
    pub(crate) max: f32,
    pub(crate) value: f32,
    pub(crate) fill: Child,
    pub(crate) bg: Child,
    pub(crate) state: State<ProgressBarState>,
    pub(crate) interpolation_speed: f32,
}

#[derive(Default, Clone, Copy, Debug)]
pub(crate) struct ProgressBarState {
    displayed_value: f32,
}

impl ProgressBar {
    pub fn new(value: f32, max: f32, fill: UiRef, bg: Child, id: usize) -> UiRef {
        ProgressBar {
            max,
            value,
            fill,
            bg,
            state: State::from_id(id),
            interpolation_speed: 20.0,
        }
        .to_ref()
    }

    /// default is 10
    pub fn new_with_interpolation_speed(
        value: f32,
        max: f32,
        fill: UiRef,
        bg: Child,
        interpolation_speed: f32,
        id: usize,
    ) -> UiRef {
        ProgressBar {
            max,
            value,
            fill,
            bg,
            state: State::from_id(id),
            interpolation_speed,
        }
        .to_ref()
    }
}

impl UiNode for ProgressBar {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state.get_or_default();

        state.displayed_value = lerp(
            state.displayed_value,
            self.value,
            self.interpolation_speed * ui.delta_time,
        );
        let ratio = state.displayed_value / self.max;
        let width = area.width() * ratio;

        let fill_area = Area::new(area.top_left, vec2(width, area.height()));
        self.fill.draw(fill_area, ui);

        let bg_area = Area::new(
            area.top_left + vec2(width, 0.0),
            vec2(area.width() - width, area.height()),
        );
        self.bg.draw(bg_area, ui);

        area.size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::new(200.0, 20.0)
    }
}
