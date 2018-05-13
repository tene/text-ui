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

    pub fn new_width(width: usize) -> Pane {
        Pane {
            position: Position::new(0, 0),
            size: Size::new(width as u16, 1),
            content: None,
            focus: None,
            children: None,
            style: None,
        }
    }

    pub fn new_styled(
        position: Position,
        size: Size,
        content: Vec<String>,
        stylename: &str,
    ) -> Pane {
        Pane {
            position,
            size,
            content: Some(content),
            focus: None,
            children: None,
            style: Some(stylename.to_string()),
        }
    }

    pub fn new_children(
        position: Position,
        size: Size,
        content: Vec<String>,
        children: Vec<Pane>,
    ) -> Pane {
        let children = if children.len() > 0 {
            Some(children)
        } else {
            None
        };
        Pane {
            position,
            size,
            content: Some(content),
            focus: None,
            children,
            style: None,
        }
    }

    pub fn offset(mut self, pos: Position) -> Self {
        self.position = self.position + pos;
        self
    }

    pub fn push_child(&mut self, child: Pane) {
        let mut children = match self.children.take() {
            Some(ch) => ch,
            None => vec![],
        };
        self.size.height = max(child.position.y + child.size.height, self.size.height);
        self.size.width = max(child.position.x + child.size.width, self.size.width);
        children.push(child);
        self.children = Some(children);
    }

    pub fn set_style(&mut self, style: &str) {
        self.style = Some(style.to_owned());
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
