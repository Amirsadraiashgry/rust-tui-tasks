use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::storage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub text: String,
    pub done: bool,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub input_mode: bool,
    pub input_buffer: String,
    pub file_path: PathBuf,
    pub should_quit: bool,
    /// آخرین خطای ذخیره‌سازی (برای نمایش در نوار وضعیت)
    pub error: Option<String>,
    /// اگر بارگذاری اولیه شکست بخورد، اجازه نوشتن روی فایل اصلی را نمی‌دهیم.
    storage_writable: bool,
}

impl App {
    pub fn new(file_path: PathBuf) -> Self {
        let (tasks, error, storage_writable) = match storage::load(&file_path) {
            Ok(tasks) => (tasks, None, true),
            Err(error) => (
                Vec::new(),
                Some(format!("Load error: {error}")),
                false,
            ),
        };

        Self {
            tasks,
            selected_index: 0,
            input_mode: false,
            input_buffer: String::new(),
            file_path,
            should_quit: false,
            error,
            storage_writable,
        }
    }

    fn persist(&mut self) {
        if !self.storage_writable {
            return;
        }

        match storage::save(&self.file_path, &self.tasks) {
            Ok(()) => self.error = None,
            Err(error) => self.error = Some(format!("Save error: {error}")),
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index + 1 < self.tasks.len() {
            self.selected_index += 1;
        }
    }

    pub fn toggle_selected(&mut self) {
        if !self.storage_writable {
            return;
        }
        if let Some(task) = self.tasks.get_mut(self.selected_index) {
            task.done = !task.done;
            self.persist();
        }
    }

    pub fn delete_selected(&mut self) {
        if !self.storage_writable {
            return;
        }
        if self.selected_index < self.tasks.len() {
            self.tasks.remove(self.selected_index);
            if self.selected_index >= self.tasks.len() {
                self.selected_index = self.tasks.len().saturating_sub(1);
            }
            self.persist();
        }
    }

    pub fn start_input(&mut self) {
        if !self.storage_writable {
            return;
        }
        self.input_mode = true;
        self.input_buffer.clear();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = false;
        self.input_buffer.clear();
    }

    pub fn submit_input(&mut self) {
        if !self.storage_writable {
            return;
        }

        let text = self.input_buffer.trim().to_string();
        if text.is_empty() {
            self.cancel_input();
            return;
        }

        let next_id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0);
        let Some(id) = next_id.checked_add(1) else {
            self.error = Some("Task ID limit reached; no task was added.".to_string());
            return;
        };

        self.input_mode = false;
        self.input_buffer.clear();
        self.tasks.push(Task {
            id,
            text,
            done: false,
        });
        self.selected_index = self.tasks.len() - 1;
        self.persist();
    }
}

#[allow(dead_code)]
pub type AppResult<T> = io::Result<T>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_file(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rust-tui-tasks-{name}-{nonce}.json"))
    }

    #[test]
    fn invalid_json_is_never_overwritten_by_app_actions() {
        let path = test_file("invalid-json");
        let original = b"{ this is not valid JSON";
        fs::write(&path, original).expect("write fixture");

        let mut app = App::new(path.clone());
        app.start_input();
        app.input_buffer = "A new task".to_string();
        app.submit_input();
        app.delete_selected();
        app.toggle_selected();

        assert!(!app.input_mode);
        assert!(app.tasks.is_empty());
        assert_eq!(fs::read(&path).expect("read original file").as_slice(), original);

        fs::remove_file(path).expect("remove fixture");
    }

    #[test]
    fn task_id_overflow_does_not_panic_or_add_a_task() {
        let path = test_file("id-overflow");
        let tasks = vec![Task {
            id: u64::MAX,
            text: "Existing task".to_string(),
            done: false,
        }];
        fs::write(&path, serde_json::to_vec(&tasks).expect("serialize fixture"))
            .expect("write fixture");

        let mut app = App::new(path.clone());
        app.start_input();
        app.input_buffer = "Another task".to_string();
        app.submit_input();

        assert_eq!(app.tasks.len(), 1);
        assert!(app.error.as_deref().is_some_and(|message| message.contains("limit")));

        fs::remove_file(path).expect("remove fixture");
    }
}
