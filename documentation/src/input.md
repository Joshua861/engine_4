# Input

Input can be detected with these functions.

- `key_pressed`
- `key_pressed_os`
- `key_released`
- `key_held`
- `held_shift`
- `held_control`
- `held_alt`
- `key_pressed_logical`
- `key_released_logical`
- `mouse_pressed`
- `mouse_released`
- `mouse_held`
- `scroll_diff`
- `cursor`
- `cursor_pos`
- `cursor_diff`
- `mouse_diff`
- `input_text`
- `dropped_file`
- `window_resized`
- `resolution`
- `scale_factor_changed`
- `scale_factor`
- `destroyed`
- `close_requested`
- `window_width`
- `window_height`
- `window_size`
- `dpi_scaling`
- `time` (time since program started in seconds)
- `delta_time`
- `physics_time` (time since program started in seconds, but can be paused)
- `pause_physics_timer`
- `play_physics_timer`
- `toggle_physics_timer`

`cursor` returns Option<(f32, f32)>, and None when the cursor is off-screen. 
`cursor_pos` always returns the last position of the cursor if off-screen.

Read the reference documentation for these functions for more information.

The API is based on `winit_input_helper`.

## Action mapping

You may want to bind keys to actions, and refer to them by the name of the
action. The engine has a feature for this. You can manipulate these bindings
with these functions. 

- `action_pressed`
- `action_pressed_os`
- `action_released`
- `action_held`
- `bind_key`
- `bind_mouse`
- `bind_button`
- `bind`
- `get_key_binding`
- `get_mouse_binding`
- `get_binding`
- `get_all_binds`

These can be rebound mid-game, so that you can create some way for users to
rebind keys. Check the `action_mapping.rs` example for more.
