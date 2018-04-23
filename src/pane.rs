use std::io::{Write};

pub struct Pane {
    pub x: u16,
    pub y: u16,
    pub content: Vec<String>,
}

impl Pane {
    pub fn new(x: u16, y: u16, content: Vec<String>) -> Pane {
        Pane {
            x: x,
            y: y,
            content: content,
        }
    }
}

