#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod dao;
mod menu;
mod model;
mod util;

use std::{sync::{mpsc, atomic::{AtomicU64, Ordering}}, thread};

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
            show_window,
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
    let (tx, rx) = mpsc::channel::<String>();

    mouse_event(tx);

    let app_ref = app.handle().clone();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(payload) => {
                    app_ref.emit_all(payload.as_str(), "").unwrap()
                },
                Err(e) => {},
            }
        }
    });

    app.listen_global("keydown".to_string(), |_event| {
        println!("监听到事件");
    });

    Ok(())
}

fn mouse_event(tx: std::sync::mpsc::Sender<String>) {
    use mouce::Mouse;

    thread::spawn(|| {

        let mut mouse_manager = Mouse::new();


        let hook_result = mouse_manager.hook(Box::new(move |e| {
            let flag: AtomicU64 = AtomicU64::new(1);

            if let mouce::common::MouseEvent::Press(mouce::common::MouseButton::Left) = e {
                match tx.send("drag".to_string()) {
                    Ok(o) => {},
                    Err(e) => {
                        println!("发送失败, {:?}", e.to_string());
                    },
                }
            }

            if let mouce::common::MouseEvent::Release(mouce::common::MouseButton::Left) = e {
                match tx.send("release".to_string()) {
                    Ok(o) => {},
                    Err(e) => {
                        println!("发送失败, {:?}", e.to_string());
                    },
                }
            }

        }));

        match hook_result {
            Ok(id) => {
                println!("监听返回成功, id:{}", id);
                // assert_eq!(mouse_manager.unhook(id), Ok(()));
            }
            // Hooking may require user privileges on some systems
            // e.g. requires super user for Linux
            Err(err) => {
                println!("监听 hook 失败 {}", err);
            }
        }
    });
}
