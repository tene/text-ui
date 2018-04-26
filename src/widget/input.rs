use std::cmp::min;
use termion::event::Key;
use widget::{Bound, BoundSize, Widget};
use {Position, Size};

#[derive(Debug, PartialEq, Clone)]
pub struct TextInput {
    buf: String,
    index: usize,
}

impl Widget for TextInput {
    fn render_content(&self, _size: Size) -> Option<Vec<String>> {
        Some(vec![self.buf.clone()])
    }
    fn render_focus(&self, _size: Size) -> Option<Position> {
        Some(Position::new(self.index as u16, 0))
    }
    fn render_bounds(&self) -> BoundSize {
        BoundSize {
            width: Bound::AtLeast(1),
            height: Bound::Fixed(1),
        }
    }
}

impl TextInput {
    pub fn new(s: &str) -> TextInput {
        TextInput {
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
            }
            Key::Left => if self.index > 0 {
                self.index -= 1
            },
            Key::Right => self.index = min(self.buf.len(), self.index + 1),
            _ => {}
        }
    }
}
