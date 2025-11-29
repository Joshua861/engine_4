use bevy_math::UVec2;
use glium::winit;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use winit::event::MouseButton;
use winit::keyboard::{Key, KeyCode};
use winit_input_helper::WinitInputHelper;

use crate::get_state;

pub(crate) struct Input {
    helper: WinitInputHelper,
    action_map: HashMap<Action, Button>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

impl From<MouseButton> for Button {
    fn from(value: MouseButton) -> Self {
        Self::Mouse(value)
    }
}

impl From<KeyCode> for Button {
    fn from(value: KeyCode) -> Self {
        Self::Keyboard(value)
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Action(u32);

impl Action {
    pub const fn new(n: u32) -> Self {
        Self(n)
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            helper: WinitInputHelper::new(),
            action_map: HashMap::new(),
        }
    }

    pub fn bind_key(&mut self, action: Action, key: KeyCode) {
        self.action_map.insert(action, key.into());
    }

    pub fn bind_mouse(&mut self, action: Action, mouse_button: MouseButton) {
        self.action_map.insert(action, mouse_button.into());
    }

    pub fn bind_button(&mut self, action: Action, button: Button) {
        self.action_map.insert(action, button);
    }

    pub fn bind(&mut self, action: Action, button: impl Into<Button>) {
        self.action_map.insert(action, button.into());
    }

    pub fn get_key(&self, action: Action) -> Option<&KeyCode> {
        self.action_map.get(&action).and_then(|n| n.as_keyboard())
    }

    pub fn get_mouse(&self, action: Action) -> Option<&MouseButton> {
        self.action_map.get(&action).and_then(|n| n.as_mouse())
    }

    pub fn get_button(&self, action: Action) -> Option<&Button> {
        self.action_map.get(&action)
    }

    pub fn action_pressed(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_pressed(*key),
                Button::Mouse(button) => self.mouse_pressed(*button),
            }
        } else {
            false
        }
    }

    pub fn action_pressed_os(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_pressed_os(*key),
                Button::Mouse(button) => self.mouse_pressed(*button),
            }
        } else {
            false
        }
    }

    pub fn action_released(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_released(*key),
                Button::Mouse(button) => self.mouse_released(*button),
            }
        } else {
            false
        }
    }

    pub fn action_held(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_held(*key),
                Button::Mouse(button) => self.mouse_held(*button),
            }
        } else {
            false
        }
    }

    pub fn get_all_binds(&self) -> &HashMap<Action, Button> {
        &self.action_map
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
    get_state().input.key_pressed(keycode)
}

/// Returns true when the key with the specified keycode goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout, so for example the `W` key will be in the same physical key on both US and french keyboards.
///
/// Will repeat key presses while held down according to the OS's key repeat configuration
/// This is suitable for UI.
pub fn key_pressed_os(keycode: KeyCode) -> bool {
    get_state().input.key_pressed_os(keycode)
}

/// Returns true when the key with the specified KeyCode goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout.
pub fn key_released(keycode: KeyCode) -> bool {
    get_state().input.key_released(keycode)
}

/// Returns true when the key with the specified keycode remains "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout.
pub fn key_held(keycode: KeyCode) -> bool {
    get_state().input.key_held(keycode)
}

/// Returns true while any shift key is held on the keyboard.
/// Otherwise returns false.
pub fn held_shift() -> bool {
    get_state().input.held_shift()
}

/// Returns true while any control key is held on the keyboard.
/// Otherwise returns false.
pub fn held_control() -> bool {
    get_state().input.held_control()
}

/// Returns true while any alt key is held on the keyboard.
/// Otherwise returns false.
pub fn held_alt() -> bool {
    get_state().input.held_alt()
}

/// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
/// Will never repeat keypresses while held.
pub fn key_pressed_logical(check_key: Key<&str>) -> bool {
    get_state().input.key_pressed_logical(check_key)
}

/// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
/// Will repeat key presses while held down according to the OS's key repeat configuration.
/// This is suitable for UI.
pub fn key_pressed_os_logical(check_key: Key<&str>) -> bool {
    get_state().input.key_pressed_os_logical(check_key)
}

/// Returns true when the specified keyboard key goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
pub fn key_released_logical(check_key: Key<&str>) -> bool {
    get_state().input.key_released_logical(check_key)
}

/// Returns true while the specified keyboard key remains "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
pub fn key_held_logical(check_key: Key<&str>) -> bool {
    get_state().input.key_held_logical(check_key)
}

/// Returns true when the specified mouse button goes from "not pressed" to "pressed".
/// Otherwise returns false.
pub fn mouse_pressed(mouse_button: MouseButton) -> bool {
    get_state().input.mouse_pressed(mouse_button)
}

/// Returns true when the specified mouse button goes from "pressed" to "not pressed".
/// Otherwise returns false.
pub fn mouse_released(mouse_button: MouseButton) -> bool {
    get_state().input.mouse_released(mouse_button)
}

