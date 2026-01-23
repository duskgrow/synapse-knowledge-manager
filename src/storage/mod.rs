//! Storage module
//!
//! This module handles data persistence using SQLite and file system.

mod database;
mod db_manager;
mod dao;
mod relation_dao;

pub use database::init_database;
pub use db_manager::DatabaseManager;
pub use dao::*;
pub use relation_dao::*;
