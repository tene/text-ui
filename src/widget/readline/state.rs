//! Command processor

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use unicode_segmentation::UnicodeSegmentation;

use super::history::{Direction, History};
use super::keymap::{Anchor, At, CharSearch, RepeatCount, Word};
use super::keymap::{InputState, Refresher};
use super::line_buffer::{LineBuffer, WordAction, MAX_LINE};
use super::undo::Changeset;
use super::Offset as Position;

/// Represent the state during line editing.
/// Implement rendering.
pub struct State {
    pub line: LineBuffer, // Edited line buffer
    pub cursor: Position, /* Cursor position (relative to the start of the prompt
                           * for `row`) */
    history_index: usize, // The history index we are currently editing
    saved_line_for_history: LineBuffer, // Current edited line before history browsing
    byte_buffer: [u8; 4],
    pub changes: Rc<RefCell<Changeset>>, // changes to line, for undo/redo
    pub width: usize,
    pub rows: usize,
}

impl State {
    pub fn new(history_index: usize) -> State {
        let capacity = MAX_LINE;
        State {
            line: LineBuffer::with_capacity(capacity),
            cursor: Default::default(),
            history_index,
            saved_line_for_history: LineBuffer::with_capacity(capacity),
            byte_buffer: [0; 4],
            changes: Rc::new(RefCell::new(Changeset::new())),
            width: 80,
            rows: 1,
        }
    }

    pub fn backup(&mut self) {
        self.saved_line_for_history
            .update(self.line.as_str(), self.line.pos());
    }
    pub fn restore(&mut self) {
        self.line.update(
            self.saved_line_for_history.as_str(),
            self.saved_line_for_history.pos(),
        );
    }

    pub fn refresh(&mut self) {
        let focuspos = self.line.pos();
        let mut focus = Position::new(0, 0);
        let mut idx = 0;
        let lines = self.render_width(self.width);
        self.rows = lines.len();
        'outer: for (row, line) in lines.into_iter().enumerate() {
            if idx == focuspos {
                focus = Position::new(0, row);
                break 'outer;
            }
            for (col, _ch) in UnicodeSegmentation::graphemes(line.as_str(), true).enumerate() {
                idx += 1;
                if idx == focuspos {
                    focus = Position::new(col + 1, row);
                    break 'outer;
                }
            }
            idx += 1;
        }
        self.cursor = focus;
    }

    pub fn render_width(&self, width: usize) -> Vec<String> {
        let lines: Vec<String> = self.line
            .as_str()
            .split('\n')
            .flat_map(|l| {
                let letters: Vec<&str> = UnicodeSegmentation::graphemes(l, true).collect();
                let mut split_lines = letters
                    .chunks(width)
                    .map(|ls| ls.concat())
                    .collect::<Vec<String>>();
                if split_lines.len() == 0 {
                    split_lines.push("".to_string());
                }
                split_lines.into_iter()
            })
            .collect();
        lines
    }

    pub fn finalize(&mut self) -> String {
        let rv = self.line.as_str().to_owned();
        self.line = LineBuffer::with_capacity(MAX_LINE);
        self.refresh();
        rv
    }
}

impl Refresher for State {
    fn refresh_line(&mut self) {
        self.refresh()
    }
    fn doing_insert(&mut self) {
        self.changes.borrow_mut().begin();
    }
    fn doing_replace(&mut self) {
        self.changes.borrow_mut().begin();
    }
    fn done_inserting(&mut self) {
        self.changes.borrow_mut().end();
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("State")
            .field("buf", &self.line)
            .field("cursor", &self.cursor)
            .field("rows", &self.rows)
            .field("width", &self.width)
            .field("history_index", &self.history_index)
            .field("saved_line_for_history", &self.saved_line_for_history)
            .finish()
    }
}

impl State {
    /// Insert the character `ch` at cursor current position.
    pub fn edit_insert(&mut self, ch: char, n: RepeatCount) {
        if let Some(_) = self.line.insert(ch, n) {
            self.refresh_line()
        };
    }

    /// Replace a single (or n) character(s) under the cursor (Vi mode)
    pub fn edit_replace_char(&mut self, ch: char, n: RepeatCount) {
        self.changes.borrow_mut().begin();
        let succeed = if let Some(chars) = self.line.delete(n) {
            let count = chars.graphemes(true).count();
            self.line.insert(ch, count);
            self.line.move_backward(1);
            true
        } else {
            false
        };
        self.changes.borrow_mut().end();
        if succeed {
            self.refresh_line()
        }
    }

