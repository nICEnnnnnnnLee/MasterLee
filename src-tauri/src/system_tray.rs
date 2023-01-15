use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    AppHandle,
};
pub fn generate_system_tray() -> SystemTray {
    let expand = CustomMenuItem::new("expand".to_string(), "还原窗口");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new()
        .add_item(expand)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    system_tray
}

pub fn event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {..} => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick { .. } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick { .. } => {
            let window = app.get_window("main").unwrap();
            if !window.is_visible().unwrap() {
                window.show().unwrap();
                window.unminimize().unwrap();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "expand" => {
                // let local_window = tauri::WindowBuilder::new(
                //     app,
                //     "local",
                //     tauri::WindowUrl::App("index.html".into())
                //   ).build()?;
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.unminimize().unwrap();
            }
            _ => {}
        },
        _ => {}
    }

}
