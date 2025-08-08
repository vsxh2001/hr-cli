use storage::Storage;
use home::home_dir;

pub mod commands;
pub mod models;
pub mod storage;


fn main() {
    let default_storage_path = home_dir().unwrap().join(".hr_data");
    let storage_path = std::env::var("HR_STORAGE_PATH")
        .unwrap_or_else(|_| default_storage_path.to_string_lossy().to_string());
    let storage = Storage::new(storage_path);
    commands::run(&storage);
}
