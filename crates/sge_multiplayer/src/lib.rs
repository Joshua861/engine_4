use std::collections::HashMap;

use backends::{Message, MultiplayerBackend, itty::IttyBackend};
use flate2::{Compress, CompressError, Compression, DecompressError, FlushCompress};
use log::warn;
use sge_error_union::ErrorUnion;
use sge_persistence::{Diffable, PartialLerp, Persistent};
use sge_rng::rand;

pub mod backends;

pub struct MultiplayerState<T: Diffable + Persistent + Clone>
where
    T::Diff: Persistent,
{
    buffer: Vec<u8>,
    current: T,
    old: T,
    username: String,
    user_id: u32,
    users: HashMap<u32, UserData<T>>,
    backend: Box<dyn MultiplayerBackend>,
    notifications: Vec<Notification>,
}

pub struct Notification {
    pub user_id: u32,
    pub data: Vec<u8>,
}

pub struct UserData<T: Diffable>
where
    T::Diff: Persistent,
{
    pub username: String,
    pub current: Option<T>,
    pub history: Vec<(f32, T)>,
}

impl<T: Diffable + PartialLerp + Clone> UserData<T>
where
    T::Diff: Persistent,
{
    pub fn current_lerped(&self, render_time: f32) -> Option<T> {
        if self.history.is_empty() {
            return self.current.clone();
        }

        if render_time <= self.history[0].0 {
            return Some(self.history[0].1.clone());
        }

        if render_time >= self.history[self.history.len() - 1].0 {
            return Some(self.history[self.history.len() - 1].1.clone());
        }

        for i in 0..self.history.len() - 1 {
            let (time_a, state_a) = &self.history[i];
            let (time_b, state_b) = &self.history[i + 1];

            if render_time >= *time_a && render_time <= *time_b {
                let delta = time_b - time_a;
                let t = if delta > 0.0 {
                    (render_time - time_a) / delta
                } else {
                    0.0
                };
                return Some(state_a.partial_lerp(state_b, t));
            }
        }

        self.current.clone()
    }
}

#[derive(ErrorUnion, Debug)]
pub enum MultiplayerError {
    Compress(CompressError),
    Decompress(DecompressError),
    Persistence(sge_persistence::Error),
}

