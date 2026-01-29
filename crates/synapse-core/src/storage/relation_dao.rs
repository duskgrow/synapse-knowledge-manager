//! Data Access Object (DAO) layer for relation tables

use rusqlite::{params, Connection};

use crate::Error;

/// Note-Folder relation DAO
pub struct NoteFolderDao;

impl NoteFolderDao {
    /// Add a note to a folder
    pub fn add(conn: &Connection, note_id: &str, folder_id: &str, is_primary: bool, position: i64) -> Result<(), Error> {
        let created_at = chrono::Utc::now().timestamp();
        conn.execute(
            r#"
            INSERT INTO note_folders (note_id, folder_id, is_primary, position, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![note_id, folder_id, is_primary as i32, position, created_at],
        )?;
        Ok(())
    }

    /// Remove a note from a folder
    pub fn remove(conn: &Connection, note_id: &str, folder_id: &str) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM note_folders WHERE note_id = ?1 AND folder_id = ?2",
            params![note_id, folder_id],
        )?;
        Ok(())
    }

    /// Set primary folder for a note
    pub fn set_primary(conn: &Connection, note_id: &str, folder_id: &str) -> Result<(), Error> {
        // First, unset all primary folders for this note
        conn.execute(
            "UPDATE note_folders SET is_primary = 0 WHERE note_id = ?1",
            params![note_id],
        )?;
        // Then set the specified folder as primary
        conn.execute(
            "UPDATE note_folders SET is_primary = 1 WHERE note_id = ?1 AND folder_id = ?2",
            params![note_id, folder_id],
        )?;
        Ok(())
    }

    /// Get all folders for a note
    pub fn get_folders_for_note(conn: &Connection, note_id: &str) -> Result<Vec<(String, bool, i64)>, Error> {
        let mut stmt = conn.prepare(
            "SELECT folder_id, is_primary, position FROM note_folders WHERE note_id = ?1 ORDER BY is_primary DESC, position"
        )?;
        let rows = stmt.query_map(params![note_id], |row| {
            Ok((row.get(0)?, row.get::<_, i32>(1)? != 0, row.get(2)?))
        })?;

        let mut folders = Vec::new();
        for row in rows {
            folders.push(row?);
        }
        Ok(folders)
    }

    /// Get all notes in a folder
    pub fn get_notes_in_folder(conn: &Connection, folder_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT note_id FROM note_folders WHERE folder_id = ?1 ORDER BY position"
        )?;
        let rows = stmt.query_map(params![folder_id], |row| row.get(0))?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        Ok(notes)
    }

    /// Update note position in folder
    pub fn update_position(conn: &Connection, note_id: &str, folder_id: &str, position: i64) -> Result<(), Error> {
        conn.execute(
            "UPDATE note_folders SET position = ?3 WHERE note_id = ?1 AND folder_id = ?2",
            params![note_id, folder_id, position],
        )?;
        Ok(())
    }
}

/// Note-Tag relation DAO
pub struct NoteTagDao;

impl NoteTagDao {
    /// Add a tag to a note
    pub fn add(conn: &Connection, note_id: &str, tag_id: &str) -> Result<(), Error> {
        let created_at = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO note_tags (note_id, tag_id, created_at) VALUES (?1, ?2, ?3)",
            params![note_id, tag_id, created_at],
        )?;
        Ok(())
    }

    /// Remove a tag from a note
    pub fn remove(conn: &Connection, note_id: &str, tag_id: &str) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM note_tags WHERE note_id = ?1 AND tag_id = ?2",
            params![note_id, tag_id],
        )?;
        Ok(())
    }

    /// Get all tags for a note
    pub fn get_tags_for_note(conn: &Connection, note_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT tag_id FROM note_tags WHERE note_id = ?1"
        )?;
        let rows = stmt.query_map(params![note_id], |row| row.get(0))?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }

    /// Get all notes with a tag
    pub fn get_notes_with_tag(conn: &Connection, tag_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT note_id FROM note_tags WHERE tag_id = ?1"
        )?;
        let rows = stmt.query_map(params![tag_id], |row| row.get(0))?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        Ok(notes)
    }
}

/// Note-Attachment relation DAO
pub struct NoteAttachmentDao;

impl NoteAttachmentDao {
    /// Add an attachment to a note
    pub fn add(conn: &Connection, note_id: &str, attachment_id: &str, position: i64) -> Result<(), Error> {
        let created_at = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO note_attachments (note_id, attachment_id, position, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![note_id, attachment_id, position, created_at],
        )?;
        Ok(())
    }

