#![allow(deprecated)]

use std::num::NonZeroU32;

use glium::{
    Version,
    backend::{Facade, glutin::Display},
    glutin::{
        config::ConfigTemplateBuilder,
        context::ContextAttributesBuilder,
        display::GetGlDisplay,
        prelude::*,
        surface::{SurfaceAttributesBuilder, SwapInterval, WindowSurface},
    },
    winit::{
        self,
        dpi::PhysicalSize,
        error::ExternalError,
        event_loop::EventLoop,
        raw_window_handle::HasRawWindowHandle,
        window::{
            Cursor, CursorGrabMode, CursorIcon, Fullscreen, Window, WindowAttributes, WindowLevel,
        },
    },
};
use glutin_winit::{DisplayBuilder, GlWindow};
#[cfg(debug_assertions)]
use log::info;
use sge_error_union::ErrorUnion;
use sge_vectors::{UVec2, Vec2};

pub type SgeDisplay = Display<WindowSurface>;

pub struct WindowState {
    pub window: Window,
    pub display: SgeDisplay,
    pub event_loop: EventLoop<()>,
    pub window_size: Vec2,
    pub using_custom_cursor: bool,
}

pub fn end_of_frame() {
    let state = get_window_state();

    match state.using_custom_cursor {
        true => {
            state.using_custom_cursor = false;
        }
        false => {
            state.window.set_cursor(Cursor::Icon(CursorIcon::Default));
        }
    }
}

sge_global::global!(WindowState, window_state);

pub struct WindowOptions {
    pub template: ConfigTemplateBuilder,
    pub surface_attributes: SurfaceAttributesBuilder<WindowSurface>,
    pub context_attributes: ContextAttributesBuilder,
    pub swap_interval: SwapInterval,
    pub window_attributes: WindowAttributes,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            template: ConfigTemplateBuilder::new().with_multisampling(4),
            surface_attributes: Default::default(),
            context_attributes: ContextAttributesBuilder::new(),
            // swap_interval: SwapInterval::DontWait,
            swap_interval: SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
            window_attributes: Window::default_attributes().with_transparent(true),
        }
    }
}

#[derive(ErrorUnion, Debug)]
pub enum WindowCreationError {
    EventLoop(glium::winit::error::EventLoopError),
    Handle(glium::winit::raw_window_handle::HandleError),
    Glutin(glium::glutin::error::Error),
    IncompatibleOpenGl(glium::IncompatibleOpenGl),
}

pub fn init(opts: WindowOptions) -> Result<(), WindowCreationError> {
    let event_loop = EventLoop::builder().build()?;

    let window_attributes = opts.window_attributes;

    let (window, gl_config) = DisplayBuilder::new()
        .with_preference(glutin_winit::ApiPreference::FallbackEgl)
        .with_window_attributes(Some(window_attributes))
        .build(&event_loop, opts.template, |configs| {
            // Find the config with the maximum number of samples
            configs
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let window = window.unwrap();

    let context_attributes = opts
        .context_attributes
        .build(Some(window.raw_window_handle()?));

    let gl_display = gl_config.display();
    let not_current_context =
        unsafe { gl_display.create_context(&gl_config, &context_attributes)? };

    let surface_attributes = window
        .build_surface_attributes(opts.surface_attributes)
        .expect("Failed to build surface attributes");

    let gl_surface = unsafe { gl_display.create_window_surface(&gl_config, &surface_attributes)? };

    let gl_context = not_current_context.make_current(&gl_surface)?;

    gl_surface
        .set_swap_interval(&gl_context, opts.swap_interval)
        .expect("Failed to set swap interval");

    let display = Display::from_context_surface(gl_context, gl_surface)?;

    #[cfg(debug_assertions)]
    {
        use glium::CapabilitiesSource;

        let samples = display
            .get_context()
            .get_capabilities()
            .max_framebuffer_samples;
        info!("Max samples: {:?}", samples);
    }

    let state = WindowState {
        window_size: physical_size_to_vec2(window.inner_size()),
        display,
        window,
        event_loop,
        using_custom_cursor: false,
    };

    set_window_state(state);

    Ok(())
}

fn physical_size_to_vec2(size: PhysicalSize<u32>) -> Vec2 {
    Vec2::new(size.width as f32, size.height as f32)
}

pub fn window_size() -> Vec2 {
    get_window_state().window_size
}

pub fn window_center() -> Vec2 {
    window_size() / 2.0
}

pub fn window_size_u32() -> UVec2 {
    let size = get_window_state().window_size;
    UVec2::new(size.x as u32, size.y as u32)
}

pub fn window_height() -> f32 {
    get_window_state().window_size.y
}

pub fn window_aspect_ratio() -> f32 {
    let size = get_window_state().window_size;
    size.x / size.y
}

pub fn window_width() -> f32 {
    get_window_state().window_size.x
}

pub fn handle_window_resize(size: PhysicalSize<u32>) {
    let window_state = get_window_state();
    window_state.display.resize(size.into());
    let size = window_state.window.inner_size();
    window_state.window_size = physical_size_to_vec2(size);
}

pub fn dpi_scaling() -> f32 {
    get_window_state().window.scale_factor() as f32
}

pub fn get_display() -> &'static SgeDisplay {
    &get_window_state().display
}

pub fn get_display_mut() -> &'static mut SgeDisplay {
    &mut get_window_state().display
}

