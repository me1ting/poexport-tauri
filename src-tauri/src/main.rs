// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
mod config;
mod logger;
mod utils;
mod poe;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {}))
        .invoke_handler(tauri::generate_handler![
            cmds::get_config,
            cmds::reset_config,
            cmds::set_poe_session_id,
            cmds::set_pob_path,
            cmds::set_pob_proxy_supported,
        ])
        .setup(|app| {
            if let Err(err) = utils::init::init_app(app) {
                // if the log is not initialized successfully, it is just invalid
                // https://docs.rs/log/latest/log/#in-executables
                log::error!("failed to init application: {:#}", err);
                utils::panic_dialog(&err);
                app.handle().exit(-1);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
