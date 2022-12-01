#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dao;
mod menu;
mod api;
mod model;
mod util;

use api::*;
use tauri::Manager;

fn main() {
    let mut _app = tauri::Builder::default()
        .setup(set_up)
        .invoke_handler(tauri::generate_handler![
            init_table,
            add_images_meta,
            get_images_meta_list,
            get_images_folder_info,
            get_images_meta,
            update_browse_settings,
            get_browse_settings,
            delete_images_meta,
            update_images_meta,
            upload_file,
            hide_window,
            get_file_meta_list,
            
        ])
        // 菜单
        .menu(menu::menu())
        // 系统托盘
        .system_tray(menu::system_menu())
        // 系统托盘事件，必须有
        .on_system_tray_event(menu::system_menu_handler)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn set_up(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    app.listen_global("keydown".to_string(), |event| {
        println!("监听到事件");
    });

    Ok(())
}