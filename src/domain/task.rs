use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub tag: String,
    pub created_at: DateTime<Local>,
    pub done: bool,
}

impl Task {
    pub fn new(title: String, tag: Option<String>) -> Self {
        Self {
            title,
            tag: tag.unwrap_or_else(|| "custom".to_string()),
            created_at: Local::now(),
            done: false,
        }
    }
}
