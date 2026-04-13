mod app;

use app::commands;
use app::state::SharedProject;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(SharedProject::default())
        .invoke_handler(tauri::generate_handler![
            commands::workspace_open,
            commands::preview_render,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Docwise");
}
