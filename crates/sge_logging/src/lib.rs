use log::{Level, LevelFilter, Log};
use sge_color::Color;
use sge_types::{Area, Verbosity};
use sge_vectors::Vec2;
use sge_window::window_size;
use std::fs::File;

use sge_text::rich_text::{RichText, RichTextBlock};

pub(crate) struct UnitLogger;
static LOGGER: UnitLogger = UnitLogger;

pub(crate) static mut LOG_STATE: Option<Logger> = None;

#[allow(static_mut_refs)]
pub fn get_logger() -> &'static mut Logger {
    unsafe { LOG_STATE.as_mut().unwrap_or_else(|| panic!()) }
}

pub struct Logger {
    pub min_log_level: LevelFilter,
    pub output_file: Option<File>,
    pub verbosity: Verbosity,
    /// how many lines to draw
    pub draw_lines: usize,
    pub lines: Vec<RichText>,
}

pub fn draw_logs() {
    get_logger().draw();
}

pub fn log_lines() -> &'static mut [RichText] {
    &mut get_logger().lines
}

pub fn set_min_log_level(level: LevelFilter) {
    get_logger().set_min_log_level(level);
}

pub fn set_logger_verbosity(verbosity: Verbosity) {
    get_logger().verbosity = verbosity;
}

pub fn set_max_drawn_log_lines(lines: usize) {
    get_logger().draw_lines = lines;
}

pub fn init() -> Result<(), log::SetLoggerError> {
    unsafe {
        LOG_STATE = Some(Logger::default());
    }

    log::set_logger(&LOGGER).map(|()| log::set_max_level(get_logger().min_log_level))?;
    log::info!("Initialized logger");
    Ok(())
}

pub fn log_to_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    get_logger().output_file = Some(file);
    Ok(())
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            min_log_level: LevelFilter::Debug,
            output_file: None,
            verbosity: Verbosity::Medium,
            draw_lines: 10,
            lines: vec![],
        }
    }
}

impl Logger {
    fn draw(&self) {
        let mut cursor = Vec2::ZERO;

        for i in 0..(self.draw_lines.min(self.lines.len())) {
            let i = if self.lines.len() >= self.draw_lines {
                i + (self.lines.len() - self.draw_lines)
            } else {
                i
            };

            let line = &self.lines[i];

            let dimensions = line.draw(Area::new(cursor, window_size()), 1.0);
            cursor.y += dimensions.size.y;
        }
    }

    fn set_min_log_level(&mut self, level: LevelFilter) {
        self.min_log_level = level;
        log::set_max_level(self.min_log_level);
    }

    fn color(level: Level) -> Color {
        match level {
            Level::Debug => Color::NEUTRAL_400,
            Level::Error => Color::RED_500,
            Level::Warn => Color::YELLOW_500,
            Level::Info => Color::SKY_500,
            Level::Trace => Color::NEUTRAL_600,
        }
    }

    fn text(level: Level) -> &'static str {
        match level {
            Level::Debug => "[DEBUG]",
            Level::Error => "[ERROR]",
            Level::Warn => "[WARN] ",
            Level::Info => "[INFO] ",
            Level::Trace => "[TRACE]",
        }
    }

    fn log(&mut self, record: &log::Record) {
        if !record.file().is_some_and(|f| f.contains("winit")) || record.level() > Level::Debug {
            let line = self.format_log(record);
            line.print_to_stdout();
            self.lines.push(line);
        }
    }

    fn format_log(&self, record: &log::Record) -> RichText {
        let s = record.args().to_string();
        let color = Self::color(record.level());
        let level_text = Self::text(record.level());

        match self.verbosity {
            Verbosity::Low => RichText::new(vec![RichTextBlock::from_color(s, color).with_mono()]),
            Verbosity::Medium => RichText::new(vec![
                RichTextBlock::from_color(level_text, color).with_mono(),
                RichTextBlock::from_color(format!(" {s}"), Color::WHITE).with_mono(),
            ]),
            Verbosity::High => RichText::new(vec![
                RichTextBlock::from_color(
                    format!(
                        "[{}::{}:{}]",
                        record.module_path().unwrap_or("??"),
                        record.file().unwrap_or("??"),
                        record.line().unwrap_or(0)
                    ),
                    Color::NEUTRAL_500,
                )
                .with_mono(),
                RichTextBlock::from_color(level_text, color).with_mono(),
                RichTextBlock::from_color(format!(" {s}"), Color::WHITE).with_mono(),
            ]),
        }
    }
}

impl UnitLogger {
    fn state() -> &'static mut Logger {
        get_logger()
    }
}

impl Log for UnitLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Self::state().min_log_level
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        let state = Self::state();

        if self.enabled(record.metadata()) {
            state.log(record);
        }
    }
}
