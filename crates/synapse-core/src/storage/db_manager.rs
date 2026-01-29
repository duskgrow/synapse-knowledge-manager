//! Database connection manager

use rusqlite::{Connection, Result};
use std::path::Path;

use super::database::init_database;

pub struct DatabaseManager {
    conn: Connection,
}

impl DatabaseManager {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        init_database(&conn)?;
        Ok(Self { conn })
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        init_database(&conn)?;
        Ok(Self { conn })
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

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
        let count: i64 = db.conn().prepare("SELECT COUNT(*) FROM notes").unwrap().query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 0);
    }
}
