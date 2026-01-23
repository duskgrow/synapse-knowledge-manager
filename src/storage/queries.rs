//! SQL query constants for type safety and maintainability

/// Note table queries
pub mod notes {
    pub const CREATE: &str = r#"
        INSERT INTO notes (id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
    "#;

    pub const SELECT_BASE: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
    "#;

    pub const SELECT_BY_ID: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
        WHERE id = ?1
    "#;

    pub const SELECT_BY_ID_NOT_DELETED: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
        WHERE id = ?1 AND is_deleted = 0
    "#;

    pub const SELECT_BY_ID_INCLUDE_DELETED: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
        WHERE id = ?1
    "#;

    pub const UPDATE: &str = r#"
        UPDATE notes
        SET title = ?2, content_path = ?3, updated_at = ?4, word_count = ?5, is_deleted = ?6, deleted_at = ?7
        WHERE id = ?1
    "#;

    pub const SOFT_DELETE: &str = r#"
        UPDATE notes
        SET is_deleted = 1, deleted_at = ?2
        WHERE id = ?1
    "#;

    pub const RESTORE: &str = r#"
        UPDATE notes
        SET is_deleted = 0, deleted_at = NULL
        WHERE id = ?1
    "#;

    pub const SELECT_ALL: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
        WHERE is_deleted = 0
        ORDER BY updated_at DESC
    "#;

    pub const SELECT_ALL_INCLUDE_DELETED: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
        ORDER BY updated_at DESC
    "#;

    pub const SEARCH_BY_TITLE: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
        WHERE title LIKE ?1 AND is_deleted = 0
        ORDER BY updated_at DESC
    "#;

    pub const SEARCH_BY_TITLE_INCLUDE_DELETED: &str = r#"
        SELECT id, title, content_path, created_at, updated_at, word_count, is_deleted, deleted_at
        FROM notes
        WHERE title LIKE ?1
        ORDER BY updated_at DESC
    "#;

    pub const SELECT_BY_FOLDER: &str = r#"
        SELECT n.id, n.title, n.content_path, n.created_at, n.updated_at, n.word_count, n.is_deleted, n.deleted_at
        FROM notes n
        INNER JOIN note_folders nf ON n.id = nf.note_id
        WHERE nf.folder_id = ?1 AND n.is_deleted = 0
        ORDER BY nf.position, n.updated_at DESC
    "#;

    pub const SELECT_BY_FOLDER_INCLUDE_DELETED: &str = r#"
        SELECT n.id, n.title, n.content_path, n.created_at, n.updated_at, n.word_count, n.is_deleted, n.deleted_at
        FROM notes n
        INNER JOIN note_folders nf ON n.id = nf.note_id
        WHERE nf.folder_id = ?1
        ORDER BY nf.position, n.updated_at DESC
    "#;
}

/// Block table queries
pub mod blocks {
    pub const CREATE: &str = r#"
        INSERT INTO blocks (id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
    "#;

    pub const SELECT_BY_ID: &str = r#"
        SELECT id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at
        FROM blocks
        WHERE id = ?1 AND is_deleted = 0
    "#;

    pub const SELECT_BY_ID_INCLUDE_DELETED: &str = r#"
        SELECT id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at
        FROM blocks
        WHERE id = ?1
    "#;

    pub const SELECT_BY_NOTE: &str = r#"
        SELECT id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at
        FROM blocks
        WHERE note_id = ?1 AND is_deleted = 0
        ORDER BY position
    "#;

    pub const SELECT_BY_NOTE_INCLUDE_DELETED: &str = r#"
        SELECT id, note_id, block_type, content, position, created_at, updated_at, is_deleted, deleted_at
        FROM blocks
        WHERE note_id = ?1
        ORDER BY position
    "#;

    pub const UPDATE: &str = r#"
        UPDATE blocks
        SET block_type = ?2, content = ?3, position = ?4, updated_at = ?5, is_deleted = ?6, deleted_at = ?7
        WHERE id = ?1
    "#;

    pub const SOFT_DELETE: &str = r#"
        UPDATE blocks
        SET is_deleted = 1, deleted_at = ?2
        WHERE id = ?1
    "#;

    pub const RESTORE: &str = r#"
        UPDATE blocks
        SET is_deleted = 0, deleted_at = NULL
        WHERE id = ?1
    "#;
}

/// Folder table queries
pub mod folders {
    pub const CREATE: &str = r#"
        INSERT INTO folders (id, name, parent_id, path, created_at, updated_at, position)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
    "#;

    pub const SELECT_BY_ID: &str = r#"
        SELECT id, name, parent_id, path, created_at, updated_at, position
        FROM folders
        WHERE id = ?1
    "#;

    pub const SELECT_ROOTS: &str = r#"
        SELECT id, name, parent_id, path, created_at, updated_at, position
        FROM folders
        WHERE parent_id IS NULL
        ORDER BY position
    "#;

    pub const SELECT_CHILDREN: &str = r#"
        SELECT id, name, parent_id, path, created_at, updated_at, position
        FROM folders
        WHERE parent_id = ?1
        ORDER BY position
    "#;

    pub const UPDATE: &str = r#"
        UPDATE folders
        SET name = ?2, parent_id = ?3, path = ?4, updated_at = ?5, position = ?6
        WHERE id = ?1
    "#;

    pub const DELETE: &str = r#"
        DELETE FROM folders
        WHERE id = ?1
    "#;
}

/// Tag table queries
pub mod tags {
    pub const CREATE: &str = r#"
        INSERT INTO tags (id, name, color, icon, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
    "#;

    pub const SELECT_BY_ID: &str = r#"
        SELECT id, name, color, icon, created_at
        FROM tags
        WHERE id = ?1
    "#;

    pub const SELECT_BY_NAME: &str = r#"
        SELECT id, name, color, icon, created_at
        FROM tags
        WHERE name = ?1
    "#;

    pub const SELECT_ALL: &str = r#"
        SELECT id, name, color, icon, created_at
        FROM tags
        ORDER BY name
    "#;

    pub const UPDATE: &str = r#"
        UPDATE tags
        SET name = ?2, color = ?3, icon = ?4
        WHERE id = ?1
    "#;

    pub const DELETE: &str = r#"
        DELETE FROM tags
        WHERE id = ?1
    "#;
}

/// Attachment table queries
pub mod attachments {
    pub const CREATE: &str = r#"
        INSERT INTO attachments (id, file_name, file_path, file_type, mime_type, file_size, width, height, hash, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
    "#;

    pub const SELECT_BY_ID: &str = r#"
        SELECT id, file_name, file_path, file_type, mime_type, file_size, width, height, hash, created_at, updated_at
        FROM attachments
        WHERE id = ?1
    "#;

    pub const SELECT_BY_HASH: &str = r#"
        SELECT id, file_name, file_path, file_type, mime_type, file_size, width, height, hash, created_at, updated_at
        FROM attachments
        WHERE hash = ?1
    "#;

    pub const UPDATE: &str = r#"
        UPDATE attachments
        SET file_name = ?2, file_path = ?3, file_type = ?4, mime_type = ?5, file_size = ?6, width = ?7, height = ?8, updated_at = ?9
        WHERE id = ?1
    "#;

    pub const DELETE: &str = r#"
        DELETE FROM attachments
        WHERE id = ?1
    "#;
}