    /// Overwrite the character under the cursor (Vi mode)
    pub fn edit_overwrite_char(&mut self, ch: char) {
        if let Some(end) = self.line.next_pos(1) {
            {
                let text = ch.encode_utf8(&mut self.byte_buffer);
                let start = self.line.pos();
                self.line.replace(start..end, text);
            }
            self.refresh_line()
        }
    }

    // Yank/paste `text` at current position.
    pub fn edit_yank(
        &mut self,
        input_state: &InputState,
        text: &str,
        anchor: Anchor,
        n: RepeatCount,
    ) {
        if let Anchor::After = anchor {
            self.line.move_forward(1);
        }
        if self.line.yank(text, n).is_some() {
            if !input_state.is_emacs_mode() {
                self.line.move_backward(1);
            }
            self.refresh_line()
        }
    }

    // Delete previously yanked text and yank/paste `text` at current position.
    pub fn edit_yank_pop(&mut self, yank_size: usize, text: &str) {
        self.changes.borrow_mut().begin();
        if self.line.yank_pop(yank_size, text).is_some() {
            self.refresh_line()
        };
        self.changes.borrow_mut().end();
    }

    /// Move cursor on the left.
    pub fn edit_move_backward(&mut self, n: RepeatCount) {
        if self.line.move_backward(n) {
            self.refresh()
        }
    }

    /// Move cursor on the right.
    pub fn edit_move_forward(&mut self, n: RepeatCount) {
        if self.line.move_forward(n) {
            self.refresh()
        }
    }

    /// Move cursor to the start of the line.
    pub fn edit_move_home(&mut self) {
        if self.line.move_home() {
            self.refresh()
        }
    }

    /// Move cursor to the end of the line.
    pub fn edit_move_end(&mut self) {
        if self.line.move_end() {
            self.refresh()
        }
    }

    /// Delete the character at the right of the cursor without altering the
    /// cursor position. Basically this is what happens with the "Delete"
    /// keyboard key.
    pub fn edit_delete(&mut self, n: RepeatCount) {
        if self.line.delete(n).is_some() {
            self.refresh_line()
        }
    }

    /// Backspace implementation.
    pub fn edit_backspace(&mut self, n: RepeatCount) {
        if self.line.backspace(n) {
            self.refresh_line()
        }
    }

    /// Kill the text from point to the end of the line.
    pub fn edit_kill_line(&mut self) {
        if self.line.kill_line() {
            self.refresh_line()
        }
    }

    /// Kill backward from point to the beginning of the line.
    pub fn edit_discard_line(&mut self) {
        if self.line.discard_line() {
            self.refresh_line()
        }
    }

    /// Exchange the char before cursor with the character at cursor.
    pub fn edit_transpose_chars(&mut self) {
        self.changes.borrow_mut().begin();
        let succeed = self.line.transpose_chars();
        self.changes.borrow_mut().end();
        if succeed {
            self.refresh_line()
        }
    }

    pub fn edit_move_to_prev_word(&mut self, word_def: Word, n: RepeatCount) {
        if self.line.move_to_prev_word(word_def, n) {
            self.refresh()
        }
    }

    /// Delete the previous word, maintaining the cursor at the start of the
    /// current word.
    pub fn edit_delete_prev_word(&mut self, word_def: Word, n: RepeatCount) {
        if self.line.delete_prev_word(word_def, n) {
            self.refresh_line()
        }
    }

    pub fn edit_move_to_next_word(&mut self, at: At, word_def: Word, n: RepeatCount) {
        if self.line.move_to_next_word(at, word_def, n) {
            self.refresh()
        }
    }

    pub fn edit_move_to(&mut self, cs: CharSearch, n: RepeatCount) {
        if self.line.move_to(cs, n) {
            self.refresh()
        }
    }

    /// Kill from the cursor to the end of the current word, or, if between
    /// words, to the end of the next word.
    pub fn edit_delete_word(&mut self, at: At, word_def: Word, n: RepeatCount) {
        if self.line.delete_word(at, word_def, n) {
            self.refresh_line()
        }
    }

    pub fn edit_delete_to(&mut self, cs: CharSearch, n: RepeatCount) {
        if self.line.delete_to(cs, n) {
            self.refresh_line()
        }
    }

    pub fn edit_word(&mut self, a: WordAction) {
        self.changes.borrow_mut().begin();
        let succeed = self.line.edit_word(a);
        self.changes.borrow_mut().end();
        if succeed {
            self.refresh_line()
        }
    }

