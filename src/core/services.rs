//! Service layer for core business logic
//!
//! This module provides high-level business logic services that coordinate
//! between file system operations and database operations.

use std::path::{Path, PathBuf};
use std::fs;

use crate::core::models::*;
use crate::core::{Error, Result};
use crate::storage::{DatabaseManager, NoteDao, TagDao, FolderDao, LinkDao, BlockDao};
use crate::storage::{NoteFolderDao, NoteTagDao};

/// Service context that holds database and file system paths
pub struct ServiceContext {
    db: DatabaseManager,
    data_dir: PathBuf,
}

impl ServiceContext {
    /// Create a new service context
    pub fn new<P: AsRef<Path>>(db_path: P, data_dir: P) -> Result<Self> {
        let db = DatabaseManager::new(db_path)?;
        let data_dir = data_dir.as_ref().to_path_buf();
        
        // Ensure data directory exists
        fs::create_dir_all(&data_dir)?;
        fs::create_dir_all(data_dir.join("notes"))?;
        fs::create_dir_all(data_dir.join("attachments"))?;
        
        Ok(Self { db, data_dir })
    }

    /// Get database connection
    pub fn conn(&self) -> &rusqlite::Connection {
        self.db.conn()
    }

    /// Get data directory path
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
}

/// Note service for managing notes
pub struct NoteService;

impl NoteService {
    /// Create a new note with content
    pub fn create(ctx: &ServiceContext, title: String, content: String) -> Result<Note> {
        // Generate note ID
        let uuid = uuid::Uuid::new_v4();
        let note_id = format!("note-{}", uuid);
        
        // Generate file path (simplified: just use UUID for now, slug can be added later)
        let file_name = format!("{}-{}.md", uuid, Self::slugify(&title));
        let content_path = format!("notes/{}", file_name);
        let full_path = ctx.data_dir().join(&content_path);
        
        // Create note model
        let mut note = Note::new(note_id.clone(), title, content_path.clone());
        
        // Write content to file
        fs::write(&full_path, content.as_bytes())?;
        
        // Calculate word count
        note.update_word_count(Self::count_words(&content));
        
        // Save to database
        NoteDao::create(ctx.conn(), &note)?;
        
        Ok(note)
    }

    /// Get a note by ID (including content from file)
    pub fn get_by_id(ctx: &ServiceContext, id: &str, include_deleted: bool) -> Result<Option<NoteWithContent>> {
        let note = NoteDao::get_by_id(ctx.conn(), id, include_deleted)?;
        
        match note {
            Some(note) => {
                // Read content from file
                let content_path = ctx.data_dir().join(&note.content_path);
                let content = if content_path.exists() {
                    fs::read_to_string(&content_path)?
                } else {
                    String::new()
                };
                
                Ok(Some(NoteWithContent {
                    note,
                    content,
                }))
            }
            None => Ok(None),
        }
    }

    /// Update note title and/or content
    pub fn update(ctx: &ServiceContext, id: &str, title: Option<String>, content: Option<String>) -> Result<()> {
        let mut note = NoteDao::get_by_id(ctx.conn(), id, false)?
            .ok_or_else(|| Error::NotFound(format!("Note not found: {}", id)))?;
        
        // Update title if provided
        if let Some(new_title) = title {
            note.update_title(new_title);
        }
        
        // Update content if provided
        if let Some(new_content) = content {
            let content_path = ctx.data_dir().join(&note.content_path);
            fs::write(&content_path, new_content.as_bytes())?;
            note.update_word_count(Self::count_words(&new_content));
        }
        
        // Update in database
        NoteDao::update(ctx.conn(), &note)?;
        
        Ok(())
    }

    /// Update note content only
    pub fn update_content(ctx: &ServiceContext, id: &str, content: String) -> Result<()> {
        Self::update(ctx, id, None, Some(content))
    }

    /// Update note title only
    pub fn update_title(ctx: &ServiceContext, id: &str, title: String) -> Result<()> {
        Self::update(ctx, id, Some(title), None)
    }

    /// Soft delete a note
    pub fn delete(ctx: &ServiceContext, id: &str) -> Result<()> {
        NoteDao::soft_delete(ctx.conn(), id)?;
        Ok(())
    }

    /// Restore a soft-deleted note
    pub fn restore(ctx: &ServiceContext, id: &str) -> Result<()> {
        NoteDao::restore(ctx.conn(), id)?;
        Ok(())
    }

    /// List all notes
    pub fn list(ctx: &ServiceContext, include_deleted: bool) -> Result<Vec<Note>> {
        NoteDao::list(ctx.conn(), include_deleted)
    }

