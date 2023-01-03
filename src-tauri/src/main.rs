#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read_conf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
