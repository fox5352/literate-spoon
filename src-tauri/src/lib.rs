use crate::host::{inject_blacklist, parse_host_file};

mod host;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const BLOCKLIST_URL: &str =
    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn-only/hosts";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            let content = match reqwest::blocking::get(BLOCKLIST_URL) {
                Ok(response) => match response.text() {
                    Ok(text) => text,
                    Err(e) => panic!("Failed to get response text: {}", e),
                },
                Err(e) => panic!("Failed to fetch BLOCKLIST_URL: {}", e),
            };

            let block_list = parse_host_file(&content)?;

            inject_blacklist(&block_list)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
