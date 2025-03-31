use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename="time")]
    pub timestamp: i64, // Unix epoch seconds (UTC) (Milliseconds since epoch)
    #[serde(rename="log")]
    pub message: String,
}