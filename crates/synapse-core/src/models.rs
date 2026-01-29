//! Data models for the core module

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Unique identifier for a note
pub type NoteId = String;

/// Unique identifier for a block
pub type BlockId = String;

/// Unique identifier for a folder
pub type FolderId = String;

/// Unique identifier for a tag
pub type TagId = String;

/// Unique identifier for a link
pub type LinkId = String;

/// Unique identifier for a database
pub type DatabaseId = String;

/// Unique identifier for an attachment
pub type AttachmentId = String;

/// A note in the knowledge base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content_path: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub word_count: i64,
    pub is_deleted: bool,
    pub deleted_at: Option<i64>,
}

impl Note {
    pub fn new(id: NoteId, title: String, content_path: String) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id,
            title,
            content_path,
            created_at: now,
            updated_at: now,
            word_count: 0,
            is_deleted: false,
            deleted_at: None,
        }
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now().timestamp();
    }

    pub fn update_word_count(&mut self, word_count: i64) {
        self.word_count = word_count;
        self.updated_at = Utc::now().timestamp();
    }

    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.deleted_at = Some(Utc::now().timestamp());
    }

    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.deleted_at = None;
    }
}

/// Note with content loaded from file
#[derive(Debug, Clone, Serialize)]
pub struct NoteWithContent {
    pub note: Note,
    pub content: String,
}

/// A block in a note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: BlockId,
    pub note_id: NoteId,
    pub block_type: String,
    pub content: String,
    pub position: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_deleted: bool,
    pub deleted_at: Option<i64>,
}

impl Block {
    pub fn new(id: BlockId, note_id: NoteId, block_type: String, content: String, position: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id,
            note_id,
            block_type,
            content,
            position,
            created_at: now,
            updated_at: now,
            is_deleted: false,
            deleted_at: None,
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now().timestamp();
    }

    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.deleted_at = Some(Utc::now().timestamp());
    }

    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.deleted_at = None;
    }
}

/// A folder for organizing notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub parent_id: Option<FolderId>,
    pub path: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub position: i64,
}

impl Folder {
    pub fn new(id: FolderId, name: String, parent_id: Option<FolderId>, path: String) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id,
            name,
            parent_id,
            path,
            created_at: now,
            updated_at: now,
            position: 0,
        }
    }
}

/// A tag for organizing notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: i64,
}

impl Tag {
    pub fn new(id: TagId, name: String) -> Self {
        Self {
            id,
            name,
            color: None,
            icon: None,
            created_at: Utc::now().timestamp(),
        }
    }
}

/// A link between notes or blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: LinkId,
    pub source_note_id: NoteId,
    pub target_note_id: Option<NoteId>,
    pub source_block_id: Option<BlockId>,
    pub target_block_id: Option<BlockId>,
    pub link_type: String,
    pub link_text: Option<String>,
    pub created_at: i64,
}

impl Link {
    pub fn new_note_link(
        id: LinkId,
        source_note_id: NoteId,
        target_note_id: NoteId,
        link_text: Option<String>,
    ) -> Self {
        Self {
            id,
            source_note_id,
            target_note_id: Some(target_note_id),
            source_block_id: None,
            target_block_id: None,
            link_type: "note_link".to_string(),
            link_text,
            created_at: Utc::now().timestamp(),
        }
    }

    pub fn new_block_reference(
        id: LinkId,
        source_block_id: BlockId,
        target_block_id: BlockId,
        source_note_id: NoteId,
    ) -> Self {
        Self {
            id,
            source_note_id,
            target_note_id: None,
            source_block_id: Some(source_block_id),
            target_block_id: Some(target_block_id),
            link_type: "block_reference".to_string(),
            link_text: None,
            created_at: Utc::now().timestamp(),
        }
    }
}

/// An attachment (image, PDF, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: AttachmentId,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub hash: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Attachment {
    pub fn new(
        id: AttachmentId,
        file_name: String,
        file_path: String,
        file_type: String,
        mime_type: String,
        file_size: i64,
        hash: String,
    ) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id,
            file_name,
            file_path,
            file_type,
            mime_type,
            file_size,
            width: None,
            height: None,
            hash,
            created_at: now,
            updated_at: now,
        }
    }
}
