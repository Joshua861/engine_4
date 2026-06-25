use std::any::Any;

use sge_time::frames_since_input;
#[cfg(feature = "input")]
use sge_vectors::Vec2;

use crate::{WAIT_FOR_EVENTS_EXTRA_FRAME_DRAWS, user_storage::get_user_storage};

pub fn storage_store_state<T: Any>(state: T) {
    get_user_storage().store(state);
}

pub fn storage_init_state<T: Any>(state: T) {
    get_user_storage().initialize(state);
}

pub fn storage_exists<T: Any>() -> bool {
    get_user_storage().exists::<T>()
}

pub fn storage_get_state<T: Any>() -> &'static T {
    get_user_storage().get()
}

pub fn storage_try_get_state<T: Any>() -> Option<&'static T> {
    get_user_storage().try_get()
}

pub fn storage_get_state_mut<T: Any>() -> &'static mut T {
    get_user_storage().get_mut()
}

pub fn storage_try_get_state_mut<T: Any>() -> Option<&'static mut T> {
    get_user_storage().try_get_mut()
}

pub fn is_about_to_wait_for_input() -> bool {
    frames_since_input() >= WAIT_FOR_EVENTS_EXTRA_FRAME_DRAWS - 1
}

#[cfg(feature = "input")]
pub fn cursor_world() -> Option<Vec2> {
    use sge_camera::screen_to_world;
    use sge_input::cursor;

    let Some(pos) = cursor() else {
        return None;
    };
    Some(screen_to_world(pos))
}

#[cfg(feature = "input")]
pub fn last_cursor_world() -> Vec2 {
    use sge_camera::screen_to_world;
    use sge_input::last_cursor_pos;

    screen_to_world(last_cursor_pos())
}
