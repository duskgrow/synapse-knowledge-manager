//! Synapse Knowledge Manager
//!
//! A local-first knowledge management system combining the best of Notion and Obsidian.
//! Application layer re-exports core and editor crates.

pub mod core {
    pub use synapse_core::*;
}

pub mod editor {
    pub use synapse_editor::*;
}
