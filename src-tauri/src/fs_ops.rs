use std::path::PathBuf;
use tokio::fs;

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_file(path: String, contents: String) -> Result<(), String> {
    fs::write(&path, contents).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_directory(directory: String) -> Result<Vec<String>, String> {
    let mut entries = fs::read_dir(&directory).await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        result.push(entry.path().to_string_lossy().to_string());
    }
    Ok(result)
}

#[tauri::command]
pub async fn search_files(directory: String, pattern: String) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let mut dirs = vec![PathBuf::from(directory)];

    while let Some(dir) = dirs.pop() {
        let mut entries = match fs::read_dir(&dir).await {
            Ok(e) => e,
            Err(_) => continue,
        };
        while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains(&pattern) {
                    results.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(results)
}
