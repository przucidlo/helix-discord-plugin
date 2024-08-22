use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Message {
    cmd: String,
    nonce: Option<String>,
    args: Option<Value>,
    evt: Option<String>,
}

impl Message {
    pub fn idle_activity() -> Self {
        Message {
            cmd: String::from("SET_ACTIVITY"),
            nonce: Some(Uuid::new_v4().to_string()),
            args: Some(json!({
                "pid": 1,
                "activity": {
                    "state": "Idling",
                    "timestamps": {
                        "start": SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    },
                    "instance": true,
                    "assets": {
                        "large_image": "idle",
                        "large_text": "Idling",
                        "small_image": "idle",
                        "small_text": "Helix"
                    }
                },
            })),
            evt: None,
        }
    }

    pub fn file_activity(file: &str) -> Self {
        Message {
            cmd: String::from("SET_ACTIVITY"),
            nonce: Some(Uuid::new_v4().to_string()),
            args: Some(json!({
                "pid": 1,
                "activity": {
                    "state": format!("Editing {file}"),
                    "timestamps": {
                        "start": SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    },
                    "instance": true,
                    "assets": {
                        "large_image": "idle",
                        "large_text": "Editing file",
                        "small_image": "idle",
                        "small_text": "Helix"
                    }
                },
            })),
            evt: None,
        }
    }

    pub fn evt_matches(&self, evt: &str) -> bool {
        if let Some(e) = &self.evt {
            return evt == e;
        }

        return false;
    }
}

impl TryFrom<Vec<u8>> for Message {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let json_string = String::from_utf8(value)?;

        serde_json::from_str::<Message>(&json_string).map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}
