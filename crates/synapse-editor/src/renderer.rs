//! Markdown renderer: content -> HTML.

use pulldown_cmark::{Options, Parser, html};

/// Render Markdown content to HTML
pub fn render_markdown_to_html(content: &str) -> String {
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
        let html = render_markdown_to_html("# Heading");
        assert!(html.contains("<h1>"));
    }
}
