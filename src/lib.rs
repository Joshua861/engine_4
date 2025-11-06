#![allow(static_mut_refs)]

use camera::Camera;
use color::Color;
use egui_glium::{EguiGlium, egui_winit::egui::ViewportId};
use fps_ticker::Fps;
use glam::Mat4;
use glium::{
    Frame, Program,
    backend::glutin::{Display, SimpleWindowBuilder},
    glutin::{self, surface::WindowSurface},
    implement_vertex,
    winit::{
        event::Event, event_loop::EventLoop, platform::pump_events::EventLoopExtPumpEvents,
        window::Window,
    },
};
use programs::Programs;
use shapes::DrawQueue;
use winit_input_helper::WinitInputHelper;

mod api;
mod camera;
mod color;
#[feature(debugging)]
mod debugging;
pub mod prelude;
mod programs;
mod shapes;
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
    fps: Fps,
    camera: Camera,
    gui: EguiGlium,
    draw_queue: DrawQueue,
    world_draw_queue: DrawQueue,
}

unsafe impl Sync for EngineState {}
unsafe impl Send for EngineState {}

pub fn init(title: &str) -> anyhow::Result<()> {
    env_logger::init();
    color_eyre::install().expect("could not install color_eyre");

    let event_loop = EventLoop::builder().build()?;
    let (window, display) = SimpleWindowBuilder::new()
        .with_title(title)
        .build(&event_loop);
    let input = WinitInputHelper::new();
    window.request_redraw();

    let vertex_shader_src = include_str!("../assets/shaders/flat/vertex.glsl");
    let fragment_shader_src = include_str!("../assets/shaders/flat/fragment.glsl");
    let flat_program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)?;

    let vertex_shader_src = include_str!("../assets/shaders/circle/vertex.glsl");
    let fragment_shader_src = include_str!("../assets/shaders/circle/fragment.glsl");
    let circle_program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)?;

    let programs = Programs {
        circle: circle_program,
        flat: flat_program,
    };

    let frame = Some(display.draw());
    let projection = projection(&window);
    let fps = Fps::default();
    let camera = Camera::from_window(&window);
    let gui = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
    let draw_queue = DrawQueue::empty();
    let world_draw_queue = DrawQueue::empty();

    unsafe {
        ENGINE_STATE = Some(EngineState {
            programs,
            window,
            display,
            input,
            event_loop,
            frame,
            projection,
            fps,
            camera,
            gui,
            draw_queue,
            world_draw_queue,
        });
    }

    thread_assert::set_thread_id();

    Ok(())
}

pub fn next_frame() {
    let state = get_state();

    state.fps.tick();

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
    );
    state.draw_queue.draw(
        &mut frame,
        &state.display,
        &state.programs,
        &state.projection,
    );

    state.draw_queue.clear();
    state.world_draw_queue.clear();

    state.gui.paint(&state.display, &mut frame);

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
