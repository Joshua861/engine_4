use super::{Message, MultiplayerBackend};
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sys::{IttySocket, IttySocketOptions};
pub mod sys;

pub struct IttyBackend {
    socket: IttySocket,
}

impl IttyBackend {
    pub fn new(room_name: String) -> Self {
        let socket = IttySocket::connect(
            format!("{}:{}", Self::NAMESPACE, room_name),
            IttySocketOptions {
                alias: None,
                echo: false,
                announce: false,
            },
        );
        Self { socket }
    }

    fn send_payload(&mut self, user_id: u32, data: String, ty: &'static str) {
        self.socket.send(IttyMessage {
            ty: ty.to_string(),
            user_id,
            data,
        });
    }
}

impl IttyBackend {
    pub const NAMESPACE: &str = "sge-multiplayer";
    pub const JOIN_KEY: &str = "lsF7PFPAP0B7wkT2gouLzSwC";
}

const INITIAL_STATE: &str = "inital-state";
const DIFF: &str = "diff";
const JOIN: &str = "join";
const ANNOUNCE_SELF: &str = "announce-self";
const REQUEST_DATA: &str = "request-data";
const DISCONNECT: &str = "disconnect";
const NOTIFICATION: &str = "notification";
const PING: &str = "ping";
const PONG: &str = "pong";

#[derive(Serialize, Deserialize)]
pub struct IttyMessage {
    #[serde(rename = "type")]
    ty: String,
    user_id: u32,
    data: String,
}

#[allow(deprecated)]
impl MultiplayerBackend for IttyBackend {
    fn send_message(&mut self, message: Message) {
        match message {
            Message::Diff { user_id, data } => {
                self.send_payload(user_id, base64::encode(data), DIFF)
            }
            Message::InitialState { user_id, data } => {
                self.send_payload(user_id, base64::encode(data), INITIAL_STATE)
            }
            Message::Join { user_id, username } => {
                self.send_payload(user_id, username, JOIN);
            }
            Message::AnnounceSelf { user_id, username } => {
                self.send_payload(user_id, username, ANNOUNCE_SELF);
            }
            Message::RequestData { user_id, user } => {
                self.send_payload(user_id, user.to_string(), REQUEST_DATA);
            }
            Message::Disconnect { user_id } => {
                self.send_payload(user_id, String::new(), DISCONNECT);
            }
            Message::Notification { user_id, data } => {
                self.send_payload(user_id, base64::encode(data), NOTIFICATION)
            }
            Message::Ping { user_id, user } => {
                self.send_payload(user_id, user.to_string(), PING);
            }
            Message::Pong { user_id } => {
                self.send_payload(user_id, String::new(), PONG);
            }
        }
    }

    fn recieve_messages(&mut self) -> Vec<Message> {
        self.socket
            .drain()
            .into_iter()
            .filter_map(|raw| {
                fn inner(raw: &Value) -> Option<Message> {
                    let target_value = if raw.get("message").is_some() {
                        raw.get("message")?.clone()
                    } else {
                        raw.clone()
                    };

                    let msg: IttyMessage = serde_json::from_value(target_value).ok()?;

                    match msg.ty.as_str() {
                        DIFF => Some(Message::Diff {
                            user_id: msg.user_id,
                            data: base64::decode(&msg.data).ok()?,
                        }),
                        INITIAL_STATE => Some(Message::InitialState {
                            user_id: msg.user_id,
                            data: base64::decode(&msg.data).ok()?,
                        }),
                        JOIN => Some(Message::Join {
                            user_id: msg.user_id,
                            username: msg.data,
                        }),
                        ANNOUNCE_SELF => Some(Message::AnnounceSelf {
                            user_id: msg.user_id,
                            username: msg.data,
                        }),
                        REQUEST_DATA => Some(Message::RequestData {
                            user_id: msg.user_id,
                            user: msg.data.parse().ok()?,
                        }),
                        DISCONNECT => Some(Message::Disconnect {
                            user_id: msg.user_id,
                        }),
                        NOTIFICATION => Some(Message::Notification {
                            user_id: msg.user_id,
                            data: base64::decode(&msg.data).ok()?,
                        }),
                        PING => Some(Message::Ping {
                            user_id: msg.user_id,
                            user: msg.data.parse().ok()?,
                        }),
                        PONG => Some(Message::Pong {
                            user_id: msg.user_id,
                        }),
                        _ => None,
                    }
                }

                match inner(&raw) {
                    Some(v) => Some(v),
                    None => {
                        warn!(
                            "Received malformed message or unhandled server frame. Will be ignored."
                        );
                        None
                    }
                }
            })
            .collect()
    }

    fn init(&mut self) {
        self.socket.open();
    }

    fn close(&mut self) {
        self.socket.close();
    }
}
