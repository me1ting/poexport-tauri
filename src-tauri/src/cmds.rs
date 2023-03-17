use crate::config::{self, Config};

use crate::wrap_err;

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn get_config() -> CmdResult<Config> {
    let manager = config::ConfigManager::global();
    Ok(manager.get_config())
}

#[tauri::command]
pub fn reset_config() -> CmdResult<Config> {
    let manager = config::ConfigManager::global();
    let result = wrap_err!(manager.reset_config())?;
    Ok(result)
}

#[tauri::command]
pub fn set_poe_session_id(id: String) -> CmdResult {
    let manager = config::ConfigManager::global();
    let result = wrap_err!(manager.set_poe_session_id(&id))?;
    Ok(result)
}

#[tauri::command]
pub fn set_pob_path(path: String) -> CmdResult {
    let manager = config::ConfigManager::global();
    let result = wrap_err!(manager.set_pob_path(&path))?;
    Ok(result)
}

#[tauri::command]
pub fn set_pob_proxy_supported(supported: bool) -> CmdResult {
    let manager = config::ConfigManager::global();
    let result = wrap_err!(manager.set_pob_proxy_supported(supported))?;
    Ok(result)
}
