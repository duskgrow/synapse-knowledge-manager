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
    /// Unique identifier (format: note-{uuid})
    pub id: NoteId,
    /// Note title
    pub title: String,
    /// Markdown file path
    pub content_path: String,
    /// Creation timestamp (Unix timestamp)
    pub created_at: i64,
    /// Last modification timestamp (Unix timestamp)
    pub updated_at: i64,
    /// Word count
    pub word_count: i64,
    /// Whether the note is deleted (soft delete)
    pub is_deleted: bool,
    /// Deletion timestamp (Unix timestamp, None if not deleted)
    pub deleted_at: Option<i64>,
}

impl Note {
    /// Create a new note
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

    /// Update the note title
    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now().timestamp();
    }

    /// Update the word count
    pub fn update_word_count(&mut self, word_count: i64) {
        self.word_count = word_count;
        self.updated_at = Utc::now().timestamp();
    }

    /// Soft delete the note
    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.deleted_at = Some(Utc::now().timestamp());
    }

    /// Restore the note
    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.deleted_at = None;
    }
}

/// A block in a note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Unique identifier (format: block-{uuid})
    pub id: BlockId,
    /// Note ID this block belongs to
    pub note_id: NoteId,
    /// Block type (paragraph, heading, list, code, etc.)
    pub block_type: String,
    /// Block content (Markdown fragment)
    pub content: String,
    /// Position in the note (for ordering)
    pub position: i64,
    /// Creation timestamp (Unix timestamp)
    pub created_at: i64,
    /// Last modification timestamp (Unix timestamp)
    pub updated_at: i64,
    /// Whether the block is deleted (soft delete)
    pub is_deleted: bool,
    /// Deletion timestamp (Unix timestamp, None if not deleted)
    pub deleted_at: Option<i64>,
}

impl Block {
    /// Create a new block
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

    /// Update the block content
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now().timestamp();
    }

    /// Soft delete the block
    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.deleted_at = Some(Utc::now().timestamp());
    }

    /// Restore the block
    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.deleted_at = None;
    }
}

/// A folder for organizing notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// Unique identifier (format: folder-{uuid})
    pub id: FolderId,
    /// Folder name
    pub name: String,
    /// Parent folder ID (None for root folder)
    pub parent_id: Option<FolderId>,
    /// Cached path (e.g., /学习/编程)
    pub path: String,
    /// Creation timestamp (Unix timestamp)
    pub created_at: i64,
    /// Last modification timestamp (Unix timestamp)
    pub updated_at: i64,
    /// Position among siblings
    pub position: i64,
}

impl Folder {
    /// Create a new folder
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
    /// Unique identifier (format: tag-{uuid})
    pub id: TagId,
    /// Tag name (unique)
    pub name: String,
    /// Tag color (hex code)
    pub color: Option<String>,
    /// Tag icon (optional)
    pub icon: Option<String>,
    /// Creation timestamp (Unix timestamp)
    pub created_at: i64,
}

impl Tag {
    /// Create a new tag
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
    /// Unique identifier
    pub id: LinkId,
    /// Source note ID
    pub source_note_id: NoteId,
    /// Target note ID (for note links)
    pub target_note_id: Option<NoteId>,
    /// Source block ID (for block references)
    pub source_block_id: Option<BlockId>,
    /// Target block ID (for block references)
    pub target_block_id: Option<BlockId>,
    /// Link type: note_link, block_reference, database_relation
    pub link_type: String,
    /// Link text (display text)
    pub link_text: Option<String>,
    /// Creation timestamp (Unix timestamp)
    pub created_at: i64,
}

impl Link {
    /// Create a new note link
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

    /// Create a new block reference
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
    /// Unique identifier (format: attachment-{uuid})
    pub id: AttachmentId,
    /// Original file name
    pub file_name: String,
    /// File relative path (e.g., attachments/{uuid}.{ext})
    pub file_path: String,
    /// File type: image, document, media, other
    pub file_type: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub file_size: i64,
    /// Image width (only for images)
    pub width: Option<i32>,
    /// Image height (only for images)
    pub height: Option<i32>,
    /// File SHA-256 hash (for deduplication)
    pub hash: String,
    /// Creation timestamp (Unix timestamp)
    pub created_at: i64,
    /// Last modification timestamp (Unix timestamp)
    pub updated_at: i64,
}

impl Attachment {
    /// Create a new attachment
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
