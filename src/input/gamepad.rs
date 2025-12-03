#![allow(unused)]

use anyhow::anyhow;
use gilrs::{Button, Event, Gilrs};

pub struct GamepadInputState {
    gilrs: Gilrs,
}

struct CurrentDeviceInput {}

impl GamepadInputState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            gilrs: Gilrs::new().map_err(|_| anyhow!("Could not initialize gamepad state."))?,
        })
    }

    fn update(&mut self) {
        while let Some(Event {
            id, event, time, ..
        }) = self.gilrs.next_event()
        {}
    }
}
