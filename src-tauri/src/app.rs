use std::{env, path::PathBuf, sync::Arc};

use anyhow::Result;
use once_cell::sync::OnceCell;
use tauri::{App, Config};

use crate::{
    config::{self, ConfigManager},
    logger,
    poe::{self, PoeClientManager},
};

static CONFIG_MANAGER: OnceCell<ConfigManager> = OnceCell::new();
static POE_CLIENT_MANAGER: OnceCell<PoeClientManager> = OnceCell::new();
static TAURI_CONFIG: OnceCell<Arc<Config>> = OnceCell::new();

pub fn init_app(app: &mut App) -> Result<()> {
    if TAURI_CONFIG.set(app.config().clone()).is_err() {
        return Err(anyhow::anyhow!("init TAURI_CONFIG failed"));
    }

    let log_dir = env::current_exe()
        .ok()
        .ok_or(anyhow::anyhow!("cannot get exe dir"))?
        .parent()
        .ok_or(anyhow::anyhow!("cannot get log dir"))?
        .join("logs");

    logger::init(&log_dir)?;

    // 需要与wails版本使用的 https://github.com/adrg/xdg 兼容
    let config_dir = match env::var_os("LOCALAPPDATA") {
        Some(val) => val,
        None => return Err(anyhow::anyhow!("cannot get config dir")),
    };

    init_config_manager(
        &PathBuf::from(config_dir)
            .join(TAURI_CONFIG.get().unwrap().tauri.bundle.identifier.as_str()),
    )?;

    init_poe_client_manager(&app_config_manager().get().poesessid)?;

    Ok(())
}

const CONFIG_FILE_NAME: &str = "config.json";

fn init_config_manager(config_dir: &PathBuf) -> Result<()> {
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    let manager = config::ConfigManager::new(config_path);

    match CONFIG_MANAGER.set(manager) {
        Ok(()) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("it should never happen")),
    }
}

fn init_poe_client_manager(poesessid: &str) -> Result<()> {
    let manager = poe::PoeClientManager::new(poesessid)?;

    match POE_CLIENT_MANAGER.set(manager) {
        Ok(()) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("it should never happen")),
    }
}

pub fn tauri_config() -> Arc<Config> {
    return TAURI_CONFIG
        .get()
        .expect("tauri config is not initialized")
        .clone();
}

pub fn app_config_manager() -> &'static ConfigManager {
    return CONFIG_MANAGER
        .get()
        .expect("config manager is not initialized");
}

pub fn poe_client_manager() -> &'static PoeClientManager {
    return POE_CLIENT_MANAGER
        .get()
        .expect("poe client manager is not initialized");
}
