// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use tauri::Manager;
use synapse_knowledge_manager::core::Result;
use synapse_knowledge_manager::core::{Note, NoteService, ServiceContext};

/// App data paths (db + data dir), resolved from app_data_dir at startup
struct AppPaths {
    db_path: PathBuf,
    data_dir: PathBuf,
}

fn service_context(paths: &AppPaths) -> Result<ServiceContext, String> {
    ServiceContext::new(&paths.db_path, &paths.data_dir)
        .map_err(|e| format!("Failed to create service context: {}", e))
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_note(
    title: String,
    content: String,
    state: tauri::State<'_, AppPaths>,
) -> Result<String, String> {
    let ctx = service_context(&state)?;
    let note = NoteService::create(&ctx, title, content)
        .map_err(|e| format!("Failed to create note: {}", e))?;
    Ok(serde_json::to_string(&note).map_err(|e| format!("Serialization error: {}", e))?)
}

#[tauri::command]
async fn get_note(id: String, state: tauri::State<'_, AppPaths>) -> Result<String, String> {
    let ctx = service_context(&state)?;
    let note_with_content = NoteService::get_by_id(&ctx, &id, false)
        .map_err(|e| format!("Failed to get note: {}", e))?;
    match note_with_content {
        Some(n) => Ok(serde_json::to_string(&n).map_err(|e| format!("Serialization error: {}", e))?),
        None => Err("Note not found".to_string()),
    }
}

#[tauri::command]
async fn list_notes(
    include_deleted: bool,
    state: tauri::State<'_, AppPaths>,
) -> Result<String, String> {
    let ctx = service_context(&state)?;
    let notes: Vec<Note> = NoteService::list(&ctx, include_deleted)
        .map_err(|e| format!("Failed to list notes: {}", e))?;
    Ok(serde_json::to_string(&notes).map_err(|e| format!("Serialization error: {}", e))?)
}

#[tauri::command]
async fn update_note(
    id: String,
    title: Option<String>,
    content: Option<String>,
    state: tauri::State<'_, AppPaths>,
) -> Result<(), String> {
    let ctx = service_context(&state)?;
    NoteService::update(&ctx, &id, title, content)
        .map_err(|e| format!("Failed to update note: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn delete_note(id: String, state: tauri::State<'_, AppPaths>) -> Result<(), String> {
    let ctx = service_context(&state)?;
    NoteService::delete(&ctx, &id).map_err(|e| format!("Failed to delete note: {}", e))?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| e.to_string())?;
            std::fs::create_dir_all(&data_dir)?;
            let db_path = data_dir.join("synapse.db");
            app.manage(AppPaths { db_path, data_dir });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            create_note,
            get_note,
            list_notes,
            update_note,
            delete_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
