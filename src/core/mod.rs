//! Core module
//!
//! This module contains the core business logic, data models, and services.

pub mod models;
pub mod services;
pub mod error;

pub use error::{Error, Result};
pub use services::{
    ServiceContext,
    NoteService, NoteWithContent,
    TagService,
    FolderService,
    LinkService,
    SearchService,
};
