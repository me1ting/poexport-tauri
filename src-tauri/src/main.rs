// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
mod config;
mod logger;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {}))
        .invoke_handler(tauri::generate_handler![cmds::get_config])
        .setup(|app| {
            if let Err(err) = utils::init::init_app(app) {
                utils::panic_dialog(&err);
                app.handle().exit(-1);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
