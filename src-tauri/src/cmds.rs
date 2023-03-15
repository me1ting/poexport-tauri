use crate::config::{self, Config};

#[tauri::command]
pub fn get_config() -> Config {
    let manager = config::ConfigManager::global();
    manager.get_config()
}
