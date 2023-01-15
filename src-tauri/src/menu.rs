use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use crate::commands::open_folder;

pub fn generate_menu() -> Menu {
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let open_dns_page = CustomMenuItem::new("open_dns_page".to_string(), "打开DNS页面");
    let open_data_folder = CustomMenuItem::new("open_data_folder".to_string(), "打开配置文件夹");
    let submenu = Submenu::new(
        "操作",
        Menu::new()
            .add_item(open_dns_page)
            .add_item(open_data_folder),
    );
    let menu = Menu::new()
        // .add_item(CustomMenuItem::new("config", "config"))
        .add_submenu(submenu);
    menu
}

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "open_data_folder" => {
            let resource_path = event
                .window()
                .app_handle()
                .path_resolver()
                .resolve_resource("data")
                .expect("failed to resolve resource");
            open_folder(resource_path.as_path().to_str().unwrap().to_string());
        }
        "open_dns_page" => {
            std::thread::spawn(move || {
                let handle = event.window().app_handle();
                let _ = tauri::WindowBuilder::new(
                    &handle,
                    "DNS查询",
                    tauri::WindowUrl::App("dns.html".into()),
                )
                .decorations(false)
                .center()
                .inner_size(300.0, 500.0)
                .min_inner_size(300.0, 500.0)
                .focused(true)
                .build();
            });
        }
        // "quit" => {
        //     std::process::exit(0);
        // }
        // "close" => {
        //     event.window().close().unwrap();
        // }
        _ => {}
    }
}
