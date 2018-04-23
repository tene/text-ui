use termion::event::{Key};

pub struct Input {
    buf: String,
}

impl Input {
    pub fn render(&self, width: u16, height: u16) -> Vec<String> {
        vec!(self.buf.clone())
    }
    pub fn new(s: &str) -> Input {
        Input {
            buf: s.to_owned(),
        }
    }
    pub fn submit(&mut self) -> String {
        self.buf.split_off(0)
    }
    pub fn keypress(&mut self, key: Key) {
        match key {
            Key::Char(k) => self.buf.push(k),
            _ => {},
        }
    }
}

pub struct Text {
    lines: Vec<String>,
}

impl Text {
    pub fn render(&self, width: u16, height: u16) -> Vec<String> {
        let loglen = self.lines.len();
        let lines = if loglen > height as usize {
            self.lines.clone().split_off(loglen - height as usize)
        } else {
            self.lines.clone()
        };
        lines
    }
    pub fn new(l: Vec<String>) -> Text {
        Text {
            lines: l,
        }
    }
    pub fn push(&mut self, s: String) {
        self.lines.push(s);
    }
}