pub fn max_window_dimension() -> f32 {
    window_height().max(window_width())
}

pub fn min_window_dimension() -> f32 {
    window_height().min(window_width())
}

/// only active for one frame
pub fn use_cursor_icon(icon: CursorIcon) {
    let state = get_window_state();
    state.window.set_cursor(Cursor::Icon(icon));
    state.using_custom_cursor = true;
}

/// only active for one frame
pub fn use_default_cursor_icon() {
    use_cursor_icon(CursorIcon::Default);
}
/// only active for one frame
pub fn use_context_menu_cursor_icon() {
    use_cursor_icon(CursorIcon::ContextMenu);
}
/// only active for one frame
pub fn use_help_cursor_icon() {
    use_cursor_icon(CursorIcon::Help);
}
/// only active for one frame
pub fn use_pointer_cursor_icon() {
    use_cursor_icon(CursorIcon::Pointer);
}
/// only active for one frame
pub fn use_progress_cursor_icon() {
    use_cursor_icon(CursorIcon::Progress);
}
/// only active for one frame
pub fn use_wait_cursor_icon() {
    use_cursor_icon(CursorIcon::Wait);
}
/// only active for one frame
pub fn use_cell_cursor_icon() {
    use_cursor_icon(CursorIcon::Cell);
}
/// only active for one frame
pub fn use_crosshair_cursor_icon() {
    use_cursor_icon(CursorIcon::Crosshair);
}
/// only active for one frame
pub fn use_text_cursor_icon() {
    use_cursor_icon(CursorIcon::Text);
}
/// only active for one frame
pub fn use_vertical_text_cursor_icon() {
    use_cursor_icon(CursorIcon::VerticalText);
}
/// only active for one frame
pub fn use_alias_cursor_icon() {
    use_cursor_icon(CursorIcon::Alias);
}
/// only active for one frame
pub fn use_copy_cursor_icon() {
    use_cursor_icon(CursorIcon::Copy);
}
/// only active for one frame
pub fn use_move_cursor_icon() {
    use_cursor_icon(CursorIcon::Move);
}
/// only active for one frame
pub fn use_no_drop_cursor_icon() {
    use_cursor_icon(CursorIcon::NoDrop);
}
/// only active for one frame
pub fn use_not_allowed_cursor_icon() {
    use_cursor_icon(CursorIcon::NotAllowed);
}
/// only active for one frame
pub fn use_grab_cursor_icon() {
    use_cursor_icon(CursorIcon::Grab);
}
/// only active for one frame
pub fn use_grabbing_cursor_icon() {
    use_cursor_icon(CursorIcon::Grabbing);
}
/// only active for one frame
pub fn use_e_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::EResize);
}
/// only active for one frame
pub fn use_n_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::NResize);
}
/// only active for one frame
pub fn use_ne_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::NeResize);
}
/// only active for one frame
pub fn use_nw_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::NwResize);
}
/// only active for one frame
pub fn use_s_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::SResize);
}
/// only active for one frame
pub fn use_se_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::SeResize);
}
/// only active for one frame
pub fn use_sw_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::SwResize);
}
/// only active for one frame
pub fn use_w_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::WResize);
}
/// only active for one frame
pub fn use_ew_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::EwResize);
}
/// only active for one frame
pub fn use_ns_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::NsResize);
}
/// only active for one frame
pub fn use_nesw_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::NeswResize);
}
/// only active for one frame
pub fn use_nwse_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::NwseResize);
}
/// only active for one frame
pub fn use_col_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::ColResize);
}
/// only active for one frame
pub fn use_row_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::RowResize);
}
/// only active for one frame
pub fn use_all_scroll_cursor_icon() {
    use_cursor_icon(CursorIcon::AllScroll);
}
/// only active for one frame
pub fn use_zoom_in_cursor_icon() {
    use_cursor_icon(CursorIcon::ZoomIn);
}
/// only active for one frame
pub fn use_zoom_out_cursor_icon() {
    use_cursor_icon(CursorIcon::ZoomOut);
}
/// only active for one frame
pub fn use_dnd_ask_cursor_icon() {
    use_cursor_icon(CursorIcon::DndAsk);
}
/// only active for one frame
pub fn use_all_resize_cursor_icon() {
    use_cursor_icon(CursorIcon::AllResize);
}

