// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cmds;
mod config;
mod logger;
mod poe;
mod updater;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmds::get_config,
            cmds::set_poesessid,
            cmds::get_characters,
            cmds::get_items,
            cmds::get_passive_skills,
            cmds::check_for_update,
        ])
        .setup(|app| {
            if let Err(err) = app::init_app(app) {
                utils::error::panic_dialog(&err);
                app.handle().exit(-1);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