    /// Search notes by title
    pub fn search_by_title(ctx: &ServiceContext, query: &str, include_deleted: bool) -> Result<Vec<Note>> {
        NoteDao::search_by_title(ctx.conn(), query, include_deleted)
    }

    /// Get notes in a folder
    pub fn get_by_folder(ctx: &ServiceContext, folder_id: &str, include_deleted: bool) -> Result<Vec<Note>> {
        NoteDao::get_by_folder(ctx.conn(), folder_id, include_deleted)
    }

    /// Add note to folder
    pub fn add_to_folder(ctx: &ServiceContext, note_id: &str, folder_id: &str, is_primary: bool, position: i64) -> Result<()> {
        NoteFolderDao::add(ctx.conn(), note_id, folder_id, is_primary, position)?;
        Ok(())
    }

    /// Remove note from folder
    pub fn remove_from_folder(ctx: &ServiceContext, note_id: &str, folder_id: &str) -> Result<()> {
        NoteFolderDao::remove(ctx.conn(), note_id, folder_id)?;
        Ok(())
    }

    /// Add tag to note
    pub fn add_tag(ctx: &ServiceContext, note_id: &str, tag_id: &str) -> Result<()> {
        NoteTagDao::add(ctx.conn(), note_id, tag_id)?;
        Ok(())
    }

    /// Remove tag from note
    pub fn remove_tag(ctx: &ServiceContext, note_id: &str, tag_id: &str) -> Result<()> {
        NoteTagDao::remove(ctx.conn(), note_id, tag_id)?;
        Ok(())
    }

    /// Get all tags for a note
    pub fn get_tags(ctx: &ServiceContext, note_id: &str) -> Result<Vec<Tag>> {
        let tag_ids = NoteTagDao::get_tags_for_note(ctx.conn(), note_id)?;
        let mut tags = Vec::new();
        
        for tag_id in tag_ids {
            if let Some(tag) = TagDao::get_by_id(ctx.conn(), &tag_id)? {
                tags.push(tag);
            }
        }
        
        Ok(tags)
    }

    /// Helper: Count words in content
    fn count_words(content: &str) -> i64 {
        content.split_whitespace().count() as i64
    }

    /// Helper: Slugify title (simplified version)
    fn slugify(title: &str) -> String {
        // Simplified slugify: convert to lowercase, replace spaces with hyphens
        // For full implementation, consider using a crate like `slug` or `deunicode`
        title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
            .collect::<String>()
            .chars()
            .fold(String::new(), |mut acc, c| {
                if !acc.ends_with('-') || c != '-' {
                    acc.push(c);
                }
                acc
            })
            .trim_matches('-')
            .to_string()
            .chars()
            .take(50)
            .collect()
    }
}

/// Note with content loaded from file
#[derive(Debug, Clone)]
pub struct NoteWithContent {
    pub note: Note,
    pub content: String,
}

/// Tag service for managing tags
pub struct TagService;

impl TagService {
    /// Create a new tag
    pub fn create(ctx: &ServiceContext, name: String) -> Result<Tag> {
        // Check if tag with same name already exists
        if let Some(_) = TagDao::get_by_name(ctx.conn(), &name)? {
            return Err(Error::InvalidInput(format!("Tag '{}' already exists", name)));
        }
        
        let uuid = uuid::Uuid::new_v4();
        let tag_id = format!("tag-{}", uuid);
        let tag = Tag::new(tag_id.clone(), name);
        
        TagDao::create(ctx.conn(), &tag)?;
        
        Ok(tag)
    }

    /// Get a tag by ID
    pub fn get_by_id(ctx: &ServiceContext, id: &str) -> Result<Option<Tag>> {
        TagDao::get_by_id(ctx.conn(), id)
    }

    /// Get a tag by name
    pub fn get_by_name(ctx: &ServiceContext, name: &str) -> Result<Option<Tag>> {
        TagDao::get_by_name(ctx.conn(), name)
    }

    /// List all tags
    pub fn list(ctx: &ServiceContext) -> Result<Vec<Tag>> {
        TagDao::list(ctx.conn())
    }

    /// Update a tag
    pub fn update(ctx: &ServiceContext, tag: &Tag) -> Result<()> {
        TagDao::update(ctx.conn(), tag)?;
        Ok(())
    }

    /// Delete a tag
    pub fn delete(ctx: &ServiceContext, id: &str) -> Result<()> {
        TagDao::delete(ctx.conn(), id)?;
        Ok(())
    }

