#![allow(static_mut_refs)]

use std::time::Instant;

use bevy_math::Mat4;
use bevy_math::Vec2;
use camera::Camera2D;
use camera::Camera3D;
use camera::projection_from_window;
use color::Color;
use config::EngineConfig;
#[cfg(feature = "debugging")]
use debugging::DebugInfo;
pub use draw_queue_2d::Vertex3D;
use egui_glium::{EguiGlium, egui_winit::egui::ViewportId};
use fps_ticker::Fps;
use glium::Program;
use glium::{
    Frame,
    backend::glutin::{Display, SimpleWindowBuilder},
    glutin::surface::WindowSurface,
    winit::{
        event::Event, event_loop::EventLoop, platform::pump_events::EventLoopExtPumpEvents,
        window::Window,
    },
};
use input::Input;
use materials::Material;
use object_3d::Mesh;
use object_3d::Object3D;
use prelude::init_fonts;
use prelude::init_materials;
use programs::init_programs;
use rand::rngs::ThreadRng;
use render_pipeline::RenderPipeline;
use render_pipeline::RenderTexture;
use text_rendering::EngineFont;
use textures::EngineTexture;
use textures::init_textures;
use tunes::engine::AudioEngine;

const BIG_NUMBER: f32 = 9999.9;
const BIGGER_NUMBER: f32 = BIG_NUMBER * 2.0;

mod api;
mod camera;
pub mod collisions;
mod color;
mod config;
#[cfg(feature = "debugging")]
mod debugging;
mod draw_queue_2d;
mod draw_queue_3d;
mod image;
mod input;
mod materials;
mod object_3d;
mod physics;
mod post_processing;
pub mod prelude;
mod programs;
mod render_pipeline;
mod shapes_2d;
mod shapes_3d;
mod slop;
mod text_rendering;
mod textures;
mod utils;

pub(crate) static mut ENGINE_STATE: Option<EngineState> = None;

fn get_state() -> &'static mut EngineState {
    thread_assert::same_thread();

    unsafe { ENGINE_STATE.as_mut().unwrap_or_else(|| panic!()) }
}

type EngineDisplay = Display<WindowSurface>;

struct EngineState {
    window: Window,
    display: EngineDisplay,
    event_loop: EventLoop<()>,
    // bump_allocator: Bump,
    input: Input,
    frame: Option<Frame>,
    /// used for screen-space rendering
    flat_projection: Mat4,
    camera_2d: Camera2D,
    camera_3d: Camera3D,
    gui: EguiGlium,
    audio_engine: AudioEngine,
    gui_initialized: bool,
    render_pipeline: RenderPipeline,
    texture_pipeline: Option<RenderPipeline>,
    #[cfg(feature = "debugging")]
    debug_info: debugging::DebugInfo,
    storage: EngineStorage,
    rng: ThreadRng,
    config: EngineConfig,
    time: f32,
    physics_time: f32,
    is_physics_time_paused: bool,
    frame_count: usize,
    delta_time: f32,
    last_frame_end_time: Instant,
    cursor_position: Vec2,
}

unsafe impl Sync for EngineState {}
unsafe impl Send for EngineState {}

pub(crate) struct EngineStorage {
    textures: Vec<EngineTexture>,
    render_textures: Vec<RenderTexture>,
    programs: Vec<Program>,
    materials: Vec<Material>,
    objects: Vec<Object3D>,
    fonts: Vec<EngineFont>,
    meshes: Vec<Mesh>,
}

impl EngineStorage {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            programs: vec![],
            materials: vec![],
            objects: vec![],
            render_textures: vec![],
            fonts: vec![],
            meshes: vec![],
        }
    }
}