pub fn grab_cursor() -> Result<(), ExternalError> {
    let window = &mut get_window_state().window;
    window.set_cursor_visible(false);
    window
        .set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
}

pub fn release_cursor() -> Result<(), ExternalError> {
    let window = &mut get_window_state().window;
    window.set_cursor_visible(true);
    window.set_cursor_grab(CursorGrabMode::None)
}

pub fn set_cursor_visible(visible: bool) {
    get_window_state().window.set_cursor_visible(visible);
}

pub fn set_cursor_grab(grab_mode: CursorGrabMode) -> Result<(), ExternalError> {
    get_window_state().window.set_cursor_grab(grab_mode)
}

pub fn availible_monitors() -> impl Iterator<Item = glium::winit::monitor::MonitorHandle> {
    get_window_state().window.available_monitors()
}

pub fn current_monitor() -> Option<glium::winit::monitor::MonitorHandle> {
    get_window_state().window.current_monitor()
}

pub fn fullscreen() -> Option<Fullscreen> {
    get_window_state().window.fullscreen()
}

pub fn is_fullscreen() -> bool {
    get_window_state().window.fullscreen().is_some()
}

pub fn set_fullscreen(fullscreen: Option<Fullscreen>) {
    get_window_state().window.set_fullscreen(fullscreen);
}

pub fn set_window_decorations(decorations: bool) {
    get_window_state().window.set_decorations(decorations);
}

pub fn is_window_decorated() -> bool {
    get_window_state().window.is_decorated()
}

pub fn set_window_level(level: WindowLevel) {
    get_window_state().window.set_window_level(level);
}

pub fn set_window_icon(icon: Option<glium::winit::window::Icon>) {
    get_window_state().window.set_window_icon(icon);
}

pub fn is_window_focused() -> bool {
    get_window_state().window.has_focus()
}

pub fn set_window_theme(theme: Option<glium::winit::window::Theme>) {
    get_window_state().window.set_theme(theme);
}

pub fn window_theme() -> Option<glium::winit::window::Theme> {
    get_window_state().window.theme()
}

pub fn window_title() -> String {
    get_window_state().window.title().to_string()
}

pub fn set_window_content_protected(protected: bool) {
    get_window_state().window.set_content_protected(protected);
}

pub fn set_window_cursor_hittest(enabled: bool) -> Result<(), ExternalError> {
    get_window_state().window.set_cursor_hittest(enabled)
}

pub fn opengl_context() -> &'static std::rc::Rc<glium::backend::Context> {
    get_window_state().display.get_context()
}

pub fn opengl_version() -> &'static Version {
    opengl_context().get_opengl_version()
}