/// Returns true while the specified mouse button remains "pressed".
/// Otherwise returns false.
pub fn mouse_held(mouse_button: MouseButton) -> bool {
    get_state().input.mouse_held(mouse_button)
}

/// Returns the amount scrolled by the mouse during the last step.
/// Returns (horizontally, vertically).
///
/// Returns (0.0, 0.0) when the window is not focused.
pub fn scroll_diff() -> (f32, f32) {
    get_state().input.scroll_diff()
}

/// Returns the cursor coordinates in pixels, when window is focused AND
/// (cursor is on window OR any mouse button remains held while cursor moved off window).
/// Otherwise returns None.
pub fn cursor() -> Option<(f32, f32)> {
    get_state().input.cursor()
}

/// Returns the change in cursor coordinates that occurred during the last step,
/// when window is focused AND (cursor is on window OR any mouse button remains held
/// while cursor moved off window). Otherwise returns (0.0, 0.0).
pub fn cursor_diff() -> (f32, f32) {
    get_state().input.cursor_diff()
}

/// Returns the change in mouse coordinates that occurred during the last step.
///
/// This is useful when implementing first person controls with a captured mouse.
pub fn mouse_diff() -> (f32, f32) {
    get_state().input.mouse_diff()
}

/// Returns the characters pressed during the last step.
/// The characters are in the order they were pressed.
pub fn input_text() -> &'static [Key] {
    get_state().input.text()
}

/// Returns the path to a file that has been drag-and-dropped onto the window.
pub fn dropped_file() -> Option<PathBuf> {
    get_state().input.dropped_file()
}

/// Returns the current window size if it was resized during the last step.
/// Otherwise returns None.
pub fn window_resized() -> Option<UVec2> {
    get_state()
        .input
        .window_resized()
        .map(|size| UVec2::new(size.width, size.height))
}

/// Returns the current resolution of the window.
///
/// Returns None when no WindowEvent::Resized have been received yet.
pub fn resolution() -> Option<(u32, u32)> {
    get_state().input.resolution()
}

/// Returns the current scale factor if it was changed during the last step.
/// Otherwise returns None.
pub fn scale_factor_changed() -> Option<f64> {
    get_state().input.scale_factor_changed()
}

/// Returns the current scale_factor of the window.
///
/// Returns None when no WindowEvent::ScaleFactorChanged have been received yet.
pub fn scale_factor() -> Option<f64> {
    get_state().input.scale_factor()
}

/// Returns true if the window has been destroyed. Otherwise returns false.
///
/// Once this method has returned true once, all following calls to this method will also return true.
pub fn destroyed() -> bool {
    get_state().input.destroyed()
}

/// Returns true if the OS has requested the application to close during this step.
/// Otherwise returns false.
pub fn close_requested() -> bool {
    get_state().input.close_requested()
}

/// Returns true when the action's bound button goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_pressed(action: Action) -> bool {
    get_state().input.action_pressed(action)
}

/// Returns true when the action's bound button goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// For keyboard keys, will repeat key presses while held down according to the OS's key repeat configuration.
/// This is suitable for UI.
///
/// Returns false if the action is not bound to any button.
pub fn action_pressed_os(action: Action) -> bool {
    get_state().input.action_pressed_os(action)
}

/// Returns true when the action's bound button goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_released(action: Action) -> bool {
    get_state().input.action_released(action)
}

/// Returns true while the action's bound button remains "pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_held(action: Action) -> bool {
    get_state().input.action_held(action)
}
/// Binds a keyboard key to an action.
///
/// When the key is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind_key(action: Action, key: KeyCode) {
    get_state().input.bind_key(action, key)
}

/// Binds a mouse button to an action.
///
/// When the mouse button is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind_mouse(action: Action, mouse_button: MouseButton) {
    get_state().input.bind_mouse(action, mouse_button)
}

/// Binds a button (either keyboard or mouse) to an action.
///
/// When the button is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind_button(action: Action, button: Button) {
    get_state().input.bind_button(action, button)
}

/// Binds a button (either keyboard or mouse) to an action.
///
/// When the button is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind(action: Action, button: impl Into<Button>) {
    get_state().input.bind(action, button)
}

/// Returns the keyboard key bound to the specified action, if any.
///
/// Returns None if the action is not bound or is bound to a mouse button instead.
pub fn get_key(action: Action) -> Option<&'static KeyCode> {
    get_state().input.get_key(action)
}

/// Returns the mouse button bound to the specified action, if any.
///
/// Returns None if the action is not bound or is bound to a keyboard key instead.
pub fn get_mouse(action: Action) -> Option<&'static MouseButton> {
    get_state().input.get_mouse(action)
}

/// Returns the button (keyboard or mouse) bound to the specified action, if any.
///
/// Returns None if the action is not bound to any button.
pub fn get_button(action: Action) -> Option<&'static Button> {
    get_state().input.get_button(action)
}

/// Get a map of all the bindings that have been registered with the engine.
pub fn get_all_binds() -> &'static HashMap<Action, Button> {
    get_state().input.get_all_binds()
}