pub fn init(title: &str) -> anyhow::Result<()> {
    env_logger::init();
    color_eyre::install().expect("could not install color_eyre");

    let event_loop = EventLoop::builder().build()?;
    let window_params = Window::default_attributes().with_transparent(false);
    let (window, display) = SimpleWindowBuilder::new()
        .set_window_builder(window_params)
        .with_title(title)
        .build(&event_loop);
    let input = Input::new();
    window.request_redraw();

    let frame = None;

    let flat_projection = projection_from_window(&window);
    let camera_2d = Camera2D::from_window(&window);
    let camera_3d = Camera3D::from_window(&window);
    let gui = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
    #[cfg(feature = "debugging")]
    let debug_info = DebugInfo::new();
    let mut storage = EngineStorage::new();
    init_programs(&display, &mut storage)?;
    init_textures(&mut storage, &display);
    init_materials(&mut storage);
    let rng = rand::rng();
    let gui_initialized = false;
    let config = EngineConfig::default();
    let time = 0.0;
    let delta_time = 0.0;
    let last_frame_end_time = Instant::now();
    let render_pipeline = RenderPipeline::screen();
    let audio_engine = AudioEngine::new()?;
    // let bump_allocator = Bump::new();

    unsafe {
        ENGINE_STATE = Some(EngineState {
            window,
            display,
            texture_pipeline: None,
            event_loop,
            input,
            // bump_allocator,
            frame,
            flat_projection,
            camera_2d,
            camera_3d,
            audio_engine,
            gui,
            gui_initialized,
            debug_info,
            storage,
            rng,
            render_pipeline,
            config,
            time,
            delta_time,
            last_frame_end_time,
            is_physics_time_paused: false,
            cursor_position: Vec2::ZERO,
            frame_count: 0,
            physics_time: 0.0,
        });
    }

    thread_assert::set_thread_id();

    init_fonts();

    Ok(())
}

pub fn next_frame() {
    let state = get_state();

    state.debug_info.next_frame();

    #[allow(deprecated)]
    state
        .event_loop
        .pump_events(None, |event, event_loop_window_target| match event {
            Event::WindowEvent { event, .. } => {
                let gui_response = state.gui.on_event(&state.window, &event);
                if gui_response.consumed {
                    return;
                }

                state.input.process_window_event(&event);

                if state.input.close_requested() {
                    event_loop_window_target.exit();
                }

                if let Some(size) = state.input.window_resized() {
                    state.display.resize(size.into());
                    state.flat_projection = projection_from_window(&state.window);
                    let size = state.window.inner_size();
                    state.camera_2d.update_sizes(size.width, size.height);
                    state.camera_3d.update_sizes(size.width, size.height);
                }
            }
            Event::DeviceEvent { event, .. } => {
                state.input.process_device_event(&event);
            }
            Event::NewEvents(_) => {
                state.input.step();
            }
            _ => (),
        });

    let mut frame = state.frame.take().unwrap_or_else(|| state.display.draw());

    state.render_pipeline.draw_on(&mut frame);
    state.render_pipeline = RenderPipeline::screen();

    if state.gui_initialized {
        state.gui.paint(&state.display, &mut frame);
    }

    frame.finish().unwrap();
    state.window.request_redraw();

    state.frame = Some(state.display.draw());

    let delta_time = state.last_frame_end_time.elapsed().as_secs_f32();
    state.delta_time = delta_time;
    state.time += delta_time;
    state.last_frame_end_time = Instant::now();

    if !state.is_physics_time_paused {
        state.physics_time += delta_time;
    }

    state.frame_count += 1;

    if let Some(c) = state.input.cursor() {
        state.cursor_position = c.into();
    }
}

pub(crate) mod thread_assert {
    static mut THREAD_ID: Option<std::thread::ThreadId> = None;

    pub fn set_thread_id() {
        unsafe {
            THREAD_ID = Some(std::thread::current().id());
        }
    }

    pub fn same_thread() {
        unsafe {
            thread_local! {
                static CURRENT_THREAD_ID: std::thread::ThreadId = std::thread::current().id();
            }
            assert!(THREAD_ID.is_some());
            assert!(THREAD_ID.unwrap() == CURRENT_THREAD_ID.with(|id| *id));
        }
    }
}

impl Drop for EngineState {
    fn drop(&mut self) {
        if let Some(frame) = self.frame.take() {
            let _ = frame.finish();
        }
    }
}

impl EngineState {
    pub(crate) fn window_size(&self) -> Vec2 {
        let size = self.window.inner_size();
        Vec2::new(size.width as f32, size.height as f32)
    }

    pub(crate) fn dpi_scaling(&self) -> f32 {
        self.window.scale_factor() as f32
    }
}
