use widget::Widget;
use Size;

#[derive(Debug, PartialEq, Clone)]
pub struct Text {
    pub lines: Vec<String>,
}

impl Widget for Text {
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        let loglen = self.lines.len();
        let lines = if loglen > size.height as usize {
            self.lines.clone().split_off(loglen - size.height as usize)
        } else {
            self.lines.clone()
        };
        Some(lines)
    }
}

impl Text {
    pub fn new(l: Vec<String>) -> Text {
        Text { lines: l }
    }
    pub fn push(&mut self, s: String) {
        self.lines.push(s);
    }
    pub fn set(&mut self, s: &str) {
        self.lines = s.lines().map(|l| l.to_owned()).collect();
    }
}
