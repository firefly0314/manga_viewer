// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod fs_ops;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            fs_ops::read_file,
            fs_ops::write_file,
            fs_ops::list_directory,
            fs_ops::search_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
