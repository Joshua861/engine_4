use crate::get_ui_textures;
use bon::bon;
use sge_api::shapes_2d::{draw_rect, draw_rect_with_outline};
use sge_input::{cursor_diff, last_cursor_pos, mouse_held, mouse_pressed, mouse_released};
use sge_math::transform::Transform2D;
use sge_rendering::{
    api::draw_texture_ex,
    scissor::{pop_scissor, push_scissor},
};
use sge_vectors::{FloatPow, vec2};
use sge_window::{use_move_cursor_icon, use_nwse_resize_cursor_icon, use_pointer_cursor_icon};

use super::*;

#[derive(Debug)]
pub struct FloatingWindow {
    resizable: bool,
    closable: bool,
    movable: bool,
    default_open: bool,
    default_size: Vec2,
    default_position: Vec2,
    default_minimised: bool,
    contents: Child,
    title: Child,
    state: State<FloatingWindowState>,
    resize_cursor_radius: f32,
    border: BorderStyle,
    padding: f32,
    bg: Color,
    button_color: Color,
    active_button_color: Color,
}

#[derive(Debug)]
pub struct FloatingWindowState {
    pub inner_size: Vec2,
    pub open: bool,
    pub position: Vec2,
    pub minimised: bool,
    pub top_bar_captured: bool,
    pub resize_captured: bool,
}

#[bon]
impl FloatingWindow {
    #[builder]
    pub fn builder(
        contents: Child,
        title: Child,
        border: Option<BorderStyle>,
        closable: Option<bool>,
        default_minimised: Option<bool>,
        default_open: Option<bool>,
        default_position: Option<Vec2>,
        default_size: Option<Vec2>,
        movable: Option<bool>,
        padding: Option<f32>,
        resizable: Option<bool>,
        resize_cursor_radius: Option<f32>,
        bg: Option<Color>,
        button_color: Option<Color>,
        active_button_color: Option<Color>,
        id: usize,
    ) -> UiRef {
        Self {
            contents,
            title,
            border: border.unwrap_or_default(),
            closable: closable.unwrap_or(true),
            default_minimised: default_minimised.unwrap_or(false),
            default_open: default_open.unwrap_or(true),
            default_position: default_position.unwrap_or(Vec2::splat(20.0)),
            default_size: default_size.unwrap_or(contents.preferred_dimensions()),
            movable: movable.unwrap_or(true),
            padding: padding.unwrap_or(10.0),
            resizable: resizable.unwrap_or(true),
            resize_cursor_radius: resize_cursor_radius.unwrap_or(20.0),
            bg: bg.unwrap_or(Color::NEUTRAL_900),
            button_color: button_color.unwrap_or(Color::NEUTRAL_300),
            active_button_color: active_button_color.unwrap_or(Color::NEUTRAL_100),
            state: State::from_id(id),
        }
        .to_ref()
    }

