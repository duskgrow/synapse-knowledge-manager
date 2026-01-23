// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use synapse_knowledge_manager::core::{ServiceContext, NoteService};
use synapse_knowledge_manager::core::Result;

// Learn more about Tauri commands at https://v2.tauri.app/guide/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Tauri command to create a note
#[tauri::command]
async fn create_note(
    title: String,
    content: String,
    db_path: String,
    data_dir: String,
) -> Result<String, String> {
    let ctx = ServiceContext::new(&db_path, &data_dir)
        .map_err(|e| format!("Failed to create service context: {}", e))?;
    
    let note = NoteService::create(&ctx, title, content)
        .map_err(|e| format!("Failed to create note: {}", e))?;
    
    Ok(serde_json::to_string(&note).map_err(|e| format!("Serialization error: {}", e))?)
}

// Tauri command to get a note
#[tauri::command]
async fn get_note(
    id: String,
    db_path: String,
    data_dir: String,
) -> Result<String, String> {
    let ctx = ServiceContext::new(&db_path, &data_dir)
        .map_err(|e| format!("Failed to create service context: {}", e))?;
    
    let note_with_content = NoteService::get_by_id(&ctx, &id, false)
        .map_err(|e| format!("Failed to get note: {}", e))?;
    
    match note_with_content {
        Some(note) => Ok(serde_json::to_string(&note).map_err(|e| format!("Serialization error: {}", e))?),
        None => Err("Note not found".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, create_note, get_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
