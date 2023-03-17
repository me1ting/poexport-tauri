use anyhow::Result;
use tauri::App;

use crate::{config, logger};

pub fn init_app(app: &mut App) -> Result<()> {
    let log_dir = app
        .path_resolver()
        .app_log_dir()
        .ok_or(anyhow::anyhow!("could not get log dir"))?;

    logger::init(&log_dir)?;

    let config_dir = app
        .path_resolver()
        .app_config_dir()
        .ok_or(anyhow::anyhow!("could not get config dir"))?;

    config::init(&config_dir)?;

    Ok(())
}