    pub fn edit_transpose_words(&mut self, n: RepeatCount) {
        self.changes.borrow_mut().begin();
        let succeed = self.line.transpose_words(n);
        self.changes.borrow_mut().end();
        if succeed {
            self.refresh_line()
        }
    }

    /// Substitute the currently edited line with the next or previous history
    /// entry.
    pub fn edit_history_next(&mut self, history: &History, prev: bool) {
        if history.is_empty() {
            return;
        }
        if self.history_index == history.len() {
            if prev {
                // Save the current edited line before overwriting it
                self.backup();
            } else {
                return;
            }
        } else if self.history_index == 0 && prev {
            return;
        }
        if prev {
            self.history_index -= 1;
        } else {
            self.history_index += 1;
        }
        if self.history_index < history.len() {
            let buf = history.get(self.history_index).unwrap();
            self.changes.borrow_mut().begin();
            self.line.update(buf, buf.len());
            self.changes.borrow_mut().end();
        } else {
            // Restore current edited line
            self.restore();
        }
        self.refresh_line()
    }

    // Non-incremental, anchored search
    pub fn edit_history_search(&mut self, history: &History, dir: Direction) {
        if history.is_empty() {
            return;
        }
        if self.history_index == history.len() && dir == Direction::Forward {
            return;
        } else if self.history_index == 0 && dir == Direction::Reverse {
            return;
        }
        if dir == Direction::Reverse {
            self.history_index -= 1;
        } else {
            self.history_index += 1;
        }
        if let Some(history_index) = history.starts_with(
            &self.line.as_str()[..self.line.pos()],
            self.history_index,
            dir,
        ) {
            self.history_index = history_index;
            let buf = history.get(history_index).unwrap();
            self.changes.borrow_mut().begin();
            self.line.update(buf, buf.len());
            self.changes.borrow_mut().end();
            self.refresh_line()
        }
    }

    /// Substitute the currently edited line with the first/last history entry.
    pub fn edit_history(&mut self, history: &History, first: bool) {
        if history.is_empty() {
            return;
        }
        if self.history_index == history.len() {
            if first {
                // Save the current edited line before overwriting it
                self.backup();
            } else {
                return;
            }
        } else if self.history_index == 0 && first {
            return;
        }
        if first {
            self.history_index = 0;
            let buf = history.get(self.history_index).unwrap();
            self.changes.borrow_mut().begin();
            self.line.update(buf, buf.len());
            self.changes.borrow_mut().end();
        } else {
            self.history_index = history.len();
            // Restore current edited line
            self.restore();
        }
        self.refresh_line()
    }
}

#[cfg(test)]
pub fn init_state(out: &'out mut Renderer, line: &str, pos: usize) -> State<'out, 'static> {
    State {
        out,
        prompt: "",
        prompt_size: Position::default(),
        line: LineBuffer::init(line, pos, None),
        cursor: Position::default(),
        history_index: 0,
        saved_line_for_history: LineBuffer::with_capacity(100),
        byte_buffer: [0; 4],
        changes: Rc::new(RefCell::new(Changeset::new())),
        hinter: None,
    }
}

#[cfg(test)]
mod test {
    use super::init_state;
    use history::History;

    #[test]
    fn edit_history_next() {
        let mut out = ::std::io::sink();
        let line = "current edited line";
        let mut s = init_state(&mut out, line, 6);
        let mut history = History::new();
        history.add("line0");
        history.add("line1");
        s.history_index = history.len();

        for _ in 0..2 {
            s.edit_history_next(&history, false).unwrap();
            assert_eq!(line, s.line.as_str());
        }

        s.edit_history_next(&history, true).unwrap();
        assert_eq!(line, s.saved_line_for_history.as_str());
        assert_eq!(1, s.history_index);
        assert_eq!("line1", s.line.as_str());

        for _ in 0..2 {
            s.edit_history_next(&history, true).unwrap();
            assert_eq!(line, s.saved_line_for_history.as_str());
            assert_eq!(0, s.history_index);
            assert_eq!("line0", s.line.as_str());
        }

        s.edit_history_next(&history, false).unwrap();
        assert_eq!(line, s.saved_line_for_history.as_str());
        assert_eq!(1, s.history_index);
        assert_eq!("line1", s.line.as_str());

        s.edit_history_next(&history, false).unwrap();
        // assert_eq!(line, s.saved_line_for_history);
        assert_eq!(2, s.history_index);
        assert_eq!(line, s.line.as_str());
    }
}
