#![allow(static_mut_refs)]

use buffers::Buffers;
use camera::Camera;
use color::Color;
#[cfg(feature = "debugging")]
use debugging::DebugInfo;
use draw_queue::DrawQueue;
use egui_glium::{EguiGlium, egui_winit::egui::ViewportId};
use fps_ticker::Fps;
use glam::Mat4;
use glium::{
    Frame,
    backend::glutin::{Display, SimpleWindowBuilder},
    glutin::surface::WindowSurface,
    implement_vertex,
    winit::{
        event::Event, event_loop::EventLoop, platform::pump_events::EventLoopExtPumpEvents,
        window::Window,
    },
};
use programs::Programs;
use rand::rngs::ThreadRng;
use textures::EngineTexture;
use winit_input_helper::WinitInputHelper;

mod api;
mod buffers;
mod camera;
pub mod collisions;
mod color;
#[cfg(feature = "debugging")]
mod debugging;
mod draw_queue;
mod physics;
mod post_processing;
pub mod prelude;
mod programs;
mod shapes;
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

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    fn new(x: f32, y: f32, c: Color) -> Self {
        Self {
            position: [x, y],
            color: c.for_gpu(),
        }
    }
}

implement_vertex!(Vertex, position, color);

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
    #[cfg(feature = "debugging")]
    debug_info: debugging::DebugInfo,
    storage: EngineStorage,
    buffers: Buffers,
    rng: ThreadRng,
}

unsafe impl Sync for EngineState {}
unsafe impl Send for EngineState {}

pub(crate) struct EngineStorage {
    textures: Vec<EngineTexture>,
}

impl EngineStorage {
    pub fn new() -> Self {
        Self { textures: vec![] }
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

    let frame = Some(display.draw());
    let projection = projection(&window);
    let camera = Camera::from_window(&window);
    let gui = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
    let draw_queue = DrawQueue::empty();
    let world_draw_queue = DrawQueue::empty();
    #[cfg(feature = "debugging")]
    let debug_info = DebugInfo::new();
    let textures = EngineStorage::new();
    let buffers = Buffers::new(&display)?;
    let programs = Programs::new(&display)?;
    let rng = rand::rng();
    let gui_initialized = false;

    unsafe {
        ENGINE_STATE = Some(EngineState {
            programs,
            window,
            display,
            input,
            event_loop,
            frame,
            projection,
            camera,
            gui,
            gui_initialized,
            draw_queue,
            world_draw_queue,
            debug_info,
            buffers,
            storage: textures,
            rng,
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

    let mut frame = state.frame.take().unwrap();

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

    frame.finish().unwrap();
    state.window.request_redraw();

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

fn projection(window: &Window) -> Mat4 {
    let size = window.inner_size();
    Mat4::orthographic_rh(0.0, size.width as f32, size.height as f32, 0.0, -1.0, 1.0)
}