    /// Remove an attachment from a note
    pub fn remove(conn: &Connection, note_id: &str, attachment_id: &str) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM note_attachments WHERE note_id = ?1 AND attachment_id = ?2",
            params![note_id, attachment_id],
        )?;
        Ok(())
    }

    /// Get all attachments for a note
    pub fn get_attachments_for_note(conn: &Connection, note_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT attachment_id FROM note_attachments WHERE note_id = ?1 ORDER BY position"
        )?;
        let rows = stmt.query_map(params![note_id], |row| row.get(0))?;

        let mut attachments = Vec::new();
        for row in rows {
            attachments.push(row?);
        }
        Ok(attachments)
    }

    /// Get all notes with an attachment
    pub fn get_notes_with_attachment(conn: &Connection, attachment_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT note_id FROM note_attachments WHERE attachment_id = ?1"
        )?;
        let rows = stmt.query_map(params![attachment_id], |row| row.get(0))?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        Ok(notes)
    }

    /// Update attachment position in note
    pub fn update_position(conn: &Connection, note_id: &str, attachment_id: &str, position: i64) -> Result<(), Error> {
        conn.execute(
            "UPDATE note_attachments SET position = ?3 WHERE note_id = ?1 AND attachment_id = ?2",
            params![note_id, attachment_id, position],
        )?;
        Ok(())
    }
}

/// Block-Attachment relation DAO
pub struct BlockAttachmentDao;

impl BlockAttachmentDao {
    /// Add an attachment to a block
    pub fn add(conn: &Connection, block_id: &str, attachment_id: &str) -> Result<(), Error> {
        let created_at = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO block_attachments (block_id, attachment_id, created_at) VALUES (?1, ?2, ?3)",
            params![block_id, attachment_id, created_at],
        )?;
        Ok(())
    }

    /// Remove an attachment from a block
    pub fn remove(conn: &Connection, block_id: &str, attachment_id: &str) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM block_attachments WHERE block_id = ?1 AND attachment_id = ?2",
            params![block_id, attachment_id],
        )?;
        Ok(())
    }

    /// Get all attachments for a block
    pub fn get_attachments_for_block(conn: &Connection, block_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT attachment_id FROM block_attachments WHERE block_id = ?1"
        )?;
        let rows = stmt.query_map(params![block_id], |row| row.get(0))?;

        let mut attachments = Vec::new();
        for row in rows {
            attachments.push(row?);
        }
        Ok(attachments)
    }

    /// Get all blocks with an attachment
    pub fn get_blocks_with_attachment(conn: &Connection, attachment_id: &str) -> Result<Vec<String>, Error> {
        let mut stmt = conn.prepare(
            "SELECT block_id FROM block_attachments WHERE attachment_id = ?1"
        )?;
        let rows = stmt.query_map(params![attachment_id], |row| row.get(0))?;

        let mut blocks = Vec::new();
        for row in rows {
            blocks.push(row?);
        }
        Ok(blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::DatabaseManager;

    #[test]
    fn test_note_folder_relation() {
        let db = DatabaseManager::in_memory().unwrap();
        let conn = db.conn();

        // Create test note and folder
        use crate::storage::dao::{NoteDao, FolderDao};
        use crate::models::{Note, Folder};

        let note = Note::new("note-1".to_string(), "Test".to_string(), "notes/test.md".to_string());
        NoteDao::create(conn, &note).unwrap();

        let folder = Folder::new("folder-1".to_string(), "Test Folder".to_string(), None, "/Test Folder".to_string());
        FolderDao::create(conn, &folder).unwrap();

        // Add note to folder
        NoteFolderDao::add(conn, "note-1", "folder-1", true, 0).unwrap();

        // Get folders for note
        let folders = NoteFolderDao::get_folders_for_note(conn, "note-1").unwrap();
        assert_eq!(folders.len(), 1);
        assert_eq!(folders[0].0, "folder-1");
        assert!(folders[0].1); // is_primary

        // Get notes in folder
        let notes = NoteFolderDao::get_notes_in_folder(conn, "folder-1").unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0], "note-1");
    }

    #[test]
    fn test_note_tag_relation() {
        let db = DatabaseManager::in_memory().unwrap();
        let conn = db.conn();

        use crate::storage::dao::{NoteDao, TagDao};
        use crate::models::{Note, Tag};

        let note = Note::new("note-1".to_string(), "Test".to_string(), "notes/test.md".to_string());
        NoteDao::create(conn, &note).unwrap();

        let tag = Tag::new("tag-1".to_string(), "Rust".to_string());
        TagDao::create(conn, &tag).unwrap();

        // Add tag to note
        NoteTagDao::add(conn, "note-1", "tag-1").unwrap();

        // Get tags for note
        let tags = NoteTagDao::get_tags_for_note(conn, "note-1").unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "tag-1");

        // Get notes with tag
        let notes = NoteTagDao::get_notes_with_tag(conn, "tag-1").unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0], "note-1");
    }
}
