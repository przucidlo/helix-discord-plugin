use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Message {
    cmd: String,
    nonce: String,
    args: Value,
    evt: Option<String>,
}

impl Message {
    pub fn idle_activity() -> Self {
        Message {
            cmd: String::from("SET_ACTIVITY"),
            nonce: Uuid::new_v4().to_string(),
            args: json!({
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
            }),
            evt: None,
        }
    }

    pub fn file_activity(file: &str) -> Self {
        Message {
            cmd: String::from("SET_ACTIVITY"),
            nonce: Uuid::new_v4().to_string(),
            args: json!({
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
            }),
            evt: None,
        }
    }
}
