#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod core;
mod menu;
mod state;
mod system_tray;
use tauri::{Manager, WindowEvent};
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
    println!("当前版本: {}", env!("CARGO_PKG_VERSION"));
    tauri::Builder::default()
        .system_tray(system_tray::generate_system_tray())
        .on_system_tray_event(system_tray::event_handler)
        .menu(menu::generate_menu())
        .on_menu_event(menu::event_handler)
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let window_ = window.clone();
            let _ = window.set_title(&format!("MasterLee {}", env!("CARGO_PKG_VERSION")));
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
            commands::show_in_folder,
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
