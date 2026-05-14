use std::time::Instant;

use log::info;
use sge_global::global;

global!(Time, time_state);
pub struct Time {
    time: f32,
    physics_time: f32,
    physics_speed: f32,
    physics_delta_time: f32,
    is_physics_time_paused: bool,
    frame_count: usize,
    delta_time: f32,
    last_frame_end_time: Instant,
    frames_since_input: usize,
}

pub fn init() {
    set_time_state(Time {
        time: 0.0,
        physics_time: 0.0,
        physics_speed: 1.0,
        physics_delta_time: 0.0,
        is_physics_time_paused: false,
        frame_count: 0,
        delta_time: 0.0,
        last_frame_end_time: Instant::now(),
        frames_since_input: 0,
    });
    info!("Initialized sge_time");
}

/// call at end of frame
pub fn update(has_input_event: bool) {
    let state = get_time_state();

    if has_input_event {
        state.frames_since_input = 0;
    } else {
        state.frames_since_input += 1;
    }

    let delta_time = state.last_frame_end_time.elapsed().as_secs_f32();
    state.delta_time = delta_time;
    state.time += delta_time;

    if !state.is_physics_time_paused {
        state.physics_delta_time = delta_time * state.physics_speed;
        state.physics_time += state.physics_delta_time;
    } else {
        state.physics_delta_time = 0.0;
    }

    state.frame_count += 1;
    state.last_frame_end_time = Instant::now();
}

pub fn frame_count() -> usize {
    get_time_state().frame_count
}

pub fn physics_time() -> f32 {
    get_time_state().physics_time
}

pub fn physics_delta_time() -> f32 {
    get_time_state().physics_delta_time
}

pub fn physics_speed() -> f32 {
    get_time_state().physics_speed
}

pub fn physics_speed_mut() -> &'static mut f32 {
    &mut get_time_state().physics_speed
}

pub fn set_physics_speed(physics_speed: f32) {
    get_time_state().physics_speed = physics_speed;
}

pub fn pause_physics_timer() {
    get_time_state().is_physics_time_paused = true;
}

pub fn play_physics_timer() {
    get_time_state().is_physics_time_paused = false;
}

pub fn toggle_physics_timer() {
    let state = get_time_state();
    state.is_physics_time_paused = !state.is_physics_time_paused;
}

pub fn is_physics_time_paused() -> bool {
    get_time_state().is_physics_time_paused
}

pub fn is_physics_time_paused_mut() -> &'static mut bool {
    &mut get_time_state().is_physics_time_paused
}

pub fn time() -> f32 {
    get_time_state().time
}

pub fn time_since(instant: f32) -> f32 {
    get_time_state().time - instant
}

pub fn time_seconds() -> usize {
    get_time_state().time as usize
}

pub fn is_first_frame() -> bool {
    let state = get_time_state();

    state.frame_count == 0
}

pub fn oscillate(min: f32, max: f32) -> f32 {
    (time().sin() + 1.0) * 0.5 * (max - min) + min
}

pub fn oscillate_t(min: f32, max: f32, time: f32) -> f32 {
    (time.sin() + 1.0) * 0.5 * (max - min) + min
}

pub fn once_per_second() -> bool {
    let state = get_time_state();

    state.time as usize != (state.time - state.delta_time) as usize
}

pub fn once_per_n_seconds(n: f32) -> bool {
    let state = get_time_state();

    (state.time / n) as usize != ((state.time - state.delta_time) / n) as usize
}

pub fn toggle_every_n_seconds(n: f32) -> bool {
    let state = get_time_state();

    (state.time % n * 2.0) / n <= 1.0
}

pub fn delta_time() -> f32 {
    get_time_state().delta_time
}

pub fn frames_since_input() -> usize {
    get_time_state().frames_since_input
}
