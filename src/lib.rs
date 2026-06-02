#![allow(static_mut_refs)]
use std::pin::Pin;
use std::time::Instant;

use glium::winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ActiveEventLoop,
    platform::pump_events::EventLoopExtPumpEvents,
};
use log::{error, info};
use sge_config::{Opts, get_config};
#[cfg(feature = "debugging")]
use sge_debugging::get_debug_info;
#[cfg(feature = "egui")]
use sge_egui::{egui, get_egui_state};
use sge_error_union::ErrorUnion;
use sge_exec::FrameFuture;
#[cfg(feature = "input")]
use sge_input::get_input;
pub use sge_math::collision;
use sge_rendering::{get_render_state, pipeline::RenderPipeline};
pub use sge_shapes::d2 as shapes_2d;

const WAIT_FOR_EVENTS_EXTRA_FRAME_DRAWS: usize = 60 * 5; // stops rendering after 300 frames of no input, when config.wait_for_events is true

pub use modules::*;
use sge_time::frames_since_input;
use sge_window::{get_display, get_window_state, window_size_u32};

pub mod api;
mod modules;
pub mod prelude;
mod user_storage;

pub fn init(title: impl AsRef<str>) -> Result<(), InitError> {
    let opts = Opts::builder().title(title.as_ref().to_string()).build();
    init_custom(opts)
}

#[derive(ErrorUnion, Debug)]
pub enum InitError {
    SetLogger(log::SetLoggerError),
    WindowCreation(sge_window::WindowCreationError),
    Program(glium::ProgramCreationError),
    #[cfg(feature = "audio")]
    Audio(sge_audio::AudioInitError),
    #[cfg(feature = "input")]
    Input(sge_input::InputError),
    #[cfg(feature = "text")]
    LoadFont(sge_text::LoadFontError),
}

#[allow(unused_mut)]
pub fn init_custom(mut opts: Opts) -> Result<(), InitError> {
    let opts = opts.build();
    sge_logging::init()?;
    sge_logging::get_logger().min_log_level = opts.min_log_level;
    sge_logging::get_logger().verbosity = opts.log_verbosity;

    match sge_window::init(opts.window) {
        Ok(output) => {
            info!("Initialized window, display, event loop.");
            output
        }
        Err(e) => {
            error!("FATAL: Could not initialize window: {e}");
            return Err(e.into());
        }
    }

    let size = window_size_u32();

    sge_time::init();
    sge_config::init(opts.config);
    get_window_state().window.request_redraw();
    #[cfg(feature = "input")]
    sge_input::init()?;
    sge_camera::init(size.x, size.y, false);
    #[cfg(feature = "egui")]
    sge_egui::init();
    #[cfg(feature = "debugging")]
    sge_debugging::init();
    sge_rng::init();
    sge_programs::init()?;
    #[cfg(feature = "audio")]
    sge_audio::init()?;
    sge_image::init();
    sge_texture_atlas::init();
    sge_textures::init();
    sge_rendering::init();
    #[cfg(feature = "text")]
    sge_text::init()?;
    #[cfg(feature = "ui")]
    sge_ui::init_ui();
    user_storage::init();
    sge_physics::init();
    sge_exec::init();
    #[cfg(feature = "multiplayer")]
    sge_multiplayer::init();

    info!("Finished initializing engine.");

    Ok(())
}

pub fn run_async(future: impl Future<Output = ()> + 'static) {
    let mut future: Pin<Box<dyn Future<Output = ()>>> = Box::pin(future);
    loop {
        if sge_exec::poll_once(&mut future).is_some() {
            break;
        }
        next_frame_sync();
    }
}

pub async fn next_frame() {
    FrameFuture::default().await;
}

fn next_frame_sync() {
    let engine_start_time = Instant::now();

    let has_input_event = process_events();

    #[cfg(feature = "input")]
    sge_input::update();
    #[cfg(feature = "ui")]
    sge_ui::update();
    render_frame();
    #[cfg(feature = "egui")]
    sge_egui::update();
    sge_window::end_of_frame();
    sge_time::update(has_input_event);
    record_frame_time(engine_start_time);
    request_redraw_if_needed();
    sge_exec::get_executor().tick();
}

fn process_events() -> bool {
    let mut has_input_event = false;
    #[cfg(feature = "input")]
    let input = get_input();
    let state = get_window_state();

    #[allow(deprecated)]
    state
        .event_loop
        .pump_events(None, |event, event_loop_window_target| match event {
            Event::WindowEvent { event, .. } => {
                has_input_event |= handle_window_event(&event, event_loop_window_target);
            }
            Event::DeviceEvent { event, .. } => {
                #[cfg(feature = "input")]
                input.process_device_event(&event);
                has_input_event = true;
            }
            Event::NewEvents(_) => {
                #[cfg(feature = "input")]
                input.step();
            }
            _ => (),
        });

    has_input_event
}

fn handle_window_event(event: &WindowEvent, event_loop_window_target: &ActiveEventLoop) -> bool {
    #[cfg(feature = "egui")]
    let egui = egui();
    #[cfg(feature = "input")]
    let input = get_input();
    #[allow(unused)]
    let window = &get_window_state().window;
    #[cfg(feature = "egui")]
    let gui_response = egui.on_event(window, event);
    #[cfg(feature = "egui")]
    if gui_response.consumed {
        return false;
    }

    #[cfg(feature = "input")]
    let is_input_event = !input.process_window_event(event);

    #[cfg(feature = "input")]
    if input.close_requested() {
        event_loop_window_target.exit();
    }

    #[cfg(feature = "input")]
    if let Some(size) = input.window_resized() {
        handle_window_resize(size);
    }

    #[cfg(not(feature = "input"))]
    if let WindowEvent::Resized(size) = event {
        handle_window_resize(*size);
    }

    #[cfg(feature = "input")]
    return is_input_event;
    #[cfg(not(feature = "input"))]
    return true;
}

fn handle_window_resize(size: PhysicalSize<u32>) {
    sge_window::handle_window_resize(size);
    sge_camera::update_cameras_on_resize(size.width, size.height);
}

fn render_frame() {
    let display = get_display();
    let pipeline = &mut get_render_state().render_pipeline;

    let mut frame = display.draw();
    pipeline.draw_on(&mut frame);
    *pipeline = RenderPipeline::screen();

    #[cfg(feature = "egui")]
    {
        let state = get_egui_state();
        if state.initialized {
            state.egui.paint(get_display(), &mut frame);
        }
    }

    frame.finish().unwrap();
}

fn request_redraw_if_needed() {
    let config = get_config();
    let window = get_window_state();
    if !config.wait_for_events || frames_since_input() < WAIT_FOR_EVENTS_EXTRA_FRAME_DRAWS {
        window.window.request_redraw();
    }
}

fn record_frame_time(engine_start_time: Instant) {
    #[cfg(feature = "debugging")]
    {
        let debug_info = get_debug_info();
        debug_info.next_frame();
        let engine_time = engine_start_time.elapsed();
        debug_info.current_frame_mut().engine_time = engine_time.as_micros() as f64 / 1000.0;
    }
}
