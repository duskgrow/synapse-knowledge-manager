//! Database connection manager

use rusqlite::{Connection, Result};
use std::path::Path;

use super::database::init_database;

/// Database manager for handling SQLite connections
pub struct DatabaseManager {
    conn: Connection,
}

impl DatabaseManager {
    /// Create a new database manager with a connection to the database file
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        init_database(&conn)?;
        Ok(Self { conn })
    }

    /// Create a new in-memory database (useful for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        init_database(&conn)?;
        Ok(Self { conn })
    }

    /// Get a reference to the database connection
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Get a mutable reference to the database connection
    pub fn conn_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_manager_creation() {
        let db = DatabaseManager::in_memory().unwrap();
        // Verify we can query the database
        let mut stmt = db
            .conn()
            .prepare("SELECT COUNT(*) FROM notes")
            .unwrap();
        let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 0);
    }
}
