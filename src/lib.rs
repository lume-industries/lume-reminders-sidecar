use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReminderItem {
    pub title: String,
    pub due: String,
    pub priority: String,
    pub list: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemindersPayload {
    pub fetched_at: String,
    pub reminders: Vec<ReminderItem>,
}
