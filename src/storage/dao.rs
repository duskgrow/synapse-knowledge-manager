//! Data Access Object (DAO) layer for database operations

use rusqlite::{params, Connection, Row};

use crate::core::models::*;
use crate::core::Error;

/// Note DAO
pub struct NoteDao;

impl NoteDao {
    /// Create a new note
    pub fn create(conn: &Connection, note: &Note) -> Result<(), Error> {
        conn.execute(
            r#"
            INSERT INTO notes (id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                note.id,
                note.title,
                note.content_path,
                note.created_at,
                note.updated_at,
                note.word_count,
                note.is_deleted as i32,
                note.deleted_at
            ],
        )?;
        Ok(())
    }

    /// Get a note by ID
    pub fn get_by_id(conn: &Connection, id: &str, include_deleted: bool) -> Result<Option<Note>, Error> {
        let mut query = "SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at FROM notes WHERE id = ?1".to_string();
        if !include_deleted {
            query.push_str(" AND is_deleted = 0");
        }

        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query_map(params![id], |row| Self::row_to_note(row))?;

        match rows.next() {
            Some(Ok(note)) => Ok(Some(note)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// Update a note
    pub fn update(conn: &Connection, note: &Note) -> Result<(), Error> {
        conn.execute(
            r#"
            UPDATE notes
            SET title = ?2, content_path = ?3, updated_at = ?4, word_count = ?5, is_deleted = ?6, deleted_at = ?7
            WHERE id = ?1
            "#,
            params![
                note.id,
                note.title,
                note.content_path,
                note.updated_at,
                note.word_count,
                note.is_deleted as i32,
                note.deleted_at
            ],
        )?;
        Ok(())
    }

    /// Soft delete a note
    pub fn soft_delete(conn: &Connection, id: &str) -> Result<(), Error> {
        let deleted_at = chrono::Utc::now().timestamp();
        conn.execute(
            "UPDATE notes SET is_deleted = 1, deleted_at = ?2 WHERE id = ?1",
            params![id, deleted_at],
        )?;
        Ok(())
    }

    /// Restore a soft-deleted note
    pub fn restore(conn: &Connection, id: &str) -> Result<(), Error> {
        conn.execute(
            "UPDATE notes SET is_deleted = 0, deleted_at = NULL WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// List all notes (excluding deleted by default)
    pub fn list(conn: &Connection, include_deleted: bool) -> Result<Vec<Note>, Error> {
        let mut query = "SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at FROM notes".to_string();
        if !include_deleted {
            query.push_str(" WHERE is_deleted = 0");
        }
        query.push_str(" ORDER BY updated_at DESC");

        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| Self::row_to_note(row))?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        Ok(notes)
    }

    /// Search notes by title
    pub fn search_by_title(conn: &Connection, query: &str, include_deleted: bool) -> Result<Vec<Note>, Error> {
        let mut sql = "SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at FROM notes WHERE title LIKE ?1".to_string();
        if !include_deleted {
            sql.push_str(" AND is_deleted = 0");
        }
        sql.push_str(" ORDER BY updated_at DESC");

        let search_pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![search_pattern], |row| Self::row_to_note(row))?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        Ok(notes)
    }

    /// Get notes by folder ID
    pub fn get_by_folder(conn: &Connection, folder_id: &str, include_deleted: bool) -> Result<Vec<Note>, Error> {
        let mut query = r#"
            SELECT n.id, n.title, n.content_path, n.created_at, n.updated_at, n.word_count, n.is_deleted, n.deleted_at
            FROM notes n
            INNER JOIN note_folders nf ON n.id = nf.note_id
            WHERE nf.folder_id = ?1
        "#
        .to_string();
        if !include_deleted {
            query.push_str(" AND n.is_deleted = 0");
        }
        query.push_str(" ORDER BY nf.position, n.updated_at DESC");

        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map(params![folder_id], |row| Self::row_to_note(row))?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        Ok(notes)
    }

    fn row_to_note(row: &Row) -> rusqlite::Result<Note> {
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
    }
}

/// Block DAO
pub struct BlockDao;

impl BlockDao {
    /// Create a new block
    pub fn create(conn: &Connection, block: &Block) -> Result<(), Error> {
        conn.execute(
            r#"
            INSERT INTO blocks (id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                block.id,
                block.note_id,
                block.block_type,
                block.content,
                block.position,
                block.created_at,
                block.updated_at,
                block.is_deleted as i32,
                block.deleted_at
            ],
        )?;
        Ok(())
    }

    /// Get a block by ID
    pub fn get_by_id(conn: &Connection, id: &str, include_deleted: bool) -> Result<Option<Block>, Error> {
        let mut query = "SELECT id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at FROM blocks WHERE id = ?1".to_string();
        if !include_deleted {
            query.push_str(" AND is_deleted = 0");
        }

        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query_map(params![id], |row| Self::row_to_block(row))?;

        match rows.next() {
            Some(Ok(block)) => Ok(Some(block)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// Get all blocks for a note
    pub fn get_by_note(conn: &Connection, note_id: &str, include_deleted: bool) -> Result<Vec<Block>, Error> {
        let mut query = "SELECT id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at FROM blocks WHERE note_id = ?1".to_string();
        if !include_deleted {
            query.push_str(" AND is_deleted = 0");
        }
        query.push_str(" ORDER BY position");

        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map(params![note_id], |row| Self::row_to_block(row))?;

        let mut blocks = Vec::new();
        for row in rows {
            blocks.push(row?);
        }
        Ok(blocks)
    }

    /// Update a block
    pub fn update(conn: &Connection, block: &Block) -> Result<(), Error> {
        conn.execute(
            r#"
            UPDATE blocks
            SET block_type = ?2, content = ?3, position = ?4, updated_at = ?5, is_deleted = ?6, deleted_at = ?7
            WHERE id = ?1
            "#,
            params![
                block.id,
                block.block_type,
                block.content,
                block.position,
                block.updated_at,
                block.is_deleted as i32,
                block.deleted_at
            ],
        )?;
        Ok(())
    }

    /// Soft delete a block
    pub fn soft_delete(conn: &Connection, id: &str) -> Result<(), Error> {
        let deleted_at = chrono::Utc::now().timestamp();
        conn.execute(
            "UPDATE blocks SET is_deleted = 1, deleted_at = ?2 WHERE id = ?1",
            params![id, deleted_at],
        )?;
        Ok(())
    }

    /// Restore a soft-deleted block
    pub fn restore(conn: &Connection, id: &str) -> Result<(), Error> {
        conn.execute(
            "UPDATE blocks SET is_deleted = 0, deleted_at = NULL WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    fn row_to_block(row: &Row) -> rusqlite::Result<Block> {
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
    }
}

/// Folder DAO
pub struct FolderDao;

impl FolderDao {
    /// Create a new folder
    pub fn create(conn: &Connection, folder: &Folder) -> Result<(), Error> {
        conn.execute(
            r#"
            INSERT INTO folders (id, name, parent_id, path, created_at, updated_at, position)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                folder.id,
                folder.name,
                folder.parent_id,
                folder.path,
                folder.created_at,
                folder.updated_at,
                folder.position
            ],
        )?;
        Ok(())
    }

    /// Get a folder by ID
    pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Folder>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, path, created_at, updated_at, position FROM folders WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| Self::row_to_folder(row))?;

        match rows.next() {
            Some(Ok(folder)) => Ok(Some(folder)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// Get all root folders (folders without parent)
    pub fn get_roots(conn: &Connection) -> Result<Vec<Folder>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, path, created_at, updated_at, position FROM folders WHERE parent_id IS NULL ORDER BY position"
        )?;
        let rows = stmt.query_map([], |row| Self::row_to_folder(row))?;

        let mut folders = Vec::new();
        for row in rows {
            folders.push(row?);
        }
        Ok(folders)
    }

    /// Get child folders
    pub fn get_children(conn: &Connection, parent_id: &str) -> Result<Vec<Folder>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, path, created_at, updated_at, position FROM folders WHERE parent_id = ?1 ORDER BY position"
        )?;
        let rows = stmt.query_map(params![parent_id], |row| Self::row_to_folder(row))?;

        let mut folders = Vec::new();
        for row in rows {
            folders.push(row?);
        }
        Ok(folders)
    }

    /// Update a folder
    pub fn update(conn: &Connection, folder: &Folder) -> Result<(), Error> {
        conn.execute(
            r#"
            UPDATE folders
            SET name = ?2, parent_id = ?3, path = ?4, updated_at = ?5, position = ?6
            WHERE id = ?1
            "#,
            params![
                folder.id,
                folder.name,
                folder.parent_id,
                folder.path,
                folder.updated_at,
                folder.position
            ],
        )?;
        Ok(())
    }

    /// Delete a folder (cascade delete)
    pub fn delete(conn: &Connection, id: &str) -> Result<(), Error> {
        conn.execute("DELETE FROM folders WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn row_to_folder(row: &Row) -> rusqlite::Result<Folder> {
        Ok(Folder {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
            path: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            position: row.get(6)?,
        })
    }
}

/// Tag DAO
pub struct TagDao;

impl TagDao {
    /// Create a new tag
    pub fn create(conn: &Connection, tag: &Tag) -> Result<(), Error> {
        conn.execute(
            r#"
            INSERT INTO tags (id, name, color, icon, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![tag.id, tag.name, tag.color, tag.icon, tag.created_at],
        )?;
        Ok(())
    }

    /// Get a tag by ID
    pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Tag>, Error> {
        let mut stmt = conn.prepare("SELECT id, name, color, icon, created_at FROM tags WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![id], |row| Self::row_to_tag(row))?;

        match rows.next() {
            Some(Ok(tag)) => Ok(Some(tag)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// Get a tag by name
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<Tag>, Error> {
        let mut stmt = conn.prepare("SELECT id, name, color, icon, created_at FROM tags WHERE name = ?1")?;
        let mut rows = stmt.query_map(params![name], |row| Self::row_to_tag(row))?;

        match rows.next() {
            Some(Ok(tag)) => Ok(Some(tag)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// List all tags
    pub fn list(conn: &Connection) -> Result<Vec<Tag>, Error> {
        let mut stmt = conn.prepare("SELECT id, name, color, icon, created_at FROM tags ORDER BY name")?;
        let rows = stmt.query_map([], |row| Self::row_to_tag(row))?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }

    /// Update a tag
    pub fn update(conn: &Connection, tag: &Tag) -> Result<(), Error> {
        conn.execute(
            "UPDATE tags SET name = ?2, color = ?3, icon = ?4 WHERE id = ?1",
            params![tag.id, tag.name, tag.color, tag.icon],
        )?;
        Ok(())
    }

    /// Delete a tag
    pub fn delete(conn: &Connection, id: &str) -> Result<(), Error> {
        conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn row_to_tag(row: &Row) -> rusqlite::Result<Tag> {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            icon: row.get(3)?,
            created_at: row.get(4)?,
        })
    }
}

/// Attachment DAO
pub struct AttachmentDao;

impl AttachmentDao {
    /// Create a new attachment
    pub fn create(conn: &Connection, attachment: &Attachment) -> Result<(), Error> {
        conn.execute(
            r#"
            INSERT INTO attachments (id, file_name, file_path, file_type, mime_type, file_size, width, height, hash, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                attachment.id,
                attachment.file_name,
                attachment.file_path,
                attachment.file_type,
                attachment.mime_type,
                attachment.file_size,
                attachment.width,
                attachment.height,
                attachment.hash,
                attachment.created_at,
                attachment.updated_at
            ],
        )?;
        Ok(())
    }

    /// Get an attachment by ID
    pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Attachment>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, file_name, file_path, file_type, mime_type, file_size, width, height, hash, created_at, updated_at FROM attachments WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| Self::row_to_attachment(row))?;

        match rows.next() {
            Some(Ok(attachment)) => Ok(Some(attachment)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// Get an attachment by hash (for deduplication)
    pub fn get_by_hash(conn: &Connection, hash: &str) -> Result<Option<Attachment>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, file_name, file_path, file_type, mime_type, file_size, width, height, hash, created_at, updated_at FROM attachments WHERE hash = ?1"
        )?;
        let mut rows = stmt.query_map(params![hash], |row| Self::row_to_attachment(row))?;

        match rows.next() {
            Some(Ok(attachment)) => Ok(Some(attachment)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// Update an attachment
    pub fn update(conn: &Connection, attachment: &Attachment) -> Result<(), Error> {
        conn.execute(
            r#"
            UPDATE attachments
            SET file_name = ?2, file_path = ?3, file_type = ?4, mime_type = ?5, file_size = ?6, width = ?7, height = ?8, updated_at = ?9
            WHERE id = ?1
            "#,
            params![
                attachment.id,
                attachment.file_name,
                attachment.file_path,
                attachment.file_type,
                attachment.mime_type,
                attachment.file_size,
                attachment.width,
                attachment.height,
                attachment.updated_at
            ],
        )?;
        Ok(())
    }

    /// Delete an attachment
    pub fn delete(conn: &Connection, id: &str) -> Result<(), Error> {
        conn.execute("DELETE FROM attachments WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn row_to_attachment(row: &Row) -> rusqlite::Result<Attachment> {
        Ok(Attachment {
            id: row.get(0)?,
            file_name: row.get(1)?,
            file_path: row.get(2)?,
            file_type: row.get(3)?,
            mime_type: row.get(4)?,
            file_size: row.get(5)?,
            width: row.get(6)?,
            height: row.get(7)?,
            hash: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }
}

/// Link DAO
pub struct LinkDao;

impl LinkDao {
    /// Create a new link
    pub fn create(conn: &Connection, link: &Link) -> Result<(), Error> {
        conn.execute(
            r#"
            INSERT INTO links (id, source_note_id, target_note_id, source_block_id, target_block_id, link_type, link_text, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                link.id,
                link.source_note_id,
                link.target_note_id,
                link.source_block_id,
                link.target_block_id,
                link.link_type,
                link.link_text,
                link.created_at
            ],
        )?;
        Ok(())
    }

    /// Get a link by ID
    pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Link>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, source_note_id, target_note_id, source_block_id, target_block_id, link_type, link_text, created_at FROM links WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| Self::row_to_link(row))?;

        match rows.next() {
            Some(Ok(link)) => Ok(Some(link)),
            Some(Err(e)) => Err(Error::Database(e)),
            None => Ok(None),
        }
    }

    /// Get all links from a note (outgoing links)
    pub fn get_outgoing_links(conn: &Connection, note_id: &str) -> Result<Vec<Link>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, source_note_id, target_note_id, source_block_id, target_block_id, link_type, link_text, created_at FROM links WHERE source_note_id = ?1"
        )?;
        let rows = stmt.query_map(params![note_id], |row| Self::row_to_link(row))?;

        let mut links = Vec::new();
        for row in rows {
            links.push(row?);
        }
        Ok(links)
    }

    /// Get all links to a note (incoming links)
    pub fn get_incoming_links(conn: &Connection, note_id: &str) -> Result<Vec<Link>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, source_note_id, target_note_id, source_block_id, target_block_id, link_type, link_text, created_at FROM links WHERE target_note_id = ?1"
        )?;
        let rows = stmt.query_map(params![note_id], |row| Self::row_to_link(row))?;

        let mut links = Vec::new();
        for row in rows {
            links.push(row?);
        }
        Ok(links)
    }

    /// Get all links from a block
    pub fn get_links_from_block(conn: &Connection, block_id: &str) -> Result<Vec<Link>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, source_note_id, target_note_id, source_block_id, target_block_id, link_type, link_text, created_at FROM links WHERE source_block_id = ?1"
        )?;
        let rows = stmt.query_map(params![block_id], |row| Self::row_to_link(row))?;

        let mut links = Vec::new();
        for row in rows {
            links.push(row?);
        }
        Ok(links)
    }

    /// Get all links to a block
    pub fn get_links_to_block(conn: &Connection, block_id: &str) -> Result<Vec<Link>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, source_note_id, target_note_id, source_block_id, target_block_id, link_type, link_text, created_at FROM links WHERE target_block_id = ?1"
        )?;
        let rows = stmt.query_map(params![block_id], |row| Self::row_to_link(row))?;

        let mut links = Vec::new();
        for row in rows {
            links.push(row?);
        }
        Ok(links)
    }

    /// Delete a link
    pub fn delete(conn: &Connection, id: &str) -> Result<(), Error> {
        conn.execute("DELETE FROM links WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Delete all links from a note
    pub fn delete_outgoing_links(conn: &Connection, note_id: &str) -> Result<(), Error> {
        conn.execute("DELETE FROM links WHERE source_note_id = ?1", params![note_id])?;
        Ok(())
    }

    /// Delete all links to a note
    pub fn delete_incoming_links(conn: &Connection, note_id: &str) -> Result<(), Error> {
        conn.execute("DELETE FROM links WHERE target_note_id = ?1", params![note_id])?;
        Ok(())
    }

    fn row_to_link(row: &Row) -> rusqlite::Result<Link> {
        Ok(Link {
            id: row.get(0)?,
            source_note_id: row.get(1)?,
            target_note_id: row.get(2)?,
            source_block_id: row.get(3)?,
            target_block_id: row.get(4)?,
            link_type: row.get(5)?,
            link_text: row.get(6)?,
            created_at: row.get(7)?,
        })
    }
}

/// Block Reference DAO
pub struct BlockReferenceDao;

impl BlockReferenceDao {
    /// Create a new block reference
    pub fn create(conn: &Connection, id: &str, source_block_id: &str, target_block_id: &str) -> Result<(), Error> {
        let created_at = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO block_references (id, source_block_id, target_block_id, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, source_block_id, target_block_id, created_at],
        )?;
        Ok(())
    }

    /// Get all blocks that reference a block
    pub fn get_referencing_blocks(conn: &Connection, block_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT source_block_id FROM block_references WHERE target_block_id = ?1"
        )?;
        let rows = stmt.query_map(params![block_id], |row| row.get(0))?;

        let mut blocks = Vec::new();
        for row in rows {
            blocks.push(row?);
        }
        Ok(blocks)
    }

    /// Get all blocks referenced by a block
    pub fn get_referenced_blocks(conn: &Connection, block_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT target_block_id FROM block_references WHERE source_block_id = ?1"
        )?;
        let rows = stmt.query_map(params![block_id], |row| row.get(0))?;

        let mut blocks = Vec::new();
        for row in rows {
            blocks.push(row?);
        }
        Ok(blocks)
    }

    /// Delete a block reference
    pub fn delete(conn: &Connection, source_block_id: &str, target_block_id: &str) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM block_references WHERE source_block_id = ?1 AND target_block_id = ?2",
            params![source_block_id, target_block_id],
        )?;
        Ok(())
    }

    /// Delete all references from a block
    pub fn delete_references_from(conn: &Connection, block_id: &str) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM block_references WHERE source_block_id = ?1",
            params![block_id],
        )?;
        Ok(())
    }

    /// Delete all references to a block
    pub fn delete_references_to(conn: &Connection, block_id: &str) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM block_references WHERE target_block_id = ?1",
            params![block_id],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::DatabaseManager;

    #[test]
    fn test_link_dao() {
        let db = DatabaseManager::in_memory().unwrap();
        let conn = db.conn();

        // Create test notes
        use crate::core::models::Note;
        let note1 = Note::new("note-1".to_string(), "Note 1".to_string(), "notes/note1.md".to_string());
        let note2 = Note::new("note-2".to_string(), "Note 2".to_string(), "notes/note2.md".to_string());
        NoteDao::create(conn, &note1).unwrap();
        NoteDao::create(conn, &note2).unwrap();

        // Create a link
        let link = Link::new_note_link(
            "link-1".to_string(),
            "note-1".to_string(),
            "note-2".to_string(),
            Some("Link text".to_string()),
        );
        LinkDao::create(conn, &link).unwrap();

        // Get outgoing links
        let outgoing = LinkDao::get_outgoing_links(conn, "note-1").unwrap();
        assert_eq!(outgoing.len(), 1);
        assert_eq!(outgoing[0].target_note_id, Some("note-2".to_string()));

        // Get incoming links
        let incoming = LinkDao::get_incoming_links(conn, "note-2").unwrap();
        assert_eq!(incoming.len(), 1);
        assert_eq!(incoming[0].source_note_id, "note-1".to_string());
    }

    #[test]
    fn test_block_reference_dao() {
        let db = DatabaseManager::in_memory().unwrap();
        let conn = db.conn();

        // Create test note and blocks
        use crate::core::models::{Note, Block};
        let note = Note::new("note-1".to_string(), "Test".to_string(), "notes/test.md".to_string());
        NoteDao::create(conn, &note).unwrap();

        let block1 = Block::new("block-1".to_string(), "note-1".to_string(), "paragraph".to_string(), "Content 1".to_string(), 0);
        let block2 = Block::new("block-2".to_string(), "note-1".to_string(), "paragraph".to_string(), "Content 2".to_string(), 1);
        BlockDao::create(conn, &block1).unwrap();
        BlockDao::create(conn, &block2).unwrap();

        // Create a block reference
        BlockReferenceDao::create(conn, "ref-1", "block-1", "block-2").unwrap();

        // Get referencing blocks
        let referencing = BlockReferenceDao::get_referencing_blocks(conn, "block-2").unwrap();
        assert_eq!(referencing.len(), 1);
        assert_eq!(referencing[0], "block-1");

        // Get referenced blocks
        let referenced = BlockReferenceDao::get_referenced_blocks(conn, "block-1").unwrap();
        assert_eq!(referenced.len(), 1);
        assert_eq!(referenced[0], "block-2");
    }

    #[test]
    fn test_note_dao() {
        let db = DatabaseManager::in_memory().unwrap();
        let conn = db.conn();

        // Create a note
        let note = Note::new(
            "note-123".to_string(),
            "Test Note".to_string(),
            "notes/test.md".to_string(),
        );
        NoteDao::create(conn, &note).unwrap();

        // Get the note
        let retrieved = NoteDao::get_by_id(conn, "note-123", false).unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "note-123");
        assert_eq!(retrieved.title, "Test Note");

        // Update the note
        let mut updated = retrieved;
        updated.update_title("Updated Title".to_string());
        NoteDao::update(conn, &updated).unwrap();

        // Verify update
        let retrieved = NoteDao::get_by_id(conn, "note-123", false).unwrap().unwrap();
        assert_eq!(retrieved.title, "Updated Title");

        // Soft delete
        NoteDao::soft_delete(conn, "note-123").unwrap();
        let retrieved = NoteDao::get_by_id(conn, "note-123", false).unwrap();
        assert!(retrieved.is_none());

        // Restore
        NoteDao::restore(conn, "note-123").unwrap();
        let retrieved = NoteDao::get_by_id(conn, "note-123", false).unwrap();
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_tag_dao() {
        let db = DatabaseManager::in_memory().unwrap();
        let conn = db.conn();

        // Create a tag
        let tag = Tag::new("tag-123".to_string(), "Rust".to_string());
        TagDao::create(conn, &tag).unwrap();

        // Get by ID
        let retrieved = TagDao::get_by_id(conn, "tag-123").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Rust");

        // Get by name
        let retrieved = TagDao::get_by_name(conn, "Rust").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, "tag-123");
    }
}
