use anyhow::Result;
use tauri::App;

use crate::logger;

pub fn init_app(app: &mut App) -> Result<()> {
    let log_dir = app
        .path_resolver()
        .app_log_dir()
        .ok_or(anyhow::anyhow!("failed to get app log dir"))?;
    logger::init(&log_dir)?;

    Ok(())
}
