use ::{Size, Position};

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
}

pub trait Widget {
    fn render(&self, pos: Position, size: Size) -> Pane {
        let content = self.render_content(size);
        let focus = self.render_focus(size);
        let children = self.render_children(size);
        Pane {
            position: pos,
            content: content,
            focus: focus,
            children: children,
        }
    }
    fn render_content(&self, Size) -> Option<Vec<String>> { None }
    fn render_focus(&self, Size) -> Option<Position> { None }
    fn render_children(&self, Size) -> Option<Vec<Pane>> { None }
}