use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub started_at: DateTime<Local>,
    pub ended_at: DateTime<Local>,
    pub planned_seconds: u64,
    pub actual_seconds: u64,
    pub session_type: String,
    pub task: Option<String>,
    pub tags: Vec<String>,
    pub completed: bool,
    pub profile: String,
    pub theme: Option<String>,
    pub energy: Option<u8>,
    pub mood: Option<String>,
    pub intention: Option<String>,
    pub note: Option<String>,
}
