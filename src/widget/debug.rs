use widget::{Bound, BoundSize, Shared, Widget};
use {Position, Size};

use std::fmt::Debug;
use std::str;

pub struct DbgDump<T: Debug> {
    object: Shared<T>,
}

impl<T: Debug> DbgDump<T> {
    pub fn new(object: &Shared<T>) -> Self {
        Self {
            object: object.clone(),
        }
    }
}

impl<T: Debug> Widget for DbgDump<T> {
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        let lines = format!("{:#?}", self.object.read())
            .lines()
            .flat_map(|l| l.as_bytes().chunks(size.width as usize))
            .map(|l| str::from_utf8(l).unwrap().to_owned())
            .collect();
        Some(lines)
    }
}