    /// Get all notes with a tag
    pub fn get_notes(ctx: &ServiceContext, tag_id: &str) -> Result<Vec<Note>> {
        let note_ids = crate::storage::NoteTagDao::get_notes_with_tag(ctx.conn(), tag_id)?;
        let mut notes = Vec::new();
        
        for note_id in note_ids {
            if let Some(note) = NoteDao::get_by_id(ctx.conn(), &note_id, false)? {
                notes.push(note);
            }
        }
        
        Ok(notes)
    }
}

/// Folder service for managing folders
pub struct FolderService;

impl FolderService {
    /// Create a new folder
    pub fn create(ctx: &ServiceContext, name: String, parent_id: Option<String>) -> Result<Folder> {
        // Validate parent exists if provided
        if let Some(ref pid) = parent_id {
            if FolderDao::get_by_id(ctx.conn(), pid)?.is_none() {
                return Err(Error::NotFound(format!("Parent folder not found: {}", pid)));
            }
        }
        
        let uuid = uuid::Uuid::new_v4();
        let folder_id = format!("folder-{}", uuid);
        
        // Calculate path
        let path = if let Some(ref pid) = parent_id {
            if let Some(parent) = FolderDao::get_by_id(ctx.conn(), pid)? {
                format!("{}/{}", parent.path, name)
            } else {
                format!("/{}", name)
            }
        } else {
            format!("/{}", name)
        };
        
        let folder = Folder::new(folder_id.clone(), name, parent_id, path);
        
        FolderDao::create(ctx.conn(), &folder)?;
        
        Ok(folder)
    }

    /// Get a folder by ID
    pub fn get_by_id(ctx: &ServiceContext, id: &str) -> Result<Option<Folder>> {
        FolderDao::get_by_id(ctx.conn(), id)
    }

    /// Get root folders
    pub fn get_roots(ctx: &ServiceContext) -> Result<Vec<Folder>> {
        FolderDao::get_roots(ctx.conn())
    }

    /// Get child folders
    pub fn get_children(ctx: &ServiceContext, parent_id: &str) -> Result<Vec<Folder>> {
        FolderDao::get_children(ctx.conn(), parent_id)
    }

    /// Update a folder
    pub fn update(ctx: &ServiceContext, folder: &Folder) -> Result<()> {
        FolderDao::update(ctx.conn(), folder)?;
        Ok(())
    }

    /// Delete a folder
    pub fn delete(ctx: &ServiceContext, id: &str) -> Result<()> {
        // Check if folder has children
        let children = FolderDao::get_children(ctx.conn(), id)?;
        if !children.is_empty() {
            return Err(Error::InvalidInput(format!("Cannot delete folder with children: {}", id)));
        }
        
        FolderDao::delete(ctx.conn(), id)?;
        Ok(())
    }

    /// Get all notes in a folder
    pub fn get_notes(ctx: &ServiceContext, folder_id: &str, include_deleted: bool) -> Result<Vec<Note>> {
        NoteDao::get_by_folder(ctx.conn(), folder_id, include_deleted)
    }
}

/// Link service for managing links between notes and blocks
pub struct LinkService;

impl LinkService {
    /// Create a note link
    pub fn create_note_link(
        ctx: &ServiceContext,
        source_note_id: String,
        target_note_id: String,
        link_text: Option<String>,
    ) -> Result<Link> {
        // Validate notes exist
        if NoteDao::get_by_id(ctx.conn(), &source_note_id, false)?.is_none() {
            return Err(Error::NotFound(format!("Source note not found: {}", source_note_id)));
        }
        if NoteDao::get_by_id(ctx.conn(), &target_note_id, false)?.is_none() {
            return Err(Error::NotFound(format!("Target note not found: {}", target_note_id)));
        }
        
        let uuid = uuid::Uuid::new_v4();
        let link_id = format!("link-{}", uuid);
        let link = Link::new_note_link(link_id.clone(), source_note_id, target_note_id, link_text);
        
        LinkDao::create(ctx.conn(), &link)?;
        
        Ok(link)
    }

    /// Create a block reference
    pub fn create_block_reference(
        ctx: &ServiceContext,
        source_block_id: String,
        target_block_id: String,
        source_note_id: String,
    ) -> Result<Link> {
        // Validate blocks exist
        if BlockDao::get_by_id(ctx.conn(), &source_block_id, false)?.is_none() {
            return Err(Error::NotFound(format!("Source block not found: {}", source_block_id)));
        }
        if BlockDao::get_by_id(ctx.conn(), &target_block_id, false)?.is_none() {
            return Err(Error::NotFound(format!("Target block not found: {}", target_block_id)));
        }
        
        let uuid = uuid::Uuid::new_v4();
        let link_id = format!("link-{}", uuid);
        let link = Link::new_block_reference(link_id.clone(), source_block_id, target_block_id, source_note_id);
        
        LinkDao::create(ctx.conn(), &link)?;
        
        Ok(link)
    }

