use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: u64,
    pub body: String,
    #[serde(default)]
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Note {
    pub fn new(id: u64, body: String) -> Self {
        let now = Local::now();
        Self {
            id,
            body,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }
}
