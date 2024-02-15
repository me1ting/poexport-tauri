use serde::{Deserialize, Serialize};

use crate::config::Config;

use crate::poe::{self, PoeClient};
use crate::{app, updater, wrap_err};

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn get_config() -> CmdResult<Config> {
    let manager = app::app_config_manager();
    Ok(manager.get())
}

#[tauri::command]
pub fn reset_config() -> CmdResult<Config> {
    let manager = app::app_config_manager();
    let r = wrap_err!(manager.reset())?;
    Ok(r)
}

#[tauri::command]
pub async fn set_poesessid(poesessid: String) -> CmdResult {
    let manager = app::app_config_manager();
    let r = wrap_err!(manager.set_poesessid(&poesessid))?;

    let poe_client_manager = app::poe_client_manager();
    let poe_client = wrap_err!(PoeClient::new(poe::TX_POE_HOST, &poesessid))?;
    poe_client_manager.set(poe_client).await;

    Ok(r)
}

#[tauri::command]
pub async fn get_characters(account_name: String) -> CmdResult<String> {
    let manager = app::poe_client_manager();
    let r = match manager
        .get()
        .await
        .get_characters(&account_name, "pc")
        .await
    {
        Ok(r) => r,
        Err(e) => return CmdResult::Err(e.to_string()),
    };

    return Ok(r);
}

#[tauri::command]
pub async fn get_items(account_name: String, character: String) -> CmdResult<String> {
    let manager = app::poe_client_manager();
    let r = wrap_err!(
        manager
            .get()
            .await
            .get_items(&account_name, &character, "pc")
            .await
    )?;

    return Ok(r);
}

#[tauri::command]
pub async fn get_passive_skills(account_name: String, character: String) -> CmdResult<String> {
    let manager = app::poe_client_manager();
    let r = wrap_err!(
        manager
            .get()
            .await
            .get_passive_skills(&account_name, &character, "pc")
            .await
    )?;

    return Ok(r);
}

#[derive(Serialize, Deserialize)]
pub struct UpdateInfo {
    need_update: bool,
    current: String,
    latest: String,
    changelog: String,
}

#[tauri::command]
pub async fn check_for_update() -> CmdResult<UpdateInfo> {
    let tauri_config = app::tauri_config();
    let current = tauri_config.package.version.as_ref().unwrap();
    let latest = wrap_err!(updater::check_for_update().await)?;

    let need_update = wrap_err!(tauri::api::version::compare(current, &latest.latest))? > 0;

    let info = UpdateInfo {
        need_update,
        current: String::from(current),
        latest: latest.latest,
        changelog: latest.changelog,
    };

    CmdResult::Ok(info)
}
