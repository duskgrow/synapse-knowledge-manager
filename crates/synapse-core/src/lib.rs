//! Synapse Core: models, storage abstraction, and services.

pub mod error;
pub mod models;
pub mod storage;
pub mod services;

pub use error::{Error, Result};
pub use models::*;
pub use services::{
    ServiceContext,
    NoteService, TagService, FolderService, LinkService,
    SearchService, BlockService, AttachmentService,
};
