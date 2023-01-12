#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod core;
mod state;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    WindowEvent,
};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
pub struct ProxyManagerSender(Mutex<Sender<u32>>);
pub struct ProxyManagerReceiver(Mutex<Receiver<u32>>);
// mod menu;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_conf(handle: tauri::AppHandle) -> String {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("resources/config/config.default.json")
        .expect("failed to resolve resource");

    let file = std::fs::File::open(&resource_path).unwrap();
    let foo: serde_json::Value = serde_json::from_reader(file).unwrap();

    foo.get("foo").unwrap().to_string()
}

fn main() {
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
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
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
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let window_ = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::Resized(size) = event {
                    if size.width == 0 && size.height == 0 {
                        window_.hide().unwrap();
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            read_conf,
            commands::proxy::start_proxy,
            commands::proxy::start_proxy2,
            commands::proxy::stop_proxy,
            commands::dns::pull_doh_servers_and_set_resolvers,
            commands::dns::query_dns_and_set_host,
            commands::dns::query_dns_of_single_domain,
            commands::dns::get_all_hosts,
            commands::dns::get_all_doh_servers,
            commands::dns::add_doh_servers,
            commands::dns::add_hosts,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api: _api, .. } => {
                // api.prevent_exit();
                app_handle.trigger_global("stop-proxy", None);
                // std::process::exit(0);
            }
            _ => {}
        });
}
