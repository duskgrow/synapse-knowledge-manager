//! Synapse Editor: parse/render and EditorCore.

mod core;
mod parser;
mod renderer;

pub use core::EditorCore;
pub use parser::parse_markdown_to_blocks;
pub use renderer::render_markdown_to_html;
