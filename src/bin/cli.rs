//! CLI tool for testing Synapse Knowledge Manager without GUI
//!
//! This tool allows testing backend functionality in headless environments.

use synapse_knowledge_manager::core::{ServiceContext, NoteService, TagService, FolderService};
use synapse_knowledge_manager::core::Result;
use std::env;
use std::path::PathBuf;

fn print_usage() {
    println!("Synapse Knowledge Manager CLI");
    println!();
    println!("Usage: synapse-cli <command> [args...]");
    println!();
    println!("Commands:");
    println!("  create-note <title> <content>    Create a new note");
    println!("  get-note <id>                    Get a note by ID");
    println!("  list-notes                       List all notes");
    println!("  search <query>                  Search notes");
    println!("  create-tag <name>                Create a tag");
    println!("  list-tags                        List all tags");
    println!("  create-folder <name> [parent]    Create a folder");
    println!("  list-folders                     List all folders");
    println!();
    println!("Environment variables:");
    println!("  SYNAPSE_DB_PATH                  Database path (default: ./data/synapse.db)");
    println!("  SYNAPSE_DATA_DIR                 Data directory (default: ./data)");
}

fn get_ctx() -> Result<ServiceContext> {
    let db_path = env::var("SYNAPSE_DB_PATH")
        .unwrap_or_else(|_| "./data/synapse.db".to_string());
    let data_dir = env::var("SYNAPSE_DATA_DIR")
        .unwrap_or_else(|_| "./data".to_string());
    
    // Ensure data directory exists
    std::fs::create_dir_all(&data_dir)?;
    std::fs::create_dir_all(PathBuf::from(&data_dir).join("notes"))?;
    
    ServiceContext::new(&db_path, &data_dir)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let ctx = match get_ctx() {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error: Failed to create service context: {}", e);
            std::process::exit(1);
        }
    };
    
    match args[1].as_str() {
        "create-note" => {
            if args.len() < 4 {
                eprintln!("Error: create-note requires <title> <content>");
                std::process::exit(1);
            }
            match NoteService::create(&ctx, args[2].clone(), args[3].clone()) {
                Ok(note) => {
                    println!("Created note: {}", note.id);
                    println!("Title: {}", note.title);
                    println!("Path: {}", note.content_path);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "get-note" => {
            if args.len() < 3 {
                eprintln!("Error: get-note requires <id>");
                std::process::exit(1);
            }
            match NoteService::get_by_id(&ctx, &args[2], false) {
                Ok(Some(note_with_content)) => {
                    println!("Note ID: {}", note_with_content.note.id);
                    println!("Title: {}", note_with_content.note.title);
                    println!("Content:\n{}", note_with_content.content);
                }
                Ok(None) => {
                    eprintln!("Note not found: {}", args[2]);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "list-notes" => {
            match NoteService::list(&ctx, false) {
                Ok(notes) => {
                    println!("Found {} notes:", notes.len());
                    for note in notes {
                        println!("  - {}: {} (updated: {})", 
                            note.id, note.title, note.updated_at);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "search" => {
            if args.len() < 3 {
                eprintln!("Error: search requires <query>");
                std::process::exit(1);
            }
            match NoteService::search_by_title(&ctx, &args[2], false) {
                Ok(notes) => {
                    println!("Found {} notes matching '{}':", notes.len(), args[2]);
                    for note in notes {
                        println!("  - {}: {}", note.id, note.title);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "create-tag" => {
            if args.len() < 3 {
                eprintln!("Error: create-tag requires <name>");
                std::process::exit(1);
            }
            match TagService::create(&ctx, args[2].clone()) {
                Ok(tag) => {
                    println!("Created tag: {} ({})", tag.name, tag.id);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "list-tags" => {
            match TagService::list(&ctx) {
                Ok(tags) => {
                    println!("Found {} tags:", tags.len());
                    for tag in tags {
                        println!("  - {}: {}", tag.id, tag.name);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "create-folder" => {
            if args.len() < 3 {
                eprintln!("Error: create-folder requires <name>");
                std::process::exit(1);
            }
            let parent_id = if args.len() > 3 {
                Some(args[3].clone())
            } else {
                None
            };
            match FolderService::create(&ctx, args[2].clone(), parent_id) {
                Ok(folder) => {
                    println!("Created folder: {} ({})", folder.name, folder.id);
                    println!("Path: {}", folder.path);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "list-folders" => {
            match FolderService::get_roots(&ctx) {
                Ok(folders) => {
                    println!("Found {} root folders:", folders.len());
                    for folder in folders {
                        println!("  - {}: {} ({})", folder.id, folder.name, folder.path);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }
}
