use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use serde_json::{Value, json};
use tungstenite::{Message, connect, stream::MaybeTlsStream};

enum OutboundCmd {
    Send(String),
    Close,
}

#[derive(Clone)]
pub struct IttySocket {
    channel_id: String,
    options: HashMap<String, String>,
    outbound_tx: std::sync::mpsc::Sender<OutboundCmd>,
    inbound_rx: Arc<Mutex<std::sync::mpsc::Receiver<Value>>>,
    start_trigger: Arc<
        Mutex<
            Option<(
                std::sync::mpsc::Receiver<OutboundCmd>,
                std::sync::mpsc::Sender<Value>,
            )>,
        >,
    >,
}

pub struct IttySocketOptions {
    pub alias: Option<String>,
    pub echo: bool,
    pub announce: bool,
}

impl Default for IttySocketOptions {
    fn default() -> Self {
        Self {
            alias: None,
            echo: false,
            announce: false,
        }
    }
}

impl IttySocket {
    pub fn connect(channel_id: impl Into<String>, opts: IttySocketOptions) -> Self {
        let mut options = HashMap::new();
        if let Some(alias) = opts.alias {
            options.insert("as".into(), alias);
        }
        if opts.echo {
            options.insert("echo".into(), "false".into());
        }
        if opts.announce {
            options.insert("announce".into(), "true".into());
        }

        let (out_tx, out_rx) = std::sync::mpsc::channel();
        let (in_tx, in_rx) = std::sync::mpsc::channel();

        Self {
            channel_id: channel_id.into(),
            options,
            outbound_tx: out_tx,
            inbound_rx: Arc::new(Mutex::new(in_rx)),
            start_trigger: Arc::new(Mutex::new(Some((out_rx, in_tx)))),
        }
    }

    fn build_url(&self) -> String {
        let query: String = self
            .options
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let base = if self.channel_id.starts_with("ws://") || self.channel_id.starts_with("wss://")
        {
            self.channel_id.clone()
        } else {
            format!("wss://itty.ws/c/{}", self.channel_id)
        };

        if query.is_empty() {
            base
        } else {
            format!("{}?{}", base, query)
        }
    }

    pub fn drain(&self) -> Vec<Value> {
        self.open();
        let rx = self.inbound_rx.lock().unwrap();
        let mut out = Vec::new();

        while let Ok(msg) = rx.try_recv() {
            out.push(msg);
        }
        out
    }

    pub fn send_to(&self, message: impl serde::Serialize, recipient: Option<&str>) -> &Self {
        let payload = serde_json::to_string(&json!(message)).unwrap();
        let raw = match recipient {
            Some(uid) => format!("\x1F{}\x1F{}", uid, payload),
            None => payload,
        };

        let _ = self.outbound_tx.send(OutboundCmd::Send(raw));
        self.open();
        self
    }

    pub fn send(&self, message: impl serde::Serialize) -> &Self {
        self.send_to(message, None)
    }

    pub fn push(&self, message: impl serde::Serialize) -> &Self {
        self.send(message);
        self.close();
        self
    }

    pub fn close(&self) -> &Self {
        let _ = self.outbound_tx.send(OutboundCmd::Close);
        self
    }

    pub fn open(&self) -> &Self {
        let mut lock = self.start_trigger.lock().unwrap();
        let channels = lock.take();
        drop(lock);

        if let Some((outbound_rx, inbound_tx)) = channels {
            let url = self.build_url();

            thread::spawn(move || {
                let conn = connect(&url);
                let mut ws = match conn {
                    Ok((w, _)) => w,
                    Err(e) => {
                        let _ =
                            inbound_tx.send(json!({ "type": "error", "message": e.to_string() }));
                        return;
                    }
                };

                let timeout = Some(Duration::from_millis(10));
                match ws.get_mut() {
                    MaybeTlsStream::Plain(s) => {
                        let _ = s.set_read_timeout(timeout);
                    }
                    MaybeTlsStream::Rustls(s) => {
                        let _ = s.get_mut().set_read_timeout(timeout);
                    }
                    _ => {}
                }

                loop {
                    while let Ok(cmd) = outbound_rx.try_recv() {
                        match cmd {
                            OutboundCmd::Send(msg) => {
                                if ws.send(Message::Text(msg.into())).is_err() {
                                    return;
                                }
                            }
                            OutboundCmd::Close => {
                                let _ = ws.send(Message::Close(None));
                                return;
                            }
                        }
                    }

                    match ws.read() {
                        Ok(Message::Text(text)) => {
                            let parsed: Value = match serde_json::from_str(&text) {
                                Ok(v) => v,
                                Err(_) => continue,
                            };

                            let payload_raw = &parsed["message"];
                            let event_payload = if payload_raw.is_object() {
                                let mut ep = payload_raw.clone();
                                if let (Some(obj), Some(parsed_obj)) =
                                    (ep.as_object_mut(), parsed.as_object())
                                {
                                    for (k, v) in parsed_obj {
                                        obj.entry(k).or_insert_with(|| v.clone());
                                    }
                                }
                                ep
                            } else {
                                parsed.clone()
                            };

                            if inbound_tx.send(event_payload).is_err() {
                                return;
                            }
                        }
                        Ok(Message::Close(_)) => return,
                        Err(tungstenite::Error::Io(ref e))
                            if e.kind() == std::io::ErrorKind::WouldBlock
                                || e.kind() == std::io::ErrorKind::TimedOut => {}
                        Err(_) => return,
                        _ => {}
                    }

                    thread::sleep(Duration::from_millis(5));
                }
            });
        }
        self
    }
}
