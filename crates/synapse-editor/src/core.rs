//! Editor core: buffer, cursor, and commands (minimal stub).
//!
//! Index and selection use grapheme clusters. See interface doc for full API.

use std::ops::Range;

use synapse_core::Result;

/// Editor core: buffer + cursor + basic edit/undo.
/// Full API per docs/02-架构设计/接口定义-core-editor.md.
pub struct EditorCore {
    buffer: String,
    cursor: usize,
    /// (anchor, head); no selection when anchor == head
    selection: Option<(usize, usize)>,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
}

impl EditorCore {
    pub fn new(initial_content: impl Into<String>) -> Self {
        let buffer = initial_content.into();
        let len = buffer.chars().count();
        Self {
            buffer: buffer.clone(),
            cursor: len.min(len),
            selection: None,
            undo_stack: vec![buffer],
            redo_stack: Vec::new(),
        }
    }

    pub fn buffer_content(&self) -> String {
        self.buffer.clone()
    }

    pub fn line_count(&self) -> usize {
        self.buffer.lines().count().max(1)
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor
    }

    pub fn set_cursor(&mut self, index: usize) -> Result<()> {
        let len = self.buffer.chars().count();
        self.cursor = index.min(len);
        Ok(())
    }

    pub fn selection(&self) -> Option<(usize, usize)> {
        self.selection
    }

    pub fn set_selection(&mut self, anchor: usize, head: usize) -> Result<()> {
        let len = self.buffer.chars().count();
        self.selection = Some((anchor.min(len), head.min(len)));
        Ok(())
    }

    pub fn insert_at_cursor(&mut self, text: &str) -> Result<()> {
        let byte_pos = self.char_offset_to_byte(self.cursor);
        self.buffer.insert_str(byte_pos, text);
        self.cursor += text.chars().count();
        self.undo_stack.push(self.buffer.clone());
        self.redo_stack.clear();
        Ok(())
    }

    pub fn delete_backward(&mut self, n: usize) -> Result<usize> {
        let byte_pos = self.char_offset_to_byte(self.cursor);
        let start = (0..self.buffer.len())
            .rev()
            .filter(|&i| self.buffer.is_char_boundary(i))
            .nth(n)
            .unwrap_or(0);
        let removed = if start >= byte_pos {
            0
        } else {
            self.buffer[start..byte_pos].chars().count()
        };
        if removed > 0 {
            self.buffer.drain(start..byte_pos);
            self.cursor = self.cursor.saturating_sub(removed);
            self.undo_stack.push(self.buffer.clone());
            self.redo_stack.clear();
        }
        Ok(removed)
    }

    pub fn delete_forward(&mut self, n: usize) -> Result<usize> {
        let byte_pos = self.char_offset_to_byte(self.cursor);
        let end_byte = (byte_pos..=self.buffer.len())
            .filter(|&i| self.buffer.is_char_boundary(i))
            .nth(n)
            .unwrap_or(self.buffer.len());
        let removed = self.buffer[byte_pos..end_byte].chars().count();
        if removed > 0 {
            self.buffer.drain(byte_pos..end_byte);
            self.undo_stack.push(self.buffer.clone());
            self.redo_stack.clear();
        }
        Ok(removed)
    }

    pub fn undo(&mut self) -> Result<bool> {
        if self.undo_stack.len() <= 1 {
            return Ok(false);
        }
        self.redo_stack.push(self.undo_stack.pop().unwrap());
        self.buffer = self.undo_stack.last().cloned().unwrap_or_default();
        let len = self.buffer.chars().count();
        self.cursor = self.cursor.min(len);
        Ok(true)
    }

    pub fn redo(&mut self) -> Result<bool> {
        let Some(prev) = self.redo_stack.pop() else {
            return Ok(false);
        };
        self.undo_stack.push(self.buffer.clone());
        self.buffer = prev;
        let len = self.buffer.chars().count();
        self.cursor = self.cursor.min(len);
        Ok(true)
    }

    pub fn can_undo(&self) -> bool {
        self.undo_stack.len() > 1
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn cursor_line_index(&self) -> usize {
        self.buffer
            .chars()
            .take(self.cursor)
            .filter(|&c| c == '\n')
            .count()
    }

    pub fn line_range(&self, line_index: usize) -> Option<Range<usize>> {
        let mut start = 0usize;
        let mut idx = 0usize;
        let char_count = self.buffer.chars().count();
        for (i, c) in self.buffer.chars().enumerate() {
            if idx == line_index {
                let end = self
                    .buffer
                    .chars()
                    .skip(i)
                    .position(|ch| ch == '\n')
                    .map(|j| i + j)
                    .unwrap_or(char_count);
                return Some(start..end);
            }
            if c == '\n' {
                start = i + 1;
                idx += 1;
            }
        }
        if idx == line_index {
            Some(start..char_count)
        } else {
            None
        }
    }

    pub fn line_content(&self, line_index: usize) -> Option<String> {
        self.line_range(line_index).map(|r| {
            self.buffer
                .chars()
                .skip(r.start)
                .take(r.end - r.start)
                .collect()
        })
    }

    fn char_offset_to_byte(&self, char_offset: usize) -> usize {
        self.buffer
            .chars()
            .take(char_offset)
            .map(char::len_utf8)
            .sum()
    }
}
