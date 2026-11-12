use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
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

    serde_json::from_str(&data)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

/// ذخیره اتمیک: داده ابتدا در فایل موقت منحصربه‌فرد نوشته و sync می‌شود؛
/// فقط پس از موفقیت کامل، فایل مقصد جایگزین خواهد شد.
pub fn save(path: &PathBuf, tasks: &[Task]) -> io::Result<()> {
    let json = serde_json::to_vec_pretty(tasks)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid task file path"))?;
    let temp_name = format!(
        ".{}.{}.{}.tmp",
        file_name.to_string_lossy(),
        std::process::id(),
        TEMP_FILE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    );
    let temp_path = path.with_file_name(temp_name);

    let result = write_and_replace(&temp_path, path, &json);
    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result
}

static TEMP_FILE_COUNTER: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

fn write_and_replace(temp_path: &PathBuf, path: &PathBuf, data: &[u8]) -> io::Result<()> {
    let mut file: File = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp_path)?;
    file.write_all(data)?;
    file.sync_all()?;
    drop(file);
    fs::rename(temp_path, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_file(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rust-tui-tasks-{name}-{nonce}.json"))
    }

    #[test]
    fn missing_file_loads_as_empty_list() {
        let path = test_file("missing");
        assert!(load(&path).expect("load missing file").is_empty());
    }

    #[test]
    fn save_and_load_round_trip() {
        let path = test_file("round-trip");
        let tasks = vec![Task {
            id: 1,
            text: "یادگیری Rust".to_string(),
            done: true,
        }];

        save(&path, &tasks).expect("save tasks");
        assert_eq!(load(&path).expect("load tasks")[0].text, tasks[0].text);
        assert_eq!(load(&path).expect("load tasks")[0].done, true);

        fs::remove_file(path).expect("remove fixture");
    }

    #[test]
    fn invalid_json_returns_an_error() {
        let path = test_file("invalid");
        fs::write(&path, b"{invalid").expect("write fixture");

        assert_eq!(load(&path).unwrap_err().kind(), io::ErrorKind::InvalidData);

        fs::remove_file(path).expect("remove fixture");
    }
}
