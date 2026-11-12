use std::fs;
use std::io;
use std::path::PathBuf;

use crate::app::Task;

/// مسیر فایل tasks.json: پوشه home کاربر، یا پوشه جاری اگر home پیدا نشد.
pub fn default_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tasks.json")
}

/// بارگذاری تسک‌ها؛ اگر فایل وجود نداشته باشد لیست خالی برمی‌گرداند.
pub fn load(path: &PathBuf) -> io::Result<Vec<Task>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    if data.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// ذخیره تسک‌ها (ابتدا در فایل موقت نوشته می‌شود تا فایل اصلی خراب نشود).
pub fn save(path: &PathBuf, tasks: &[Task]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(&tmp, path)
}