    /// Get a link by ID
    pub fn get_by_id(ctx: &ServiceContext, id: &str) -> Result<Option<Link>> {
        LinkDao::get_by_id(ctx.conn(), id)
    }

    /// Get outgoing links from a note
    pub fn get_outgoing_links(ctx: &ServiceContext, note_id: &str) -> Result<Vec<Link>> {
        LinkDao::get_outgoing_links(ctx.conn(), note_id)
    }

    /// Get incoming links to a note
    pub fn get_incoming_links(ctx: &ServiceContext, note_id: &str) -> Result<Vec<Link>> {
        LinkDao::get_incoming_links(ctx.conn(), note_id)
    }

    /// Get links from a block
    pub fn get_links_from_block(ctx: &ServiceContext, block_id: &str) -> Result<Vec<Link>> {
        LinkDao::get_links_from_block(ctx.conn(), block_id)
    }

    /// Get links to a block
    pub fn get_links_to_block(ctx: &ServiceContext, block_id: &str) -> Result<Vec<Link>> {
        LinkDao::get_links_to_block(ctx.conn(), block_id)
    }

    /// Delete a link
    pub fn delete(ctx: &ServiceContext, id: &str) -> Result<()> {
        LinkDao::delete(ctx.conn(), id)?;
        Ok(())
    }
}

/// Search service for full-text search
pub struct SearchService;

impl SearchService {
    /// Search notes by full-text (using FTS5)
    pub fn search_notes(ctx: &ServiceContext, query: &str, include_deleted: bool) -> Result<Vec<Note>> {
        let conn = ctx.conn();
        
        // Build FTS query - FTS5 uses rowid to join with source table
        let mut sql = r#"
            SELECT DISTINCT n.id, n.title, n.content_path, n.created_at, n.updated_at, n.word_count, n.is_deleted, n.deleted_at
            FROM notes_fts fts
            INNER JOIN notes n ON n.rowid = fts.rowid
            WHERE notes_fts MATCH ?1
        "#
        .to_string();
        
        if !include_deleted {
            sql.push_str(" AND n.is_deleted = 0");
        }
        
        sql.push_str(" ORDER BY n.updated_at DESC");
        
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![query], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content_path: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                word_count: row.get(5)?,
                is_deleted: row.get::<_, i32>(6)? != 0,
                deleted_at: row.get(7)?,
            })
        })?;
        
        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        
        Ok(notes)
    }

    /// Search blocks by full-text (using FTS5)
    pub fn search_blocks(ctx: &ServiceContext, query: &str, include_deleted: bool) -> Result<Vec<Block>> {
        let conn = ctx.conn();
        
        // Build FTS query - FTS5 uses rowid to join with source table
        let mut sql = r#"
            SELECT DISTINCT b.id, b.note_id, b.block_type, b.content, b.position, b.created_at, b.updated_at, b.is_deleted, b.deleted_at
            FROM blocks_fts fts
            INNER JOIN blocks b ON b.rowid = fts.rowid
            WHERE blocks_fts MATCH ?1
        "#
        .to_string();
        
        if !include_deleted {
            sql.push_str(" AND b.is_deleted = 0");
        }
        
        sql.push_str(" ORDER BY b.position");
        
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![query], |row| {
            Ok(Block {
                id: row.get(0)?,
                note_id: row.get(1)?,
                block_type: row.get(2)?,
                content: row.get(3)?,
                position: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
                is_deleted: row.get::<_, i32>(7)? != 0,
                deleted_at: row.get(8)?,
            })
        })?;
        
        let mut blocks = Vec::new();
        for row in rows {
            blocks.push(row?);
        }
        
        Ok(blocks)
    }
}

/// Block service for managing blocks
pub struct BlockService;

impl BlockService {
    /// Create a new block
    pub fn create(
        ctx: &ServiceContext,
        note_id: String,
        block_type: String,
        content: String,
        position: i64,
    ) -> Result<Block> {
        // Validate note exists
        if NoteDao::get_by_id(ctx.conn(), &note_id, false)?.is_none() {
            return Err(Error::NotFound(format!("Note not found: {}", note_id)));
        }
        
        let uuid = uuid::Uuid::new_v4();
        let block_id = format!("block-{}", uuid);
        let block = Block::new(block_id.clone(), note_id, block_type, content, position);
        
        BlockDao::create(ctx.conn(), &block)?;
        
        Ok(block)
    }

