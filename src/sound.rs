use crate::get_state;
use rodio::{OutputStream, OutputStreamBuilder};

pub struct Sound {
    hint: &'static str,
    data: Vec<u8>,
}

pub(crate) struct SoundState {
    pub stream_handle: OutputStream,
}

pub struct SoundRef {
    id: usize,
}

impl SoundRef {
    pub fn get(&self) -> &Sound {
        &get_state().storage.sounds[self.id]
    }

    pub fn get_mut(&self) -> &mut Sound {
        &mut get_state().storage.sounds[self.id]
    }
}

impl SoundState {
    pub fn new() -> anyhow::Result<Self> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;

        Ok(Self { stream_handle })
    }
}

// pub fn play_sound(sound: SoundRef) {
//     get_state().sound.stream_handle.mixer().add(sound.get());
// }

// pub fn load_sound(bytes: &[u8], type_hint: &'static str) -> SoundRef {
//     let sound = Sound {
//         data: bytes.to_vec(),
//         hint: type_hint,
//     };
//     let state = get_state();

//     let id = state.storage.sounds.len();
//     state.storage.sounds.push(sound);

//     let id = SoundRef {

//     }
// }
