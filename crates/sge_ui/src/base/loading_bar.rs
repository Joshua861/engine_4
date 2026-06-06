use super::*;
use sge_api::{area::AreaExt, shapes_2d::draw_custom_shape};
use sge_rendering::scissor::{pop_scissor, push_scissor};
use sge_vectors::vec2;

#[derive(Debug)]
pub struct LoadingBar {
    fg: Color,
    bg: Color,
    speed: f32,
    bar_width: f32,
}

impl Default for LoadingBar {
    fn default() -> Self {
        Self {
            fg: Color::NEUTRAL_200,
            bg: Color::NEUTRAL_300,
            speed: 100.0,
            bar_width: 20.0,
        }
    }
}

impl LoadingBar {
    pub fn new_default() -> UiRef {
        Self::default().to_ref()
    }

    pub fn new(fg: Color, bg: Color) -> UiRef {
        Self {
            fg,
            bg,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_with_speed(fg: Color, bg: Color, speed: f32) -> UiRef {
        Self {
            fg,
            bg,
            speed,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn custom(fg: Color, bg: Color, speed: f32, bar_width: f32) -> UiRef {
        Self {
            fg,
            bg,
            speed,
            bar_width,
        }
        .to_ref()
    }
}

impl UiNode for LoadingBar {
    fn preferred_dimensions(&self) -> Vec2 {
        vec2(200.0, 20.0)
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        push_scissor(area.to_glium_rect());
        area.fill(self.bg);

        let pattern_width = self.bar_width;
        let offset = (time() * self.speed) % pattern_width;

        let num_bars = ((area.width() / self.bar_width).ceil() as usize) + 2;

        for i in 0..num_bars {
            let bottom_left_x = area.left() - self.bar_width + offset + (i as f32 * self.bar_width);
            let bottom_right_x = bottom_left_x + self.bar_width * 0.5;
            let top_left_x = bottom_right_x;
            let top_right_x = bottom_left_x + self.bar_width;

            let bottom_y = area.top() + area.height();
            let top_y = area.top();

            let bottom_left = vec2(bottom_left_x, bottom_y);
            let bottom_right = vec2(bottom_right_x, bottom_y);
            let top_left = vec2(top_left_x, top_y);
            let top_right = vec2(top_right_x, top_y);

            draw_custom_shape(&[top_left, top_right, bottom_right, bottom_left], self.fg);
        }

        pop_scissor();
        area.size
    }
}
