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
}

impl App {
    pub fn new(file_path: PathBuf) -> Self {
        let (tasks, error) = match storage::load(&file_path) {
            Ok(t) => (t, None),
            Err(e) => (Vec::new(), Some(format!("Load error: {e}"))),
        };
        Self {
            tasks,
            selected_index: 0,
            input_mode: false,
            input_buffer: String::new(),
            file_path,
            should_quit: false,
            error,
        }
    }

    fn persist(&mut self) {
        self.error = storage::save(&self.file_path, &self.tasks)
            .err()
            .map(|e| format!("Save error: {e}"));
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
        if let Some(task) = self.tasks.get_mut(self.selected_index) {
            task.done = !task.done;
            self.persist();
        }
    }

    pub fn delete_selected(&mut self) {
        if self.selected_index < self.tasks.len() {
            self.tasks.remove(self.selected_index);
            if self.selected_index >= self.tasks.len() {
                self.selected_index = self.tasks.len().saturating_sub(1);
            }
            self.persist();
        }
    }

    pub fn start_input(&mut self) {
        self.input_mode = true;
        self.input_buffer.clear();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = false;
        self.input_buffer.clear();
    }

    pub fn submit_input(&mut self) {
        let text = self.input_buffer.trim().to_string();
        self.input_mode = false;
        self.input_buffer.clear();
        if text.is_empty() {
            return;
        }
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task { id, text, done: false });
        self.selected_index = self.tasks.len() - 1;
        self.persist();
    }
}

#[allow(dead_code)]
pub type AppResult<T> = io::Result<T>;
