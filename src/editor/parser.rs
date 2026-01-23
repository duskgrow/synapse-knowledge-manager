//! Markdown parser module
//!
//! This module handles parsing Markdown content into Block structures.

use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use uuid::Uuid;

use crate::core::Result;
use crate::core::models::Block;

/// Parse Markdown content into blocks
pub fn parse_markdown_to_blocks(content: &str, note_id: &str) -> Result<Vec<Block>> {
    let parser = Parser::new(content);
    let mut blocks = Vec::new();
    let mut position = 0i64;
    let mut current_block_type = "paragraph".to_string();
    let mut current_content = String::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                // If we have accumulated content, save it as a block
                if !current_content.trim().is_empty() && !in_code_block {
                    let block_id = format!("block-{}", Uuid::new_v4());
                    blocks.push(Block::new(
                        block_id,
                        note_id.to_string(),
                        current_block_type.clone(),
                        current_content.trim().to_string(),
                        position,
                    ));
                    position += 1;
                    current_content.clear();
                }

                match tag {
                    Tag::Heading { level, .. } => {
                        current_block_type = format!("heading_{}", level);
                    }
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        code_block_lang = match kind {
                            pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                            pulldown_cmark::CodeBlockKind::Indented => String::new(),
                        };
                        current_block_type = "code_block".to_string();
                    }
                    Tag::List(Some(_)) => {
                        current_block_type = "ordered_list".to_string();
                    }
                    Tag::List(None) => {
                        current_block_type = "unordered_list".to_string();
                    }
                    Tag::Item => {
                        current_block_type = "list_item".to_string();
                    }
                    Tag::BlockQuote(_) => {
                        current_block_type = "quote".to_string();
                    }
                    Tag::Table(_) => {
                        current_block_type = "table".to_string();
                    }
                    Tag::TableRow => {
                        current_block_type = "table_row".to_string();
                    }
                    Tag::TableCell => {
                        current_block_type = "table_cell".to_string();
                    }
                    _ => {
                        // For other tags, keep current block type
                    }
                }
            }
            Event::End(tag_end) => {
                match tag_end {
                    TagEnd::CodeBlock => {
                        // Save code block
                        if !current_content.trim().is_empty() {
                            let block_id = format!("block-{}", Uuid::new_v4());
                            let mut block = Block::new(
                                block_id,
                                note_id.to_string(),
                                "code_block".to_string(),
                                current_content.trim().to_string(),
                                position,
                            );
                            // Store language in block_type or as metadata (simplified: append to content)
                            if !code_block_lang.is_empty() {
                                // For now, we'll store it in the content with a marker
                                // In the future, we might add a metadata field to Block
                                block.content =
                                    format!("```{}\n{}\n```", code_block_lang, block.content);
                            } else {
                                block.content = format!("```\n{}\n```", block.content);
                            }
                            blocks.push(block);
                            position += 1;
                            current_content.clear();
                        }
                        in_code_block = false;
                        code_block_lang.clear();
                    }
                    TagEnd::Heading(_)
                    | TagEnd::Paragraph
                    | TagEnd::List(_)
                    | TagEnd::Item
                    | TagEnd::BlockQuote(_)
                    | TagEnd::Table
                    | TagEnd::TableRow
                    | TagEnd::TableCell => {
                        // Save block if we have content
                        if !current_content.trim().is_empty() {
                            let block_id = format!("block-{}", Uuid::new_v4());
                            blocks.push(Block::new(
                                block_id,
                                note_id.to_string(),
                                current_block_type.clone(),
                                current_content.trim().to_string(),
                                position,
                            ));
                            position += 1;
                            current_content.clear();
                        }
                        // Reset to paragraph for next block
                        current_block_type = "paragraph".to_string();
                    }
                    _ => {
                        // Other tags don't need special handling
                    }
                }
            }
            Event::Text(text) => {
                current_content.push_str(&text);
            }
            Event::Code(code) => {
                // Inline code - wrap in backticks
                current_content.push_str(&format!("`{}`", code));
            }
            Event::Html(html) => {
                // HTML content - preserve as-is (for future support)
                current_content.push_str(&html);
            }
            Event::SoftBreak => {
                current_content.push('\n');
            }
            Event::HardBreak => {
                current_content.push_str("\n\n");
            }
            Event::Rule => {
                // Horizontal rule - save as separate block
                if !current_content.trim().is_empty() {
                    let block_id = format!("block-{}", Uuid::new_v4());
                    blocks.push(Block::new(
                        block_id,
                        note_id.to_string(),
                        current_block_type.clone(),
                        current_content.trim().to_string(),
                        position,
                    ));
                    position += 1;
                    current_content.clear();
                }
                // Add horizontal rule block
                let block_id = format!("block-{}", Uuid::new_v4());
                blocks.push(Block::new(
                    block_id,
                    note_id.to_string(),
                    "horizontal_rule".to_string(),
                    "---".to_string(),
                    position,
                ));
                position += 1;
            }
            Event::TaskListMarker(checked) => {
                // Task list marker
                let marker = if checked { "- [x]" } else { "- [ ]" };
                current_content.push_str(marker);
            }
            Event::FootnoteReference(_) => {
                // Footnotes - not fully supported yet, but we can store the content
            }
            Event::InlineMath(math) => {
                // Inline math formula - wrap in $...$
                current_content.push_str(&format!("${}$", math));
            }
            Event::DisplayMath(math) => {
                // Display math formula - wrap in $$...$$
                current_content.push_str(&format!("$${}\n$$", math));
            }
            Event::InlineHtml(html) => {
                // Inline HTML - preserve as-is
                current_content.push_str(&html);
            }
        }
    }

    // Save any remaining content
    if !current_content.trim().is_empty() {
        let block_id = format!("block-{}", Uuid::new_v4());
        blocks.push(Block::new(
            block_id,
            note_id.to_string(),
            current_block_type,
            current_content.trim().to_string(),
            position,
        ));
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_markdown() {
        let content = "# Heading\n\nThis is a paragraph.";
        let note_id = "note-123";
        let blocks = parse_markdown_to_blocks(content, note_id).unwrap();

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].block_type, "heading_h1");
        assert_eq!(blocks[0].content, "Heading");
        assert_eq!(blocks[1].block_type, "paragraph");
        assert_eq!(blocks[1].content, "This is a paragraph.");
    }

    #[test]
    fn test_parse_code_block() {
        let content = "```rust\nfn main() {}\n```";
        let note_id = "note-123";
        let blocks = parse_markdown_to_blocks(content, note_id).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].block_type, "code_block");
        assert!(blocks[0].content.contains("rust"));
    }

    #[test]
    fn test_parse_list() {
        let content = "- Item 1\n- Item 2";
        let note_id = "note-123";
        let blocks = parse_markdown_to_blocks(content, note_id).unwrap();

        assert!(blocks.len() >= 2);
        assert_eq!(blocks[0].block_type, "list_item");
    }
}
