use unicode_segmentation::UnicodeSegmentation;
use widget::Widget;
use Size;

#[derive(Debug, PartialEq, Clone)]
pub struct Text {
    pub lines: Vec<String>,
    pub style: Option<String>,
}

impl Widget for Text {
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        let lines: Vec<String> = self.lines
            .iter()
            .flat_map(|l| l.lines())
            .flat_map(|l| {
                let letters: Vec<&str> = UnicodeSegmentation::graphemes(l, true).collect();
                letters
                    .chunks(size.width)
                    .map(|ls| ls.concat())
                    .collect::<Vec<String>>()
                    .into_iter()
            })
            .collect();
        let loglen = lines.len();
        let lines = if loglen > size.height {
            lines.clone().split_off(loglen - size.height)
        } else {
            lines.clone()
        };
        Some(lines)
    }
    fn render_style(&self) -> Option<String> {
        self.style.clone()
    }
}

impl Text {
    pub fn new(lines: Vec<String>) -> Text {
        Text { lines, style: None }
    }
    pub fn new_styled(lines: Vec<String>, s: &str) -> Text {
        let style = Some(s.to_owned());
        Text { lines, style }
    }
    pub fn push(&mut self, s: String) {
        self.lines.extend(s.lines().map(|l| l.to_owned()));
    }
    pub fn set(&mut self, s: &str) {
        self.lines = s.lines().map(|l| l.to_owned()).collect();
    }
}
