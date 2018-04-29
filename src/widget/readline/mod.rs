use termion::event::Key;
use widget::{Bound, BoundSize, Widget};
use {Position, Size};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod config;
mod consts;
mod state;
mod edit;
mod history;
mod keymap;
mod kill_ring;
mod line_buffer;
mod process;
mod undo;

pub use self::state::State;
pub use self::edit::Editor;
use self::keymap::{InputState,Cmd};
use self::consts::KeyPress;
use self::config::Config;
use self::process::{process_char, process_command};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Offset {
    pub col: usize,
    pub row: usize,
}

impl Offset {
    pub fn new(col: usize, row: usize) -> Self {
        Offset { col, row }
    }
}

pub struct Readline {
    pub state: State,
    pub editor: Editor,
    pub input_state: InputState,
}

impl Readline {
    pub fn new() -> Self {
        let mut state = State::new(0);
        let editor = Editor::new();
        let input_state = InputState::new(&editor.config, editor.custom_bindings.clone());
        state.line.set_delete_listener(editor.kill_ring.clone());
        state.line.set_change_listener(state.changes.clone());
        Readline {
            state,
            editor,
            input_state,
        }
    }
    pub fn update(&mut self, buf: &str, pos: usize) {
        self.state.line.update(buf, pos);
        self.state.refresh();
    }
    pub fn width(&mut self, width: usize) {
        self.state.width = width;
        self.state.refresh();
    }

    pub fn process_char(&mut self, ch: char) {
        process_char(&mut self.state, &mut self.editor, ch, &mut self.input_state);
    }

    pub fn process_keypress(&mut self, kp: KeyPress) {
        let cmd = self.input_state.next_cmd(kp, &mut self.state, true);
        process_command(&mut self.state, &mut self.editor, cmd, &mut self.input_state);
    }

    pub fn finalize(&mut self) -> String {
        self.state.finalize()
    }
}

// text-ui integration to be broken out later
fn key_to_keypress(key: Key) -> KeyPress {
    match key {
        Key::Backspace => KeyPress::Backspace,
        Key::Left => KeyPress::Left,
        Key::Right => KeyPress::Right,
        Key::Up => KeyPress::Up,
        Key::Down => KeyPress::Down,
        Key::Home => KeyPress::Home,
        Key::End => KeyPress::End,
        Key::PageUp => KeyPress::PageUp,
        Key::PageDown => KeyPress::PageDown,
        Key::Delete => KeyPress::Delete,
        Key::Insert => KeyPress::Insert,
        Key::F(u8) => KeyPress::F(u8),
        Key::Char(char) => KeyPress::Char(char),
        Key::Alt(char) => KeyPress::Meta(char),
        Key::Ctrl(char) => KeyPress::Ctrl(char),
        Key::Null => KeyPress::Null,
        Key::Esc => KeyPress::Esc,
        _ => KeyPress::UnknownEscSeq,
    }
}

impl Readline {
    pub fn process_key(&mut self, key: Key) {
        self.process_keypress(key_to_keypress(key))
    }
}

impl Widget for Readline {
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        /*if self.state.width != size.width as usize {
            self.state.width = size.width as usize;
        }*/
        Some(self.state.render_width(size.width as usize))
    }
    fn render_bounds(&self) -> BoundSize {
        BoundSize {
            width: Bound::Fixed(self.state.width as u16),
            height: Bound::Fixed(self.state.rows as u16),
        }
    }
    fn render_focus(&self, _size: Size) -> Option<Position> {
        let pos = self.state.cursor;
        Some(Position::new(pos.col as u16, pos.row as u16))
    }
}
