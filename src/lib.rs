#![allow(static_mut_refs)]

use std::time::Instant;

use bevy_math::Mat4;
use bevy_math::Vec2;
use buffers::Buffers;
use camera::Camera2D;
use camera::Camera3D;
use camera::projection;
use color::Color;
use config::EngineConfig;
#[cfg(feature = "debugging")]
use debugging::DebugInfo;
use draw_queue_2d::DrawQueue2D;
pub use draw_queue_2d::Vertex3D;
use draw_queue_3d::DrawQueue3D;
use egui_glium::{EguiGlium, egui_winit::egui::ViewportId};
use fps_ticker::Fps;
use glium::Program;
use glium::Surface;
use glium::{
    Frame,
    backend::glutin::{Display, SimpleWindowBuilder},
    glutin::surface::WindowSurface,
    winit::{
        event::Event, event_loop::EventLoop, platform::pump_events::EventLoopExtPumpEvents,
        window::Window,
    },
};
use materials::Material;
use object_3d::Object3D;
use programs::init_programs;
use rand::rngs::ThreadRng;
use sound::{Sound, SoundState};
use textures::EngineTexture;
use textures::init_textures;
use winit_input_helper::WinitInputHelper;

const BIG_NUMBER: f32 = 9999.9;
const BIGGER_NUMBER: f32 = BIG_NUMBER * 2.0;
const SMALL_NUMBER: f32 = 0.001;

mod api;
mod buffers;
mod camera;
pub mod collisions;
mod color;
mod config;
#[cfg(feature = "debugging")]
mod debugging;
mod draw_queue_2d;
mod draw_queue_3d;
mod materials;
mod object_3d;
mod physics;
mod post_processing;
pub mod prelude;
mod programs;
mod shapes_2d;
mod shapes_3d;
mod slop;
mod sound;
mod text_rendering;
mod textures;
mod utils;

pub(crate) static mut ENGINE_STATE: Option<EngineState> = None;

fn get_state() -> &'static mut EngineState {
    thread_assert::same_thread();

    unsafe { ENGINE_STATE.as_mut().unwrap_or_else(|| panic!()) }
}

fn get_frame() -> &'static mut Frame {
    get_state().frame.as_mut().unwrap()
}

type EngineDisplay = Display<WindowSurface>;

struct EngineState {
    window: Window,
    display: EngineDisplay,
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
    frame: Option<Frame>,
    /// used for screen-space rendering
    flat_projection: Mat4,
    camera_2d: Camera2D,
    camera_3d: Camera3D,
    gui: EguiGlium,
    gui_initialized: bool,
    draw_queue: DrawQueue2D,
    world_draw_queue: DrawQueue2D,
    draw_queue_3d: DrawQueue3D,
    #[cfg(feature = "debugging")]
    debug_info: debugging::DebugInfo,
    storage: EngineStorage,
    buffers: Buffers,
    rng: ThreadRng,
    sound: SoundState,
    clear_color: Option<Color>,
    config: EngineConfig,
    time: f32,
    delta_time: f32,
    last_frame_end_time: Instant,
}

unsafe impl Sync for EngineState {}
unsafe impl Send for EngineState {}

pub(crate) struct EngineStorage {
    textures: Vec<EngineTexture>,
    sounds: Vec<Sound>,
    programs: Vec<Program>,
    materials: Vec<Material>,
    objects: Vec<Object3D>,
}

impl EngineStorage {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            sounds: vec![],
            programs: vec![],
            materials: vec![],
            objects: vec![],
        }
    }
}

pub fn init(title: &str) -> anyhow::Result<()> {
    env_logger::init();
    color_eyre::install().expect("could not install color_eyre");

    let event_loop = EventLoop::builder().build()?;
    let (window, display) = SimpleWindowBuilder::new()
        .with_title(title)
        .build(&event_loop);
    let input = WinitInputHelper::new();
    window.request_redraw();

    let frame = None;

    let flat_projection = projection(&window);
    let camera_2d = Camera2D::from_window(&window);
    let camera_3d = Camera3D::from_window(&window);
    let gui = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
    let draw_queue = DrawQueue2D::empty();
    let world_draw_queue = DrawQueue2D::with_z_config(-BIG_NUMBER, 0.01);
    let draw_queue_3d = DrawQueue3D::empty();
    #[cfg(feature = "debugging")]
    let debug_info = DebugInfo::new();
    let mut storage = EngineStorage::new();
    init_programs(&display, &mut storage)?;
    init_textures(&mut storage, &display);
    let buffers = Buffers::new(&display)?;
    let rng = rand::rng();
    let gui_initialized = false;
    let sound = SoundState::new()?;
    let config = EngineConfig::default();
    let time = 0.0;
    let delta_time = 0.0;
    let last_frame_end_time = Instant::now();

    unsafe {
        ENGINE_STATE = Some(EngineState {
            window,
            display,
            event_loop,
            input,
            frame,
            clear_color: None,
            flat_projection,
            camera_2d,
            camera_3d,
            gui,
            gui_initialized,
            draw_queue,
            world_draw_queue,
            draw_queue_3d,
            debug_info,
            buffers,
            storage,
            rng,
            sound,
            config,
            time,
            delta_time,
            last_frame_end_time,
        });
    }

    thread_assert::set_thread_id();

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
                    state.flat_projection = projection(&state.window);
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

    if let Some(c) = state.clear_color {
        frame.clear_color(c.r, c.g, c.b, c.a);
    }

    frame.clear_depth(1.0);

    let view_proj = state.camera_3d.view_proj();
    state.draw_queue_3d.draw(&mut frame, &view_proj);

    state
        .world_draw_queue
        .draw(&mut frame, &state.camera_2d.projection_matrix());
    state.draw_queue.draw(&mut frame, &state.flat_projection);

    state.draw_queue.clear();
    state.world_draw_queue.clear();

    if state.gui_initialized {
        state.gui.paint(&state.display, &mut frame);
    }

    frame.finish().unwrap();
    state.window.request_redraw();

    state.frame = Some(state.display.draw());

    let now = Instant::now();
    let delta_time = state.last_frame_end_time.elapsed().as_secs_f32();
    state.delta_time = delta_time;
    state.time += delta_time;
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
}
