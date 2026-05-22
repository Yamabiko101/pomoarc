use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub focus_minutes: u64,
    pub short_break_minutes: u64,
    pub long_break_minutes: u64,
    pub long_break_every: u32,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            focus_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            long_break_every: 4,
        }
    }
}

impl Profile {
    pub fn deep_work() -> Self {
        Self {
            focus_minutes: 50,
            short_break_minutes: 10,
            long_break_minutes: 25,
            long_break_every: 3,
        }
    }

    pub fn micro() -> Self {
        Self {
            focus_minutes: 10,
            short_break_minutes: 2,
            long_break_minutes: 8,
            long_break_every: 5,
        }
    }
}
