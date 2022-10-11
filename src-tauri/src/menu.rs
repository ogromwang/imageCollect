use tauri::{
    api::dialog::message, AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Window,
};

pub fn menu() -> Menu {
    let submenu_main = Submenu::new(
        "Test-ImageTiny".to_string(),
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );

    Menu::new().add_submenu(submenu_main)
}

// 托盘菜单
pub fn system_menu() -> SystemTray {
    // 子菜单
    let submenu = SystemTraySubmenu::new(
        // 子菜单名称
        "文件",
        // 子菜单项（新增）
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("new_file".to_string(), "新建文件"))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "编辑文件")),
    );

    let tray_menu = SystemTrayMenu::new()
        .add_submenu(submenu)
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(CustomMenuItem::new("show".to_string(), "显示主窗口")) // 显示应用窗口
        .add_item(CustomMenuItem::new("hide".to_string(), "隐藏")) // 隐藏应用窗口
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(CustomMenuItem::new("quit".to_string(), "退出")); // 退出

    // 设置在右键单击系统托盘时显示菜单
    SystemTray::new().with_menu(tray_menu)
}

// 系统菜单事件
pub fn system_menu_handler(app: &AppHandle, event: SystemTrayEvent) {

    // 获取应用窗口
    let window = match app.get_window("main") {
        Some(w) => w,
        _ => {
            tauri::window::WindowBuilder::new(
                app,
                "main".to_string(),
                tauri::WindowUrl::App("index.html".into()),
                )
                .title("ImageView")
                // .center()
                .fullscreen(false)
                .decorations(true)
                .visible(false)
                .build()
                .unwrap()
        },
    };

    let parent_window = Some(&window);
 
    // 匹配点击事件
    match event {
        // 左键点击
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
     
        }
        // 右键点击
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        // 双击，macOS / Linux 不支持
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        // 根据菜单 id 进行事件匹配
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "edit_file" => {
                tauri::api::dialog::message(parent_window, "Eidt File", "TODO");
            }
            "new_file" => {
                message(parent_window, "New File", "TODO");
            }
            "quit" => {
                std::process::exit(0);
            }
            "show" => {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            "hide" => {
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {
            println!("other event")
        }
    }
}
