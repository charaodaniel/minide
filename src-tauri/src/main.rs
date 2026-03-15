#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

// Função para listar o conteúdo de um diretório
#[tauri::command]
fn list_dir(path: String) -> Result<Vec<String>, String> {
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut result = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    result.push(entry.file_name().to_string_lossy().to_string());
                }
            }
            Ok(result)
        }
        Err(e) => Err(e.to_string()),
    }
}

// Função para ler o conteúdo de um arquivo
#[tauri::command]
fn read_file_content(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_pty::init())
        .invoke_handler(tauri::generate_handler![
            list_dir,
            read_file_content,
        ])
        .run(tauri::generate_context!())    
        .expect("error while running tauri application");
}
