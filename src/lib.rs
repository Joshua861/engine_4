#![allow(static_mut_refs)]
#![allow(unused)]

use bevy_math::Mat4;
use buffers::Buffers;
use camera::Camera;
use camera::projection;
use color::Color;
use config::EngineConfig;
#[cfg(feature = "debugging")]
use debugging::DebugInfo;
use draw_queue::DrawQueue;
pub use draw_queue::Vertex3D;
use egui_glium::{EguiGlium, egui_winit::egui::ViewportId};
use fps_ticker::Fps;
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
use programs::Programs;
use rand::rngs::ThreadRng;
use sound::{Sound, SoundState};
use textures::EngineTexture;
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
mod draw_queue;
mod physics;
mod post_processing;
pub mod prelude;
mod programs;
mod shapes_2d;
mod shapes_3d;
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
    programs: Programs,
    window: Window,
    display: EngineDisplay,
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
    frame: Option<Frame>,
    projection: Mat4,
    camera: Camera,
    gui: EguiGlium,
    gui_initialized: bool,
    draw_queue: DrawQueue,
    world_draw_queue: DrawQueue,
    draw_queue_3d: DrawQueue,
    #[cfg(feature = "debugging")]
    debug_info: debugging::DebugInfo,
    storage: EngineStorage,
    buffers: Buffers,
    rng: ThreadRng,
    sound: SoundState,
    clear_color: Option<Color>,
    config: EngineConfig,
}

unsafe impl Sync for EngineState {}
unsafe impl Send for EngineState {}

pub(crate) struct EngineStorage {
    textures: Vec<EngineTexture>,
    sounds: Vec<Sound>,
}

impl EngineStorage {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            sounds: vec![],
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

    let projection = projection(&window);
    let camera = Camera::from_window(&window);
    let gui = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
    let draw_queue = DrawQueue::empty();
    let world_draw_queue = DrawQueue::with_z_config(-BIG_NUMBER, 0.01);
    let draw_queue_3d = DrawQueue::empty();
    #[cfg(feature = "debugging")]
    let debug_info = DebugInfo::new();
    let textures = EngineStorage::new();
    let buffers = Buffers::new(&display)?;
    let programs = Programs::new(&display)?;
    let rng = rand::rng();
    let gui_initialized = false;
    let sound = SoundState::new()?;
    let config = EngineConfig::default();

    unsafe {
        ENGINE_STATE = Some(EngineState {
            programs,
            window,
            display,
            event_loop,
            input,
            frame,
            clear_color: None,
            projection,
            camera,
            gui,
            gui_initialized,
            draw_queue,
            world_draw_queue,
            draw_queue_3d,
            debug_info,
            buffers,
            storage: textures,
            rng,
            sound,
            config,
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
                    state.projection = projection(&state.window);
                    let size = state.window.inner_size();
                    state.camera.update_sizes(size.width, size.height);
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
        frame.clear_depth(1.0);
    }

    state.world_draw_queue.draw(
        &mut frame,
        &state.display,
        &state.programs,
        &state.camera.projection_matrix(),
        &state.buffers,
    );
    state.draw_queue.draw(
        &mut frame,
        &state.display,
        &state.programs,
        &state.projection,
        &state.buffers,
    );

    state.draw_queue.clear();
    state.world_draw_queue.clear();

    if state.gui_initialized {
        state.gui.paint(&state.display, &mut frame);
    }

    // Finish the frame
    frame.finish().unwrap();
    state.window.request_redraw();

    // Create a new frame for the next iteration
    state.frame = Some(state.display.draw());
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