    fn state(&self) -> &'static mut FloatingWindowState {
        self.state.get_or_create_mut(|| FloatingWindowState {
            inner_size: self.default_size,
            open: self.default_open,
            position: self.default_position,
            minimised: self.default_minimised,
            top_bar_captured: false,
            resize_captured: false,
        })
    }

    const TOP_BAR_HEIGHT: f32 = 24.0;

    fn draw_top_bar(&self, state: &mut FloatingWindowState, ui: &UiState) -> Vec2 {
        let width = state.inner_size.x;
        let height = Self::TOP_BAR_HEIGHT + 2.0 * self.border.thickness;
        let size = vec2(width, height);
        let mut area = Area::new(state.position, size);

        // outline
        draw_rect_with_outline(
            area.top_left,
            area.size + vec2(0.0, 2.0),
            self.bg,
            self.border.thickness,
            self.border.color,
        );

        let border_size = Vec2::splat(self.border.thickness);
        area.size -= border_size * 2.0;
        area.top_left += border_size;

        // buttons
        if self.closable {
            let button_height = Self::TOP_BAR_HEIGHT;
            let button_block_width = (button_height + self.border.thickness) * 2.0;
            let button_top_left = area.top_right() - vec2(button_block_width, 0.0);
            let button_size = vec2(button_block_width, button_height);
            let mut button_area = Area::new(button_top_left, button_size);

            draw_rect(
                button_area.top_left - vec2(0.0, self.border.thickness),
                vec2(
                    self.border.thickness,
                    button_height + self.border.thickness * 2.0,
                ),
                self.border.color,
            );

            button_area.top_left += vec2(self.border.thickness, 0.0);

            let minimise_area = Area::new(button_area.top_left, Vec2::splat(button_height));
            let color = if ui.is_hovered(minimise_area) {
                self.active_button_color
            } else {
                self.button_color
            };
            draw_texture_ex(
                get_ui_textures().minimise,
                Transform2D::new()
                    .with_translation(button_area.top_left)
                    .with_scale(Vec2::splat(button_height)),
                color,
                None,
            );

            if ui.is_clicked(minimise_area) {
                state.minimised = !state.minimised;
            }

            if ui.is_hovered(minimise_area) {
                use_pointer_cursor_icon();
            }

            button_area.top_left += vec2(button_height, 0.0);

            draw_rect(
                button_area.top_left - vec2(0.0, self.border.thickness),
                vec2(
                    self.border.thickness,
                    button_height + self.border.thickness * 2.0,
                ),
                self.border.color,
            );

            button_area.top_left += vec2(self.border.thickness, 0.0);

            let close_area = Area::new(button_area.top_left, Vec2::splat(button_height));
            let color = if ui.is_hovered(close_area) {
                self.active_button_color
            } else {
                self.button_color
            };
            draw_texture_ex(
                get_ui_textures().close,
                Transform2D::new()
                    .with_translation(button_area.top_left)
                    .with_scale(Vec2::splat(button_height)),
                color,
                None,
            );

            if ui.is_clicked(close_area) {
                state.open = false;
            }

            if ui.is_hovered(close_area) {
                use_pointer_cursor_icon();
            }

            area.size.x -= button_block_width;
        }

        push_scissor(area.to_rect());
        area.top_left.x += self.padding;
        Center::vertical(self.title).draw(area, ui);
        pop_scissor();

        vec2(area.size.x, size.y)
    }

    fn draw_contents(&self, ui: &UiState, area: Area) {
        draw_rect_with_outline(
            area.top_left,
            area.size,
            self.bg,
            self.border.thickness,
            self.border.color,
        );
        push_scissor(area.to_rect());
        self.contents.draw(area.with_padding(self.padding), ui);
        pop_scissor();
    }

    fn do_dragging(&self, mut top_bar_area: Area, ui: &UiState, state: &mut FloatingWindowState) {
        if mouse_pressed(MouseButton::Left) {
            state.top_bar_captured = ui.is_hovered(top_bar_area);
        }

        if mouse_released(MouseButton::Left) {
            state.top_bar_captured = false;
        }

        if state.top_bar_captured {
            let delta = cursor_diff();
            state.position += delta;
            top_bar_area.top_left += delta;
        }

        if ui.is_hovered(top_bar_area) {
            use_move_cursor_icon();
        }
    }

    fn do_resizing(&self, state: &mut FloatingWindowState, mut bottom_right: Vec2) {
        let radius_squared = self.resize_cursor_radius.squared();
        let delta = bottom_right - last_cursor_pos();
        let dist_squared = delta.length_squared();

        if radius_squared > dist_squared {
            use_nwse_resize_cursor_icon();
        }

        if mouse_pressed(MouseButton::Left) {
            state.resize_captured = radius_squared > dist_squared;
        }

        if mouse_released(MouseButton::Left) {
            state.resize_captured = false;
        }

        if state.resize_captured && mouse_held(MouseButton::Left) {
            let delta = last_cursor_pos() - bottom_right;
            let before = state.inner_size;
            state.inner_size += delta;
            state.inner_size = state.inner_size.clamp(Vec2::new(100.0, 0.0), window_size());
            bottom_right += state.inner_size - before;
        }
    }

    pub(crate) fn actually_draw(&self, ui: &UiState) {
        let state = self.state();

        if !state.open {
            return;
        }

        let pos = state.position;

        let mut cursor = pos;
        let top_bar_size = self.draw_top_bar(state, ui);
        cursor.y += top_bar_size.y;

        if self.movable {
            let top_bar_area = Area::new(state.position, top_bar_size);
            self.do_dragging(top_bar_area, ui, state);
        }

        if !state.minimised {
            let inner_area = Area::new(cursor, state.inner_size);
            self.draw_contents(ui, inner_area);
            cursor += state.inner_size;
        }

        if self.resizable {
            self.do_resizing(state, cursor);
        }

        // check if the cursor is anywhere over the window or titlebar
        let window_area = Area::new(state.position, cursor - state.position);
        if ui.is_hovered(window_area) {
            ui.consume_input();
        }
    }
}

impl UiNode for FloatingWindow {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn size(&self, _: Area) -> Vec2 {
        Vec2::ZERO
    }

    //  we dont actually draw here because we need this to be on top of other ui stuff, so we just add
    //  it to the ui state windows thingy to draw at the end of the frame
    fn draw(&self, _: Area, _: &UiState) -> Vec2 {
        get_ui_storage().windows.push(self as *const Self);

        Vec2::ZERO
    }
}

pub fn floating_window_state(id: usize) -> Option<&'static mut FloatingWindowState> {
    State::from_id(id).get_mut()
}
