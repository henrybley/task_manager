use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub due_date_time: chrono::NaiveDateTime,
    pub completed: bool,
}

impl Task {
    pub fn new(title: String, description: String, due_date_time: chrono::NaiveDateTime) -> Self {
        Task {
            title,
            description,
            due_date_time,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true
    }
    pub fn get_completed(&self) -> String {
        if (self.completed) {
            "[Complete]".to_string()
        } else {
            "[Incomplete]".to_string()
        }
    }
}
