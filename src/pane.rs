use std::cmp::max;

use Position;
use Size;

#[derive(Debug, PartialEq, Clone)]
pub struct Pane {
    pub position: Position,
    pub size: Size,
    pub content: Option<Vec<String>>,
    pub focus: Option<Position>,
    pub children: Option<Vec<Pane>>,
    pub style: Option<String>,
}

impl Pane {
    pub fn new(position: Position, size: Size, content: Vec<String>) -> Pane {
        Pane {
            position,
            size,
            content: Some(content),
            focus: None,
            children: None,
            style: None,
        }
    }

    pub fn offset(mut self, pos: Position) -> Self {
        self.focus = self.focus.map(|f| f + pos);
        self.position = self.position + pos;
        self
    }

    pub fn push_child(&mut self, child: Pane) {
        let mut children = match self.children.take() {
            Some(ch) => ch,
            None => vec![],
        };
        children.push(child);
        self.children = Some(children);
    }

    /*pub fn height(&self) -> usize {
        let mut rv = match &self.content {
            Some(content) => content.len(),
            None => 0,
        };
        if let Some(children) = &self.children {
            rv = children
                .iter()
                .map(|c| c.height() + c.position.y as usize)
                .fold(rv, max);
        }
        rv
    }

    pub fn width(&self) -> usize {
        let mut rv = match &self.content {
            Some(content) => content.iter().map(|l| l.len()).fold(0, max),
            None => 0,
        };
        if let Some(children) = &self.children {
            rv = children
                .iter()
                .map(|c| c.width() + c.position.x as usize)
                .fold(rv, max);
        }
        rv
    }*/
}
