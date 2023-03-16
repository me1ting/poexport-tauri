use tauri::api::dialog;

pub mod init;

/// Use blocking native message dialog to show error.
/// 
/// Should NOT be used when running on the main thread context, unless you end the program after.
///
/// The function only to show error causing panic, and you should end the program after.
pub fn panic_dialog(err: &anyhow::Error) {
    dialog::blocking::MessageDialogBuilder::new("Error", format!("{:#}", err))
        .kind(dialog::MessageDialogKind::Error)
        .show();
}
