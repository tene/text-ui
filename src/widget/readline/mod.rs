use termion::event::Key;
use widget::{Bound, BoundSize, Widget};
use {Position, Size};

mod config;
mod consts;
mod edit;
mod history;
mod keymap;
mod line_buffer;
mod undo;

use self::edit::State;

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
    pub state: edit::State,
}

impl Readline {
    pub fn new() -> Self {
        Readline {
            state: State::new(0),
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
}