impl<T: Diffable + Persistent + Clone> MultiplayerState<T>
where
    T::Diff: Persistent,
{
    pub fn new(state: T, username: String, room_name: String) -> Self {
        Self::new_with_old(state.clone(), state, username, room_name)
    }

    pub fn new_with_backend(
        state: T,
        username: String,
        backend: impl MultiplayerBackend + 'static,
    ) -> Self {
        Self::new_with_old_and_backend(state.clone(), state, username, backend)
    }

    pub fn new_with_old(current: T, old: T, username: String, room_name: String) -> Self {
        Self::new_with_old_and_backend(current, old, username, IttyBackend::new(room_name))
    }

    pub fn new_with_old_and_backend(
        current: T,
        old: T,
        username: String,
        backend: impl MultiplayerBackend + 'static,
    ) -> Self {
        Self {
            current,
            old,
            username,
            user_id: rand(),
            buffer: vec![],
            users: HashMap::new(),
            backend: Box::new(backend),
            notifications: vec![],
        }
    }

    pub fn update(&mut self, current_time: f32) -> Result<(), MultiplayerError> {
        let messages = self.backend.recieve_messages();

        for message in messages {
            self.handle_message(message, current_time)?;
        }

        let data = self.diff_compressed()?.to_vec();
        self.backend.send_message(Message::Diff {
            user_id: self.user_id,
            data,
        });

        self.old = self.current.clone();

        Ok(())
    }

    pub fn get_user(&self, user_id: u32) -> Option<&UserData<T>> {
        self.users.get(&user_id)
    }

    pub fn get_user_mut(&mut self, user_id: u32) -> Option<&mut UserData<T>> {
        self.users.get_mut(&user_id)
    }

    pub fn receive_updates(&mut self, current_time: f32) -> Result<(), MultiplayerError> {
        let messages = self.backend.recieve_messages();
        for message in messages {
            self.handle_message(message, current_time)?;
        }
        Ok(())
    }

    pub fn send_update(&mut self) -> Result<(), MultiplayerError> {
        let data = self.diff_compressed()?.to_vec();
        self.backend.send_message(Message::Diff {
            user_id: self.user_id,
            data,
        });
        self.old = self.current.clone();
        Ok(())
    }

    pub fn send_notification(&mut self, data: Vec<u8>) {
        self.backend.send_message(Message::Notification {
            user_id: self.user_id,
            data,
        });
    }

    fn handle_message(
        &mut self,
        message: Message,
        current_time: f32,
    ) -> Result<(), MultiplayerError> {
        match message {
            Message::Notification { user_id, data } => {
                self.notifications.push(Notification { user_id, data });
            }
            Message::AnnounceSelf { user_id, username } => {
                if self.users.contains_key(&user_id) {
                    self.users.get_mut(&user_id).unwrap().username = username;
                } else {
                    self.users.insert(
                        user_id,
                        UserData {
                            username,
                            current: None,
                            history: vec![],
                        },
                    );
                }
            }
            Message::Join { user_id, username } => {
                if self.users.contains_key(&user_id) {
                    warn!(
                        "Multiplayer: recieved join message for user already registered: {user_id} {username}"
                    );
                    self.users.get_mut(&user_id).unwrap().username = username;
                } else {
                    self.users.insert(
                        user_id,
                        UserData {
                            username,
                            current: None,
                            history: vec![],
                        },
                    );

                    self.announce_self()?;
                }
            }
            Message::Diff { user_id, data } => {
                let diff = self.uncompress_diff(&data)?;
                if let Some(user) = self.users.get_mut(&user_id) {
                    if let Some(user_data) = &mut user.current {
                        user_data.apply_diff(diff);

                        user.history.push((current_time, user_data.clone()));

                        if user.history.len() > 15 {
                            user.history.remove(0);
                        }
                    } else {
                        warn!("requesting missing data for {} {}", user_id, user.username);
                        self.request_data(user_id);
                    }
                } else {
                    warn!("recieved diff message from unregistered user, requesting data");
                    self.request_data(user_id);
                }
            }
            Message::InitialState { user_id, data } => {
                let parsed_state = T::from_bytes(data)?;
                if let Some(user) = self.users.get_mut(&user_id) {
                    user.current = Some(parsed_state.clone());
                    user.history = vec![(current_time, parsed_state)];
                } else {
                    self.users.insert(
                        user_id,
                        UserData {
                            username: String::new(),
                            current: Some(parsed_state.clone()),
                            history: vec![(current_time, parsed_state)],
                        },
                    );
                }
            }
            Message::RequestData { user, .. } => {
                if user == self.user_id {
                    self.announce_self()?;
                }
            }
            Message::Disconnect { user_id } => {
                self.users.remove(&user_id);
            }
        }
        Ok(())
    }

    fn request_data(&mut self, user: u32) {
        self.backend.send_message(Message::RequestData {
            user_id: self.user_id,
            user,
        });
    }

    pub fn drain_notifications(&mut self) -> std::vec::Drain<'_, Notification> {
        self.notifications.drain(..)
    }

    pub fn announce_self(&mut self) -> Result<(), MultiplayerError> {
        self.backend.send_message(Message::AnnounceSelf {
            user_id: self.user_id,
            username: self.username.clone(),
        });

        self.backend.send_message(Message::InitialState {
            user_id: self.user_id,
            data: self.current.to_bytes()?,
        });

        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.backend.send_message(Message::Disconnect {
            user_id: self.user_id,
        });
    }

    pub fn your_user_data(&self) -> UserData<T> {
        UserData {
            username: self.username.clone(),
            current: Some(self.current.clone()),
            history: vec![],
        }
    }

    pub fn your_username(&self) -> &str {
        &self.username
    }

    pub fn your_id(&self) -> u32 {
        self.user_id
    }

    pub fn your_state(&self) -> &T {
        &self.current
    }

    pub fn your_state_mut(&mut self) -> &mut T {
        &mut self.current
    }

    pub fn other_users(&self) -> &HashMap<u32, UserData<T>> {
        &self.users
    }

    pub fn state(&self) -> &T {
        &self.current
    }

    pub fn state_mut(&mut self) -> &mut T {
        &mut self.current
    }

    pub fn diff(&self) -> T::Diff {
        self.current.diff(&self.old)
    }

    pub fn diff_bytes(&self) -> sge_persistence::Result<Vec<u8>> {
        self.diff().to_bytes()
    }

    pub fn diff_compressed(&mut self) -> Result<&[u8], MultiplayerError> {
        let bytes = self.diff_bytes()?;
        let mut c = Compress::new(Compression::fast(), false);

        loop {
            let status = c.compress(&bytes, &mut self.buffer, FlushCompress::Finish)?;

            if status == flate2::Status::StreamEnd {
                break;
            }

            let new_len = self.buffer.len().max(512) * 2;
            self.buffer.resize(new_len, 0);
        }
        let n = c.total_out() as usize;
        Ok(&self.buffer[..n])
    }

    pub fn uncompress_diff(&mut self, compressed: &[u8]) -> Result<T::Diff, MultiplayerError> {
        let mut d = flate2::Decompress::new(false);
        loop {
            let status = d.decompress(
                compressed,
                &mut self.buffer,
                flate2::FlushDecompress::Finish,
            )?;
            if status == flate2::Status::StreamEnd {
                break;
            }

            let new_len = self.buffer.len().max(512) * 2;
            self.buffer.resize(new_len, 0);
        }
        let n = d.total_out() as usize;
        Ok(T::Diff::from_bytes(&self.buffer[..n])?)
    }
}

pub fn init() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
}
