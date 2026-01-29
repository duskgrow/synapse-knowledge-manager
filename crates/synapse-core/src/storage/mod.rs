//! Storage abstraction and SQLite implementation.
//!
//! Storage is abstracted behind [`StorageBackend`] so the implementation can be swapped
//! (e.g. for testing or alternative backends). Per-entity storage traits (NoteStorage,
//! BlockStorage, etc.) may be introduced in a later refinement.

mod backend;
mod database;
mod db_manager;
mod dao;
mod relation_dao;

pub use backend::{StorageBackend, SqliteBackend};
pub use database::init_database;
pub use db_manager::DatabaseManager;
pub use dao::*;
pub use relation_dao::*;