    /// Get a block by ID
    pub fn get_by_id(ctx: &ServiceContext, id: &str, include_deleted: bool) -> Result<Option<Block>> {
        BlockDao::get_by_id(ctx.conn(), id, include_deleted)
    }

    /// Get all blocks for a note
    pub fn get_by_note(ctx: &ServiceContext, note_id: &str, include_deleted: bool) -> Result<Vec<Block>> {
        BlockDao::get_by_note(ctx.conn(), note_id, include_deleted)
    }

    /// Update a block
    pub fn update(ctx: &ServiceContext, block: &Block) -> Result<()> {
        BlockDao::update(ctx.conn(), block)?;
        Ok(())
    }

    /// Update block content
    pub fn update_content(ctx: &ServiceContext, id: &str, content: String) -> Result<()> {
        let mut block = BlockDao::get_by_id(ctx.conn(), id, false)?
            .ok_or_else(|| Error::NotFound(format!("Block not found: {}", id)))?;
        
        block.update_content(content);
        BlockDao::update(ctx.conn(), &block)?;
        
        Ok(())
    }

    /// Update block position
    pub fn update_position(ctx: &ServiceContext, id: &str, position: i64) -> Result<()> {
        let mut block = BlockDao::get_by_id(ctx.conn(), id, false)?
            .ok_or_else(|| Error::NotFound(format!("Block not found: {}", id)))?;
        
        block.position = position;
        block.updated_at = chrono::Utc::now().timestamp();
        BlockDao::update(ctx.conn(), &block)?;
        
        Ok(())
    }

    /// Soft delete a block
    pub fn delete(ctx: &ServiceContext, id: &str) -> Result<()> {
        BlockDao::soft_delete(ctx.conn(), id)?;
        Ok(())
    }

    /// Restore a soft-deleted block
    pub fn restore(ctx: &ServiceContext, id: &str) -> Result<()> {
        BlockDao::restore(ctx.conn(), id)?;
        Ok(())
    }

    /// Get blocks that reference a block
    pub fn get_referencing_blocks(ctx: &ServiceContext, block_id: &str) -> Result<Vec<Block>> {
        use crate::storage::BlockReferenceDao;
        
        let referencing_ids = BlockReferenceDao::get_referencing_blocks(ctx.conn(), block_id)?;
        let mut blocks = Vec::new();
        
        for id in referencing_ids {
            if let Some(block) = BlockDao::get_by_id(ctx.conn(), &id, false)? {
                blocks.push(block);
            }
        }
        
        Ok(blocks)
    }

    /// Get blocks referenced by a block
    pub fn get_referenced_blocks(ctx: &ServiceContext, block_id: &str) -> Result<Vec<Block>> {
        use crate::storage::BlockReferenceDao;
        
        let referenced_ids = BlockReferenceDao::get_referenced_blocks(ctx.conn(), block_id)?;
        let mut blocks = Vec::new();
        
        for id in referenced_ids {
            if let Some(block) = BlockDao::get_by_id(ctx.conn(), &id, false)? {
                blocks.push(block);
            }
        }
        
        Ok(blocks)
    }

    /// Create a block reference
    pub fn create_reference(
        ctx: &ServiceContext,
        source_block_id: String,
        target_block_id: String,
    ) -> Result<()> {
        use crate::storage::BlockReferenceDao;
        
        // Validate blocks exist
        if BlockDao::get_by_id(ctx.conn(), &source_block_id, false)?.is_none() {
            return Err(Error::NotFound(format!("Source block not found: {}", source_block_id)));
        }
        if BlockDao::get_by_id(ctx.conn(), &target_block_id, false)?.is_none() {
            return Err(Error::NotFound(format!("Target block not found: {}", target_block_id)));
        }
        
        let uuid = uuid::Uuid::new_v4();
        let ref_id = format!("ref-{}", uuid);
        
        BlockReferenceDao::create(ctx.conn(), &ref_id, &source_block_id, &target_block_id)?;
        
        Ok(())
    }

    /// Delete a block reference
    pub fn delete_reference(
        ctx: &ServiceContext,
        source_block_id: String,
        target_block_id: String,
    ) -> Result<()> {
        use crate::storage::BlockReferenceDao;
        
        BlockReferenceDao::delete(ctx.conn(), &source_block_id, &target_block_id)?;
        Ok(())
    }
}

use rusqlite::params;
