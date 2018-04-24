use termion::event::{Key};
use std::cmp::{min,max};
use widget::{Widget};
use ::{Size, Position};

pub struct Input {
    buf: String,
    index: usize,
}

impl Widget for Input {
    fn render_content(&self, _size: Size) -> Option<Vec<String>> {
        Some(vec!(self.buf.clone()))
    }
    fn render_focus(&self, _size: Size) -> Option<Position> { Some(Position::new(self.index as u16, 0)) }
}

impl Input {
    pub fn new(s: &str) -> Input {
        Input {
            buf: s.to_owned(),
            index: 0,
        }
    }
    pub fn submit(&mut self) -> String {
        self.index = 0;
        self.buf.split_off(0)
    }
    pub fn keypress(&mut self, key: Key) {
        match key {
            Key::Char(k) => {
                self.buf.insert(self.index, k);
                self.index += 1;
            },
            Key::Left => self.index = max(self.index-1, 0),
            Key::Right => self.index = min(self.buf.len(), self.index+1),
            _ => {},
        }
    }
}
