use termion::event::{Key};
use std::cmp::{min,max};
use pane::{Widget};

pub struct Input {
    buf: String,
    index: usize,
}

impl Widget for Input {
    fn render_content(&self, _width: u16, _height: u16) -> Option<Vec<String>> {
        Some(vec!(self.buf.clone()))
    }
    fn render_focus(&self, _width: u16, _height: u16) -> Option<(u16,u16)> { Some((self.index as u16, 0)) }
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

pub struct Text {
    lines: Vec<String>,
}

impl Widget for Text {
    fn render_content(&self, _width: u16, height: u16) -> Option<Vec<String>> {
        let loglen = self.lines.len();
        let lines = if loglen > height as usize {
            self.lines.clone().split_off(loglen - height as usize)
        } else {
            self.lines.clone()
        };
        Some(lines)
    }
}

impl Text {
    pub fn new(l: Vec<String>) -> Text {
        Text {
            lines: l,
        }
    }
    pub fn push(&mut self, s: String) {
        self.lines.push(s);
    }
}
