#[cfg(feature = "clipboard")]
use arboard::Clipboard;
#[cfg(feature = "gamepad")]
pub use gilrs;
#[cfg(feature = "gamepad")]
use gilrs::Gilrs;
use glium::winit;
use helper::WinitInputHelper;
use log::info;
use sge_error_union::{ErrorUnion, Union};
use sge_global::global;
use sge_types::Area;
use sge_vectors::{UVec2, Vec2, vec2};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
pub use winit::event::MouseButton;
pub use winit::keyboard::{Key, KeyCode};

#[cfg(feature = "gamepad")]
pub mod gamepad;
pub mod helper;
pub mod keys;

pub struct Input {
    pub helper: WinitInputHelper,
    action_map: HashMap<Action, Vec<Button>>,
    #[cfg(feature = "gamepad")]
    pub gamepad: Gilrs,
    last_cursor_position: Vec2,
    #[cfg(feature = "clipboard")]
    clipboard: Clipboard,
}

global!(Input, input);

#[cfg(feature = "gamepad")]
pub fn init() -> Result<(), InputError> {
    set_input(Input::new()?);
    info!("Initialized input");
    Ok(())
}

pub fn update() {
    if let Some(cursor) = cursor() {
        get_input().last_cursor_position = cursor;
    }

    #[cfg(feature = "gamepad")]
    {
        let input = get_input();
        while let Some(event) = input.gamepad.next_event() {
            input.gamepad.update(&event);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Union)]
pub enum Button {
    Mouse(MouseButton),
    Keyboard(KeyCode),
}

impl Button {
    /// Returns `true` if the button is [`Mouse`].
    ///
    /// [`Mouse`]: Button::Mouse
    #[must_use]
    pub fn is_mouse(&self) -> bool {
        matches!(self, Self::Mouse(..))
    }

    /// Returns `true` if the button is [`Keyboard`].
    ///
    /// [`Keyboard`]: Button::Keyboard
    #[must_use]
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Self::Keyboard(..))
    }

    pub fn as_mouse(&self) -> Option<&MouseButton> {
        if let Self::Mouse(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_keyboard(&self) -> Option<&KeyCode> {
        if let Self::Keyboard(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Action(u32);

impl Action {
    pub const fn new(n: u32) -> Self {
        Self(n)
    }
}

#[derive(ErrorUnion, Debug)]
pub enum InputError {
    #[cfg(feature = "gamepad")]
    Gilrs(GilrsError),
    #[cfg(feature = "clipboard")]
    Clipboard(arboard::Error),
    Other(&'static str),
}

impl Input {
    pub fn new() -> Result<Self, InputError> {
        Ok(Self {
            helper: WinitInputHelper::new(),
            action_map: HashMap::new(),
            #[cfg(feature = "gamepad")]
            gamepad: Gilrs::new().map_err(|e| GilrsError::from(e))?,
            last_cursor_position: Vec2::ZERO,
            #[cfg(feature = "clipboard")]
            clipboard: Clipboard::new()?,
        })
    }

    pub fn is_cursor_within_area(&self, area: Area) -> bool {
        let cursor = self.cursor();
        if let Some(cursor) = cursor {
            cursor.0 >= area.top_left().x
                && cursor.0 <= area.bottom_right().x
                && cursor.1 >= area.top_left().y
                && cursor.1 <= area.bottom_right().y
        } else {
            false
        }
    }

    pub fn bind(&mut self, action: Action, button: impl Into<Button>) {
        let button = button.into();

        if let Some(list) = self.action_map.get_mut(&action) {
            list.push(button);
        } else {
            self.action_map.insert(action, vec![button]);
        }
    }

    fn action_query(&self, action: Action, f: impl Fn(Button) -> bool) -> bool {
        if let Some(buttons) = self.action_map.get(&action) {
            for button in buttons {
                if f(*button) {
                    return true;
                }
            }

            false
        } else {
            false
        }
    }

    pub fn action_pressed(&self, action: Action) -> bool {
        self.action_query(action, |button| match button {
            Button::Keyboard(key) => self.key_pressed(key),
            Button::Mouse(key) => self.mouse_held(key),
        })
    }

    pub fn action_pressed_os(&self, action: Action) -> bool {
        self.action_query(action, |button| match button {
            Button::Keyboard(key) => self.key_pressed_os(key),
            Button::Mouse(key) => self.mouse_held(key),
        })
    }

    pub fn action_released(&self, action: Action) -> bool {
        self.action_query(action, |button| match button {
            Button::Keyboard(key) => self.key_released(key),
            Button::Mouse(key) => self.mouse_released(key),
        })
    }

    pub fn action_held(&self, action: Action) -> bool {
        self.action_query(action, |button| match button {
            Button::Keyboard(key) => self.key_held(key),
            Button::Mouse(key) => self.mouse_held(key),
        })
    }

    pub fn get_all_binds(&self) -> &HashMap<Action, Vec<Button>> {
        &self.action_map
    }

    pub fn last_cursor_pos(&self) -> Vec2 {
        self.last_cursor_position
    }
}

impl Deref for Input {
    type Target = WinitInputHelper;

    fn deref(&self) -> &Self::Target {
        &self.helper
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.helper
    }
}

/// Returns true when the key with the specified keycode goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout, so for example the `W` key will be in the same physical key on both US and french keyboards.
///
/// This is suitable for game controls.
pub fn key_pressed(keycode: KeyCode) -> bool {
    get_input().key_pressed(keycode)
}

pub fn button_pressed(button: Button) -> bool {
    match button {
        Button::Keyboard(key) => key_pressed(key),
        Button::Mouse(button) => mouse_pressed(button),
    }
}

/// Returns true when the key with the specified keycode goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout, so for example the `W` key will be in the same physical key on both US and french keyboards.
///
/// Will repeat key presses while held down according to the OS's key repeat configuration
/// This is suitable for UI.
pub fn key_pressed_os(keycode: KeyCode) -> bool {
    get_input().key_pressed_os(keycode)
}

/// Returns true when the key with the specified KeyCode goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout.
pub fn key_released(keycode: KeyCode) -> bool {
    get_input().key_released(keycode)
}

pub fn button_released(button: Button) -> bool {
    match button {
        Button::Keyboard(key) => key_released(key),
        Button::Mouse(button) => mouse_released(button),
    }
}

/// Returns true when the key with the specified keycode remains "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout.
pub fn key_held(keycode: KeyCode) -> bool {
    get_input().key_held(keycode)
}

pub fn button_held(button: Button) -> bool {
    match button {
        Button::Keyboard(key) => key_held(key),
        Button::Mouse(button) => mouse_held(button),
    }
}

/// Returns true while any shift key is held on the keyboard.
/// Otherwise returns false.
pub fn held_shift() -> bool {
    get_input().held_shift()
}

/// Returns true while any control key is held on the keyboard.
/// Otherwise returns false.
pub fn held_control() -> bool {
    get_input().held_control()
}

/// Returns true while any alt key is held on the keyboard.
/// Otherwise returns false.
pub fn held_alt() -> bool {
    get_input().held_alt()
}

/// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
/// Will never repeat keypresses while held.
pub fn key_pressed_logical(check_key: Key<&str>) -> bool {
    get_input().key_pressed_logical(check_key)
}

/// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
/// Will repeat key presses while held down according to the OS's key repeat configuration.
/// This is suitable for UI.
pub fn key_pressed_os_logical(check_key: Key<&str>) -> bool {
    get_input().key_pressed_os_logical(check_key)
}

/// Returns true when the specified keyboard key goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
pub fn key_released_logical(check_key: Key<&str>) -> bool {
    get_input().key_released_logical(check_key)
}

/// Returns true while the specified keyboard key remains "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
pub fn key_held_logical(check_key: Key<&str>) -> bool {
    get_input().key_held_logical(check_key)
}

/// Returns true when the specified mouse button goes from "not pressed" to "pressed".
/// Otherwise returns false.
pub fn mouse_pressed(mouse_button: MouseButton) -> bool {
    get_input().mouse_pressed(mouse_button)
}

/// Returns true when the specified mouse button goes from "pressed" to "not pressed".
/// Otherwise returns false.
pub fn mouse_released(mouse_button: MouseButton) -> bool {
    get_input().mouse_released(mouse_button)
}

/// Returns true while the specified mouse button remains "pressed".
/// Otherwise returns false.
pub fn mouse_held(mouse_button: MouseButton) -> bool {
    get_input().mouse_held(mouse_button)
}

/// Returns the amount scrolled by the mouse during the last step.
/// Returns (horizontally, vertically).
///
/// Returns (0.0, 0.0) when the window is not focused.
pub fn scroll_diff() -> Vec2 {
    get_input().scroll_diff().into()
}

/// Returns the cursor coordinates in pixels, when window is focused AND
/// (cursor is on window OR any mouse button remains held while cursor moved off window).
/// Otherwise returns None.
pub fn cursor() -> Option<Vec2> {
    get_input().cursor().map(|c| vec2(c.0, c.1))
}

pub fn cursor_prev() -> Option<Vec2> {
    get_input().cursor_prev().map(|c| vec2(c.0, c.1))
}

/// Returns the change in cursor coordinates that occurred during the last step,
/// when window is focused AND (cursor is on window OR any mouse button remains held
/// while cursor moved off window). Otherwise returns (0.0, 0.0).
pub fn cursor_diff() -> Vec2 {
    get_input().cursor_diff().into()
}

/// Returns the change in mouse coordinates that occurred during the last step.
///
/// This is useful when implementing first person controls with a captured mouse.
pub fn mouse_diff() -> Vec2 {
    get_input().mouse_diff().into()
}

/// Returns the characters pressed during the last step.
/// The characters are in the order they were pressed.
pub fn input_text() -> &'static [Key] {
    get_input().text()
}

/// Returns the path to a file that has been drag-and-dropped onto the window.
pub fn dropped_file() -> Option<PathBuf> {
    get_input().dropped_file()
}

/// Returns the path to a file that the OS is currently dragging over the window, if any.
pub fn hovered_file() -> Option<PathBuf> {
    get_input().hovered_file()
}

/// Returns the current window size if it was resized during the last step.
/// Otherwise returns None.
pub fn window_resized() -> Option<UVec2> {
    get_input()
        .window_resized()
        .map(|size| UVec2::new(size.width, size.height))
}

/// Returns the current resolution of the window.
///
/// Returns None when no WindowEvent::Resized have been received yet.
pub fn resolution() -> Option<(u32, u32)> {
    get_input().resolution()
}

/// Returns the current scale factor if it was changed during the last step.
/// Otherwise returns None.
pub fn scale_factor_changed() -> Option<f64> {
    get_input().scale_factor_changed()
}

/// Returns the current scale_factor of the window.
///
/// Returns None when no WindowEvent::ScaleFactorChanged have been received yet.
pub fn scale_factor() -> Option<f64> {
    get_input().scale_factor()
}

/// Returns true if the window has been destroyed. Otherwise returns false.
///
/// Once this method has returned true once, all following calls to this method will also return true.
pub fn destroyed() -> bool {
    get_input().destroyed()
}

/// Returns true if the OS has requested the application to close during this step.
/// Otherwise returns false.
pub fn close_requested() -> bool {
    get_input().close_requested()
}

/// Returns true when the action's bound button goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_pressed(action: Action) -> bool {
    get_input().action_pressed(action)
}

/// Returns true when the action's bound button goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// For keyboard keys, will repeat key presses while held down according to the OS's key repeat configuration.
/// This is suitable for UI.
///
/// Returns false if the action is not bound to any button.
pub fn action_pressed_os(action: Action) -> bool {
    get_input().action_pressed_os(action)
}

/// Returns true when the action's bound button goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_released(action: Action) -> bool {
    get_input().action_released(action)
}

/// Returns true while the action's bound button remains "pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_held(action: Action) -> bool {
    get_input().action_held(action)
}

/// Binds a button (either keyboard or mouse) to an action.
///
/// When the button is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind(action: Action, button: impl Into<Button>) {
    get_input().bind(action, button)
}

