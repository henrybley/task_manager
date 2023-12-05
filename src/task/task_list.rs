use std::{fs, io};

use serde_json;

pub use crate::Task;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl<'a> TaskList {
    pub fn new() -> Self {
        TaskList { tasks: vec![] }
    }
    pub fn load() -> Self {
        let file_path = "src/files/task_list.json";
        let data = fs::read_to_string(file_path).expect("Should have been able to read the file");
        serde_json::from_str(data.as_str()).unwrap()
    }
    pub fn save(&self) -> io::Result<()> {
        let file_path = "src/files/task_list.json";
        let data = serde_json::to_string(self).unwrap();
        fs::write(file_path, data)
    }
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn get_all(&'a self) -> &'a Vec<Task> {
        &self.tasks
    }
    pub fn get(&'a self, i: usize) -> Option<&'a Task> {
        self.tasks.get(i)
    }
    pub fn complete_task(&mut self, task_id: usize) -> Result<String, String> {
        match self.tasks.get_mut(task_id) {
            None => Err("Task Not Found!".to_string()),
            Some(task) => {
                task.complete();
                Ok("Task Completed".to_string())
            }
        }
    }
}
