//! Data models for the core module

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for a note
pub type NoteId = String;

/// Unique identifier for a tag
pub type TagId = String;

/// A note in the knowledge base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// Unique identifier
    pub id: NoteId,
    /// Note title
    pub title: String,
    /// Markdown content
    pub content: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Associated tags
    pub tags: Vec<TagId>,
    /// Parent note ID (for hierarchical organization)
    pub parent_id: Option<NoteId>,
}

impl Note {
    /// Create a new note
    pub fn new(id: NoteId, title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            content,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            parent_id: None,
        }
    }

    /// Update the note content
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now();
    }

    /// Update the note title
    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }
}

/// A tag for organizing notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Unique identifier
    pub id: TagId,
    /// Tag name
    pub name: String,
    /// Tag color (hex code)
    pub color: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Tag {
    /// Create a new tag
    pub fn new(id: TagId, name: String) -> Self {
        Self {
            id,
            name,
            color: None,
            created_at: Utc::now(),
        }
    }
}
