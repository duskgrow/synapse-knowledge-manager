//! Markdown renderer module
//!
//! This module handles rendering Markdown content to HTML.

use pulldown_cmark::{html, Options, Parser};

/// Render Markdown content to HTML
pub fn render_markdown_to_html(content: &str) -> String {
    // Enable all GFM features
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_heading() {
        let content = "# Heading";
        let html = render_markdown_to_html(content);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Heading"));
    }

    #[test]
    fn test_render_paragraph() {
        let content = "This is a paragraph.";
        let html = render_markdown_to_html(content);
        assert!(html.contains("<p>"));
        assert!(html.contains("This is a paragraph."));
    }

    #[test]
    fn test_render_code_block() {
        let content = "```rust\nfn main() {}\n```";
        let html = render_markdown_to_html(content);
        assert!(html.contains("<code"));
    }

    #[test]
    fn test_render_list() {
        let content = "- Item 1\n- Item 2";
        let html = render_markdown_to_html(content);
        assert!(html.contains("<ul>"));
        assert!(html.contains("Item 1"));
    }
}
