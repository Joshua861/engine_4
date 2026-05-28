use sge_api::shapes_2d::{draw_line_to, draw_rect_outline_to, draw_rect_to};
use sge_color::Color;
use sge_rendering::d2::Renderer2D;
use sge_vectors::{Vec2, vec2};

use crate::{TextDimensions, TextDrawParams, draw_text_to};

use super::{RichText, RichTextParams, RichTextStyle, layout::LayoutLine};

impl RichText {
    pub fn draw_layout_to(
        &self,
        lines: &[LayoutLine],
        origin: Vec2,
        params: RichTextParams,
        renderer: Renderer2D,
    ) -> TextDimensions {
        let RichTextParams {
            line_spacing,
            do_dpi_scaling,
        } = params;

        let mut y = 0.0;
        let mut total_width: f32 = 0.0;

        for line in lines {
            let line_height = line.max_font_size as f32 * line_spacing;
            let mut x = 0.0;

            let baseline_y = origin.y + y + line.max_font_size as f32;
            let underline_y = baseline_y + line.max_font_size as f32 * 0.08;
            let strike_y = origin.y + y + line_height * 0.7;

            let thickness = (line.max_font_size as f32 * 0.06).max(1.0);

            let mut highlights = Vec::new();
            let mut underlines = Vec::new();
            let mut strikethroughs = Vec::new();
            let mut outlines = Vec::new();
            let mut text_elements = Vec::new();

            let mut tracker = State::raw_line_start();

            for word in &line.words {
                x += word.space_before;
                let start_x = origin.x + x;
                let end_x = start_x + word.width;

                tracker.update(
                    start_x,
                    end_x,
                    &word.style,
                    &mut highlights,
                    &mut underlines,
                    &mut strikethroughs,
                    &mut outlines,
                );

                let baseline_offset = (line.max_font_size - word.style.font_size) as f32;
                let pos = origin + vec2(x, y + baseline_offset);
                text_elements.push((word, pos));

                x += word.width;
            }

            tracker.flush_remaining(
                &mut highlights,
                &mut underlines,
                &mut strikethroughs,
                &mut outlines,
            );

            for run in highlights {
                draw_rect_to(
                    vec2(run.start, origin.y + y),
                    vec2(run.end - run.start, line_height),
                    run.color,
                    renderer,
                );
            }

            for (word, pos) in text_elements {
                draw_text_to(
                    &word.text,
                    TextDrawParams {
                        font: Some(word.style.font()),
                        font_size: word.style.font_size,
                        color: word.style.color,
                        position: pos,
                        do_dpi_scaling,
                    },
                    renderer,
                );
            }

            for run in underlines {
                draw_line_to(
                    vec2(run.start, underline_y),
                    vec2(run.end, underline_y),
                    thickness,
                    run.color,
                    renderer,
                );
            }
            for run in strikethroughs {
                draw_line_to(
                    vec2(run.start, strike_y),
                    vec2(run.end, strike_y),
                    thickness,
                    run.color,
                    renderer,
                );
            }
            for run in outlines {
                draw_rect_outline_to(
                    vec2(run.start, origin.y + y),
                    vec2(run.end - run.start, line_height),
                    thickness,
                    run.color,
                    renderer,
                );
            }

            total_width = total_width.max(x);
            y += line_height;
        }

        let size = Vec2::new(total_width, y);
        TextDimensions {
            size,
            final_cursor_pos: size,
        }
    }
}

struct Run {
    start: f32,
    end: f32,
    color: Color,
}

struct State {
    highlight: Option<(f32, f32, Color)>,
    underline: Option<(f32, f32, Color)>,
    strike: Option<(f32, f32, Color)>,
    outline: Option<(f32, f32, Color)>,
}

impl State {
    fn raw_line_start() -> Self {
        Self {
            highlight: None,
            underline: None,
            strike: None,
            outline: None,
        }
    }

    fn update(
        &mut self,
        start_x: f32,
        end_x: f32,
        style: &RichTextStyle,
        highlights: &mut Vec<Run>,
        underlines: &mut Vec<Run>,
        strikethroughs: &mut Vec<Run>,
        outlines: &mut Vec<Run>,
    ) {
        Self::track_run(
            &mut self.highlight,
            style.highlight,
            start_x,
            end_x,
            highlights,
        );
        Self::track_run(
            &mut self.underline,
            style.underline,
            start_x,
            end_x,
            underlines,
        );
        Self::track_run(
            &mut self.strike,
            style.strikethrough,
            start_x,
            end_x,
            strikethroughs,
        );
        Self::track_run(&mut self.outline, style.outline, start_x, end_x, outlines);
    }

    fn flush_remaining(
        self,
        highlights: &mut Vec<Run>,
        underlines: &mut Vec<Run>,
        strikethroughs: &mut Vec<Run>,
        outlines: &mut Vec<Run>,
    ) {
        if let Some((start, end, color)) = self.highlight {
            highlights.push(Run { start, end, color });
        }
        if let Some((start, end, color)) = self.underline {
            underlines.push(Run { start, end, color });
        }
        if let Some((start, end, color)) = self.strike {
            strikethroughs.push(Run { start, end, color });
        }
        if let Some((start, end, color)) = self.outline {
            outlines.push(Run { start, end, color });
        }
    }

    fn track_run(
        active_run: &mut Option<(f32, f32, Color)>,
        current_style: Option<Color>,
        start_x: f32,
        end_x: f32,
        run_list: &mut Vec<Run>,
    ) {
        match (*active_run, current_style) {
            (Some((start, _, active_color)), Some(new_color)) if active_color == new_color => {
                *active_run = Some((start, end_x, active_color));
            }

            (Some((start, last_end, active_color)), _) => {
                run_list.push(Run {
                    start,
                    end: last_end,
                    color: active_color,
                });
                *active_run = current_style.map(|c| (start_x, end_x, c));
            }

            (None, Some(new_color)) => {
                *active_run = Some((start_x, end_x, new_color));
            }

            _ => {}
        }
    }
}
