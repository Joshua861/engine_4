use egui_glium::egui_winit::egui::Window;
use egui_plot::{Line, Plot, PlotPoints};

use crate::{Fps, get_state};

pub mod grid;

const FRAME_BACKLOG: usize = 240;

pub struct DebugInfo {
    pub fps: Fps,
    pub frame_offset: usize,
    pub frames: [FrameInfo; FRAME_BACKLOG],
    pub show_window: bool,
    pub max: FrameInfo,
}

#[derive(Clone, Copy)]
pub struct FrameInfo {
    pub vertex_count: usize,
    pub index_count: usize,
    pub draw_calls: usize,
    pub drawn_objects: usize,
}

impl FrameInfo {
    pub const ZERO: Self = Self {
        vertex_count: 0,
        index_count: 0,
        draw_calls: 0,
        drawn_objects: 0,
    };
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            fps: Fps::default(),
            frame_offset: 0,
            frames: [FrameInfo::ZERO; FRAME_BACKLOG],
            max: FrameInfo::ZERO,
            show_window: false,
        }
    }

    pub fn next_frame(&mut self) {
        let current_frame = self.current_frame().clone();
        self.max.index_count = self.max.index_count.max(current_frame.index_count);
        self.max.vertex_count = self.max.vertex_count.max(current_frame.vertex_count);
        self.max.draw_calls = self.max.draw_calls.max(current_frame.draw_calls);
        self.max.drawn_objects = self.max.drawn_objects.max(current_frame.drawn_objects);

        self.frame_offset = (self.frame_offset + 1) % FRAME_BACKLOG;
        self.fps.tick();
        self.frames[self.frame_offset] = FrameInfo::ZERO;
    }

    pub fn current_frame(&self) -> &FrameInfo {
        &self.frames[self.frame_offset]
    }

    pub(crate) fn current_frame_mut(&mut self) -> &mut FrameInfo {
        &mut self.frames[self.frame_offset]
    }

    pub(crate) fn draw_debug_info(&mut self, ui: &egui_glium::egui_winit::egui::Context) {
        if !self.show_window {
            return;
        }

        let vertex_points: PlotPoints = (0..FRAME_BACKLOG)
            .map(|i| {
                let frame = &self.frames[(i + self.frame_offset) % FRAME_BACKLOG];
                [i as f64, frame.vertex_count as f64]
            })
            .collect();
        let index_points: PlotPoints = (0..FRAME_BACKLOG)
            .map(|i| {
                let frame = &self.frames[(i + self.frame_offset) % FRAME_BACKLOG];
                [i as f64, frame.index_count as f64]
            })
            .collect();
        let draw_call_points: PlotPoints = (0..FRAME_BACKLOG)
            .map(|i| {
                let frame = &self.frames[(i + self.frame_offset) % FRAME_BACKLOG];
                [i as f64, frame.draw_calls as f64]
            })
            .collect();
        let drawn_object_points: PlotPoints = (0..FRAME_BACKLOG)
            .map(|i| {
                let frame = &self.frames[(i + self.frame_offset) % FRAME_BACKLOG];
                [i as f64, frame.drawn_objects as f64]
            })
            .collect();

        let vertex_line = Line::new(vertex_points);
        let index_line = Line::new(index_points);
        let draw_call_line = Line::new(draw_call_points);
        let drawn_object_line = Line::new(drawn_object_points);

        Window::new("Debug info").show(ui, |ui| {
            for (id, max, label, line) in [
                (
                    "vertex_plot",
                    self.max.vertex_count,
                    "Vertex count",
                    vertex_line,
                ),
                (
                    "index_plot",
                    self.max.index_count,
                    "Indice count",
                    index_line,
                ),
                (
                    "draw_plot",
                    self.max.draw_calls,
                    "Draw call count",
                    draw_call_line,
                ),
                (
                    "object_plot",
                    self.max.drawn_objects,
                    "Drawn object count",
                    drawn_object_line,
                ),
            ] {
                Plot::new(id)
                    .height(100.0)
                    .include_y(max as f64 * 1.5)
                    .include_y(0.0)
                    .allow_scroll(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .y_axis_label(label)
                    .show(ui, |ui| ui.line(line));
            }

            ui.label(&format!("FPS: {:.1}", self.fps.avg()));
        });
    }
}

pub fn avg_fps() -> f64 {
    get_state().debug_info.fps.avg()
}

pub fn min_fps() -> f64 {
    get_state().debug_info.fps.min()
}

pub fn max_fps() -> f64 {
    get_state().debug_info.fps.max()
}

pub fn get_debug_info() -> &'static DebugInfo {
    &get_state().debug_info
}

pub(crate) fn get_debug_info_mut() -> &'static mut DebugInfo {
    &mut get_state().debug_info
}

pub fn show_debug_info() {
    get_debug_info_mut().show_window = true;
}

pub fn hide_debug_info() {
    get_debug_info_mut().show_window = false;
}

pub fn set_show_debug_info(to: bool) {
    get_debug_info_mut().show_window = to;
}
