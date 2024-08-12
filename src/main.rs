// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod menu;

fn main() {
    let menu = menu::create_menu();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .menu(menu)
        .invoke_handler(tauri::generate_handler![
            commands::user::get_user,
            commands::user::create_user,
            commands::dataset::get_dataset,
            commands::dataset::list_datasets,
            commands::dataset::create_dataset,
            commands::collection::get_collection,
            commands::collection::list_collections,
            commands::collection::create_collection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
