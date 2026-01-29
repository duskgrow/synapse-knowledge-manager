//! Database schema and migration

use rusqlite::{Connection, Result};

/// Current database schema version
const SCHEMA_VERSION: u32 = 1;

/// Initialize the database with all tables
pub fn init_database(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    create_notes_table(conn)?;
    create_blocks_table(conn)?;
    create_folders_table(conn)?;
    create_note_folders_table(conn)?;
    create_tags_table(conn)?;
    create_note_tags_table(conn)?;
    create_links_table(conn)?;
    create_block_references_table(conn)?;
    create_databases_table(conn)?;
    create_database_notes_table(conn)?;
    create_attachments_table(conn)?;
    create_note_attachments_table(conn)?;
    create_block_attachments_table(conn)?;
    create_fts_tables(conn)?;
    create_indexes(conn)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY)",
        [],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
        [SCHEMA_VERSION],
    )?;
    Ok(())
}

fn create_notes_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content_path TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            word_count INTEGER DEFAULT 0,
            is_deleted INTEGER DEFAULT 0,
            deleted_at INTEGER
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_blocks_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS blocks (
            id TEXT PRIMARY KEY,
            note_id TEXT NOT NULL,
            block_type TEXT NOT NULL,
            content TEXT NOT NULL,
            position INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            is_deleted INTEGER DEFAULT 0,
            deleted_at INTEGER,
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_folders_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id TEXT,
            path TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            position INTEGER DEFAULT 0,
            FOREIGN KEY (parent_id) REFERENCES folders(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_note_folders_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS note_folders (
            note_id TEXT NOT NULL,
            folder_id TEXT NOT NULL,
            is_primary INTEGER DEFAULT 0,
            position INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            PRIMARY KEY (note_id, folder_id),
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_tags_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            color TEXT,
            icon TEXT,
            created_at INTEGER NOT NULL
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_note_tags_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS note_tags (
            note_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            PRIMARY KEY (note_id, tag_id),
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_links_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS links (
            id TEXT PRIMARY KEY,
            source_note_id TEXT NOT NULL,
            target_note_id TEXT,
            source_block_id TEXT,
            target_block_id TEXT,
            link_type TEXT NOT NULL,
            link_text TEXT,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (source_note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (target_note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (source_block_id) REFERENCES blocks(id) ON DELETE CASCADE,
            FOREIGN KEY (target_block_id) REFERENCES blocks(id) ON DELETE CASCADE,
            CHECK (
                (link_type = 'note_link' AND target_note_id IS NOT NULL) OR
                (link_type = 'block_reference' AND target_block_id IS NOT NULL) OR
                (link_type = 'database_relation' AND target_note_id IS NOT NULL)
            )
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_block_references_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS block_references (
            id TEXT PRIMARY KEY,
            source_block_id TEXT NOT NULL,
            target_block_id TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (source_block_id) REFERENCES blocks(id) ON DELETE CASCADE,
            FOREIGN KEY (target_block_id) REFERENCES blocks(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_databases_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS databases (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            properties TEXT NOT NULL,
            views TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_database_notes_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS database_notes (
            db_id TEXT NOT NULL,
            note_id TEXT NOT NULL,
            properties TEXT NOT NULL,
            position INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            PRIMARY KEY (db_id, note_id),
            FOREIGN KEY (db_id) REFERENCES databases(id) ON DELETE CASCADE,
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_attachments_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS attachments (
            id TEXT PRIMARY KEY,
            file_name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            file_type TEXT NOT NULL,
            mime_type TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            width INTEGER,
            height INTEGER,
            hash TEXT NOT NULL UNIQUE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_note_attachments_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS note_attachments (
            note_id TEXT NOT NULL,
            attachment_id TEXT NOT NULL,
            position INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            PRIMARY KEY (note_id, attachment_id),
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (attachment_id) REFERENCES attachments(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_block_attachments_table(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS block_attachments (
            block_id TEXT NOT NULL,
            attachment_id TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            PRIMARY KEY (block_id, attachment_id),
            FOREIGN KEY (block_id) REFERENCES blocks(id) ON DELETE CASCADE,
            FOREIGN KEY (attachment_id) REFERENCES attachments(id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_fts_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
            note_id UNINDEXED,
            title,
            content,
            content=notes,
            content_rowid=rowid
        )
        "#,
        [],
    )?;
    conn.execute(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS blocks_fts USING fts5(
            block_id UNINDEXED,
            content,
            content=blocks,
            content_rowid=rowid
        )
        "#,
        [],
    )?;
    Ok(())
}

fn create_indexes(conn: &Connection) -> Result<()> {
    conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_updated_at ON notes(updated_at)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_title ON notes(title)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_deleted_at ON notes(deleted_at)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_is_deleted ON notes(is_deleted)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_blocks_note_id ON blocks(note_id)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_blocks_position ON blocks(note_id, position)",
        [],
    )?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_blocks_content ON blocks(content)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_blocks_deleted_at ON blocks(deleted_at)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_blocks_is_deleted ON blocks(is_deleted)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_folders_parent_id ON folders(parent_id)",
        [],
    )?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_folders_path ON folders(path)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_folders_note_id ON note_folders(note_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_folders_folder_id ON note_folders(folder_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_folders_primary ON note_folders(note_id, is_primary)",
        [],
    )?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_tags_note_id ON note_tags(note_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_tags_tag_id ON note_tags(tag_id)",
        [],
    )?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_links_source_note ON links(source_note_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_links_target_note ON links(target_note_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_links_source_block ON links(source_block_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_links_target_block ON links(target_block_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_links_type ON links(link_type)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_block_refs_source ON block_references(source_block_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_block_refs_target ON block_references(target_block_id)",
        [],
    )?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_databases_name ON databases(name)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_db_notes_db_id ON database_notes(db_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_db_notes_note_id ON database_notes(note_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_attachments_type ON attachments(file_type)",
        [],
    )?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_attachments_hash ON attachments(hash)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_attachments_note_id ON note_attachments(note_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_attachments_attachment_id ON note_attachments(attachment_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_block_attachments_block_id ON block_attachments(block_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_block_attachments_attachment_id ON block_attachments(attachment_id)",
        [],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let conn = Connection::open_in_memory().unwrap();
        init_database(&conn).unwrap();
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap();
        let tables: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<String>, _>>()
            .unwrap();
        assert!(tables.contains(&"notes".to_string()));
        assert!(tables.contains(&"blocks".to_string()));
    }
}