/// Get a map of all the bindings that have been registered with the engine.
pub fn get_all_binds() -> &'static HashMap<Action, Vec<Button>> {
    get_input().get_all_binds()
}

#[cfg(feature = "precise_cursor_movement")]
pub fn cursor_movements() -> Vec<Vec2> {
    get_input()
        .helper
        .cursor_movements()
        .iter()
        .map(|&v| v.into())
        .collect()
}

pub fn last_cursor_pos() -> Vec2 {
    get_input().last_cursor_pos()
}

pub fn should_quit() -> bool {
    get_input().close_requested()
}

#[cfg(feature = "gamepad")]
pub fn gamepad_input() -> &'static Gilrs {
    &get_input().gamepad
}

pub fn pressed_movement_vector(
    up: impl Into<Button>,
    down: impl Into<Button>,
    left: impl Into<Button>,
    right: impl Into<Button>,
) -> Vec2 {
    let up = up.into();
    let down = down.into();
    let left = left.into();
    let right = right.into();

    vec2(
        (button_held(right) as i32 - button_held(left) as i32) as f32,
        (button_held(down) as i32 - button_held(up) as i32) as f32,
    )
}

#[non_exhaustive]
#[derive(Debug)]
#[cfg(feature = "gamepad")]
pub enum GilrsError {
    NotImplemented,
    InvalidAxisToBtn,
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[cfg(feature = "gamepad")]
impl From<gilrs::Error> for GilrsError {
    fn from(value: gilrs::Error) -> Self {
        match value {
            gilrs::Error::InvalidAxisToBtn => Self::InvalidAxisToBtn,
            gilrs::Error::NotImplemented(_) => Self::NotImplemented,
            gilrs::Error::Other(e) => Self::Other(e),
            _ => unimplemented!(),
        }
    }
}

#[cfg(feature = "gamepad")]
impl std::error::Error for GilrsError {}

#[cfg(feature = "gamepad")]
impl std::fmt::Display for GilrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented => f.write_str("Gilrs does not support current platform."),
            Self::InvalidAxisToBtn => f.write_str(
                "Either `pressed ≤ released` or one of values is outside [0.0, 1.0] range.",
            ),
            Self::Other(e) => e.fmt(f),
        }
    }
}

pub fn is_focused() -> bool {
    get_input().helper.current.is_some()
}

#[cfg(feature = "clipboard")]
pub fn get_clipboard_text() -> Option<String> {
    get_input().clipboard.get_text().ok()
}

#[cfg(feature = "clipboard")]
pub fn set_clipboard_text() -> Option<String> {
    get_input().clipboard.get_text().ok()
}

pub fn open_url(url: impl AsRef<OsStr>) {
    if let Err(err) = open::that(url) {
        log::error!("Failed to open url: {}", err);
    }
}
