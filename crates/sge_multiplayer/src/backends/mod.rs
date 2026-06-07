pub mod itty;
pub mod lan;

pub trait MultiplayerBackend {
    fn send_message(&mut self, message: Message);
    fn recieve_messages(&mut self) -> Vec<Message>;
    fn init(&mut self);
    fn close(&mut self);
}

#[derive(Debug)]
pub enum Message {
    InitialState { user_id: u32, data: Vec<u8> },
    Diff { user_id: u32, data: Vec<u8> },
    Join { user_id: u32, username: String },
    AnnounceSelf { user_id: u32, username: String },
    RequestData { user_id: u32, user: u32 },
    Disconnect { user_id: u32 },
    Notification { user_id: u32, data: Vec<u8> },
    Ping { user_id: u32, user: u32 },
    Pong { user_id: u32 },
}
