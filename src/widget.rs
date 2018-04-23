use ::pane::Pane;

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
}