pub fn window_enabled_buttons() -> glium::winit::window::WindowButtons {
    get_window_state().window.enabled_buttons()
}

pub fn is_window_minimised() -> Option<bool> {
    get_window_state().window.is_minimized()
}

pub fn set_window_maximized(maximized: bool) {
    get_window_state().window.set_maximized(maximized);
}

pub fn window_inner_size() -> Vec2 {
    physical_size_to_vec2(get_window_state().window.inner_size())
}

pub fn request_window_inner_size(size: impl IntoWindowSize) -> Option<PhysicalSize<u32>> {
    get_window_state()
        .window
        .request_inner_size(size.into_window_size())
}

pub fn window_inner_position() -> Option<Vec2> {
    get_window_state()
        .window
        .inner_position()
        .ok()
        .map(|pos| Vec2::new(pos.x as f32, pos.y as f32))
}

pub fn window_id() -> glium::winit::window::WindowId {
    get_window_state().window.id()
}

pub fn is_window_maximized() -> bool {
    get_window_state().window.is_maximized()
}

pub fn is_window_minimized() -> Option<bool> {
    get_window_state().window.is_minimized()
}

pub fn is_window_resizable() -> bool {
    get_window_state().window.is_resizable()
}

pub fn is_window_visible() -> Option<bool> {
    get_window_state().window.is_visible()
}

pub fn primary_monitor() -> Option<glium::winit::monitor::MonitorHandle> {
    get_window_state().window.primary_monitor()
}

pub fn window_request_user_attention(request_type: glium::winit::window::UserAttentionType) {
    get_window_state()
        .window
        .request_user_attention(Some(request_type));
}

pub fn cancel_window_request_user_attention() {
    get_window_state().window.request_user_attention(None);
}

pub fn resize_increments() -> Option<Vec2> {
    get_window_state()
        .window
        .resize_increments()
        .map(|increments| Vec2::new(increments.width as f32, increments.height as f32))
}

pub fn set_window_blur(enabled: bool) {
    get_window_state().window.set_blur(enabled);
}

pub fn set_cursor_position(position: Vec2) -> Result<(), ExternalError> {
    get_window_state()
        .window
        .set_cursor_position(winit::dpi::PhysicalPosition::new(
            position.x as f64,
            position.y as f64,
        ))
}

pub fn set_max_window_inner_size(size: impl IntoWindowSize) {
    get_window_state()
        .window
        .set_max_inner_size(Some(size.into_window_size()));
}

pub fn set_min_window_inner_size(size: impl IntoWindowSize) {
    get_window_state()
        .window
        .set_min_inner_size(Some(size.into_window_size()));
}

pub fn set_window_resizable(resizable: bool) {
    get_window_state().window.set_resizable(resizable);
}

pub fn set_window_title(title: impl Into<String>) {
    get_window_state().window.set_title(&title.into());
}

pub fn set_window_visible(visible: bool) {
    get_window_state().window.set_visible(visible);
}

pub fn window_outer_size() -> Vec2 {
    physical_size_to_vec2(get_window_state().window.outer_size())
}

pub fn window_outer_position() -> Option<Vec2> {
    get_window_state()
        .window
        .outer_position()
        .ok()
        .map(|pos| Vec2::new(pos.x as f32, pos.y as f32))
}

pub trait IntoWindowSize {
    fn into_window_size(self) -> winit::dpi::Size;
}

impl IntoWindowSize for winit::dpi::LogicalSize<f64> {
    fn into_window_size(self) -> winit::dpi::Size {
        self.into()
    }
}

impl IntoWindowSize for winit::dpi::PhysicalSize<u32> {
    fn into_window_size(self) -> winit::dpi::Size {
        self.into()
    }
}

impl IntoWindowSize for sge_vectors::Vec2 {
    fn into_window_size(self) -> winit::dpi::Size {
        winit::dpi::LogicalSize::new(self.x as f64, self.y as f64).into()
    }
}

impl IntoWindowSize for sge_vectors::UVec2 {
    fn into_window_size(self) -> winit::dpi::Size {
        winit::dpi::PhysicalSize::new(self.x, self.y).into()
    }
}
