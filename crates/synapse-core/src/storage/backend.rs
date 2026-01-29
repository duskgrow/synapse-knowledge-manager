//! Storage backend trait for swappable persistence.
//!
//! Defined for future use (e.g. mock in tests, alternative backends).
//! Current code path uses DatabaseManager directly because rusqlite::Connection
//! is !Send/!Sync; use from a single thread (e.g. Tauri main thread).

use rusqlite::Connection;

use super::DatabaseManager;

/// Storage backend abstraction. Implementations can be swapped (e.g. SQLite, mock).
/// No Send/Sync bound so that SQLite backend can be used from a single thread.
pub trait StorageBackend {
    fn conn(&self) -> &Connection;
}

/// SQLite-backed storage (default implementation).
pub struct SqliteBackend(DatabaseManager);

impl SqliteBackend {
    pub fn new(db: DatabaseManager) -> Self {
        Self(db)
    }
}

impl StorageBackend for SqliteBackend {
    fn conn(&self) -> &Connection {
        self.0.conn()
    }
}
