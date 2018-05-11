use Position;

#[derive(Debug, PartialEq, Clone)]
pub struct Pane {
    pub position: Position,
    pub content: Option<Vec<String>>,
    pub focus: Option<Position>,
    pub children: Option<Vec<Pane>>,
}

impl Pane {
    pub fn new(pos: Position, content: Vec<String>) -> Pane {
        Pane {
            position: pos,
            content: Some(content),
            focus: None,
            children: None,
        }
    }

    pub fn offset(self, pos: Position) -> Self {
        Pane {
            position: self.position + pos,
            content: self.content,
            focus: self.focus.map(|f| f + pos),
            children: self.children,
        }
    }

    pub fn push_child(&mut self, child: Pane) {
        let mut children = match self.children.take() {
            Some(ch) => ch,
            None => vec![],
        };
        children.push(child);
        self.children = Some(children);
    }
}
