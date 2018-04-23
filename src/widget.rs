use termion::event::{Key};
use std::cmp::{min,max};
use pane::{Pane,Render};

pub struct Input {
    buf: String,
    index: usize,
}

impl Render for Input {
    fn render_content(&self, _width: u16, _height: u16) -> (Vec<String>, Option<(u16,u16)>) {
        (vec!(self.buf.clone()), Some((self.index as u16, 0)))
    }
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
            Key::Right => self.index = min(self.buf.len()-1, self.index+1),
            _ => {},
        }
    }
}

pub struct Text {
    lines: Vec<String>,
}

impl Render for Text {
    fn render_content(&self, _width: u16, height: u16) -> (Vec<String>, Option<(u16,u16)>) {
        let loglen = self.lines.len();
        let lines = if loglen > height as usize {
            self.lines.clone().split_off(loglen - height as usize)
        } else {
            self.lines.clone()
        };
        (lines, None)
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
