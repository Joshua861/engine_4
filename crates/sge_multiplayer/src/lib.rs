use std::collections::HashMap;

use backends::{Message, MultiplayerBackend, itty::IttyBackend};
use flate2::{Compress, CompressError, Compression, DecompressError, FlushCompress};
use log::{debug, info, warn};
use sge_error_union::ErrorUnion;
use sge_persistence::{Diffable, PartialLerp, Persistent};
use sge_rng::{rand, rand_f32};
use sge_time::time;
use sge_utils::ResultLoggingUtils;

pub mod backends;

pub struct MultiplayerState<T: Diffable + Persistent + Clone + PartialEq>
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
    ping_time: f32,
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
    pub last_heard_from: f32,
    pub last_ping: f32,
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

impl<T: Diffable + Persistent + Clone + PartialEq> MultiplayerState<T>
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
        let mut s = Self {
            current,
            old,
            username,
            user_id: rand(),
            buffer: vec![0; 512],
            users: HashMap::new(),
            backend: Box::new(backend),
            notifications: vec![],
            ping_time: 10.0 + rand_f32() * 5.0, // randomness so not all clients ping at the same time, probably more efficient
        };

        s.backend.init();
        s.backend.send_message(Message::Join {
            user_id: s.user_id,
            username: s.username.clone(),
        });

        s
    }

    pub fn update(&mut self) -> Result<(), MultiplayerError> {
        let messages = self.backend.recieve_messages();
        let current_time = time();

        for message in messages {
            self.handle_message(message, current_time)
                .warn_if_err("failed to handle message in `update`")?;
        }

        if self.old != self.current {
            let data = self
                .diff_compressed()
                .warn_if_err("failed to compress diff")?
                .to_vec();
            self.backend.send_message(Message::Diff {
                user_id: self.user_id,
                data,
            });

            self.old = self.current.clone();
        }

        self.check_for_stale_users();

        Ok(())
    }

    pub fn check_for_stale_users(&mut self) {
        let now = time();

        let mut to_disconnect = vec![];

        for (&id, user) in &mut self.users {
            // rand_f32 is to offset when each client sends out pings
            if now - user.last_heard_from > self.ping_time {
                // needs to be pinged

                if user.last_ping > user.last_heard_from {
                    // has been pinged
                    if now - user.last_ping > 5.0 {
                        // has been a long time with no response, probably not still connected
                        debug!("{} didnt respond to ping request", user.username);
                        to_disconnect.push(id);
                    } else {
                        // keep waiting
                    }
                } else {
                    // has not been pinged, need to ping them
                    debug!("{} is stale, pinging", user.username);
                    user.last_ping = now;
                    self.backend.send_message(Message::Ping {
                        user_id: self.user_id,
                        user: id,
                    });
                }
            } else {
                // no need to ping
            }
        }

        for user_id in to_disconnect {
            self.disconnect_user(user_id);
        }
    }

    pub fn get_user(&self, user_id: u32) -> Option<&UserData<T>> {
        self.users.get(&user_id)
    }

    pub fn get_user_mut(&mut self, user_id: u32) -> Option<&mut UserData<T>> {
        self.users.get_mut(&user_id)
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
                let len = data.len();
                self.notifications.push(Notification { user_id, data });

                if let Some(user) = self.users.get_mut(&user_id) {
                    user.last_heard_from = current_time;

                    debug!(
                        "recieved {} byte notification from user {}, username {}",
                        len, user_id, self.users[&user_id].username
                    );
                } else {
                    debug!(
                        "recieved {} byte notification from unknown user {}",
                        len, user_id
                    );
                    warn!(
                        "Multiplayer: recieved notification message from unregistered user, requesting data"
                    );

                    self.request_data(user_id);
                }
            }
            Message::AnnounceSelf { user_id, username } => {
                debug!("Recieved AnnounceSelf from {}", username);
                if let Some(user) = self.users.get_mut(&user_id) {
                    debug!("    user already registered, updating their username");
                    user.username = username;
                    user.last_heard_from = current_time;
                } else {
                    debug!("    user not registered, creating user data");
                    self.users.insert(
                        user_id,
                        UserData {
                            username,
                            current: None,
                            history: vec![],
                            last_heard_from: current_time,
                            last_ping: 0.0,
                        },
                    );
                }
            }
            Message::Join { user_id, username } => {
                if let Some(user) = self.users.get_mut(&user_id) {
                    warn!(
                        "Multiplayer: recieved join message for user already registered: {user_id} {username}"
                    );

                    user.username = username;
                    user.last_heard_from = current_time;
                } else {
                    info!("{username} joined");

                    self.users.insert(
                        user_id,
                        UserData {
                            username,
                            current: None,
                            history: vec![],
                            last_heard_from: current_time,
                            last_ping: 0.0,
                        },
                    );

                    self.announce_self()
                        .warn_if_err("failed to announce self")?;
                }
            }
            Message::Diff { user_id, data } => {
                debug!("recieved {} byte diff from {user_id}", data.len());
                let diff = self
                    .uncompress_diff(&data)
                    .warn_if_err("failed to uncompress recieved diff")?;
                if let Some(user) = self.users.get_mut(&user_id) {
                    if let Some(user_data) = &mut user.current {
                        user_data.apply_diff(diff);

                        user.history.push((current_time, user_data.clone()));
                        user.last_heard_from = current_time;

                        if user.history.len() > 15 {
                            user.history.remove(0);
                        }
                    } else {
                        warn!(
                            "Multiplayer: requesting missing data for {} {}",
                            user_id, user.username
                        );
                        self.request_data(user_id);
                    }
                } else {
                    warn!(
                        "Multiplayer: recieved diff message from unregistered user, requesting data"
                    );
                    self.request_data(user_id);
                }
            }
            Message::InitialState { user_id, data } => {
                let parsed_state = T::from_bytes(data).warn_if_err(
                    "failed to parse state from bytes recieved in inital state message",
                )?;
                if let Some(user) = self.users.get_mut(&user_id) {
                    user.current = Some(parsed_state.clone());
                    user.history = vec![(current_time, parsed_state)];
                    user.last_heard_from = current_time;
                } else {
                    self.users.insert(
                        user_id,
                        UserData {
                            username: String::new(),
                            current: Some(parsed_state.clone()),
                            history: vec![(current_time, parsed_state)],
                            last_heard_from: current_time,
                            last_ping: 0.0,
                        },
                    );
                }
            }
            Message::RequestData { user, user_id } => {
                if user == self.user_id {
                    self.announce_self().warn_if_err(
                        "failed to announce self in response to data request message",
                    )?;
                }

                if let Some(user) = self.users.get_mut(&user_id) {
                    user.last_heard_from = current_time;
                }
            }
            Message::Disconnect { user_id } => {
                self.disconnect_user(user_id);
            }
            Message::Ping { user_id, user } => {
                if let Some(user) = self.users.get_mut(&user_id) {
                    user.last_heard_from = current_time;
                }

                if user == self.user_id {
                    self.backend.send_message(Message::Pong {
                        user_id: self.user_id,
                    });
                }
            }
            Message::Pong { user_id } => {
                if let Some(user) = self.users.get_mut(&user_id) {
                    user.last_heard_from = current_time;
                }
            }
        }
        Ok(())
    }

    fn disconnect_user(&mut self, user_id: u32) {
        info!(
            "user {:?} ({}) disconnected",
            self.users.get(&user_id).map(|u| u.username.as_str()),
            user_id
        );
        self.users.remove(&user_id);
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
            data: self
                .current
                .to_bytes()
                .warn_if_err("Could not convert state to bytes in announce_self")?,
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
            last_heard_from: time(),
            last_ping: 0.0,
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
        let bytes = self
            .diff_bytes()
            .warn_if_err("could not convert diff to bytes in `diff_compressed`")?;
        let mut c = Compress::new(Compression::fast(), false);

        loop {
            let status = c
                .compress(&bytes, &mut self.buffer, FlushCompress::Finish)
                .warn_if_err("error compressing in `diff_compressed`")?;

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
            let status = d
                .decompress(
                    compressed,
                    &mut self.buffer,
                    flate2::FlushDecompress::Finish,
                )
                .warn_if_err("error decompressing in `uncompress_diff`")?;
            if status == flate2::Status::StreamEnd {
                break;
            }

            let new_len = self.buffer.len().max(512) * 2;
            self.buffer.resize(new_len, 0);
        }
        let n = d.total_out() as usize;

        // copy to u64s so it's properly aligned
        let aligned: Vec<u64> = self.buffer[..n]
            .chunks(8)
            .map(|chunk| {
                let mut arr = [0u8; 8];
                arr[..chunk.len()].copy_from_slice(chunk);
                u64::from_ne_bytes(arr)
            })
            .collect();

        let aligned_bytes = unsafe { std::slice::from_raw_parts(aligned.as_ptr() as *const u8, n) };

        Ok(T::Diff::from_bytes(aligned_bytes)
            .warn_if_err("error converting diff to bytes in `uncompress_diff`")?)
    }
}

pub fn init() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
}
