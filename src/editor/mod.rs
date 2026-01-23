//! Editor module
//!
//! This module handles Markdown editing, rendering, and Vim mode support.
//!
//! Currently implements:
//! - Markdown parsing (using pulldown-cmark)
//! - Markdown to HTML rendering
//! - Block parsing (converting Markdown to Block structures)
//!
//! Future implementations:
//! - CodeMirror 6 integration (frontend)
//! - Vim mode support (frontend)
//! - Real-time preview (frontend)

mod parser;
mod renderer;

pub use parser::parse_markdown_to_blocks;
pub use renderer::render_markdown_to_html;
