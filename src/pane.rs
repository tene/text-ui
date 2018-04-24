// TODO definitely need a Position type
pub struct Pane {
    pub x: u16,
    pub y: u16,
    pub content: Option<Vec<String>>,
    pub focus: Option<(u16,u16)>,
    pub children: Option<Vec<Pane>>,
}

impl Pane {
    pub fn new(x: u16, y: u16, content: Vec<String>) -> Pane {
        Pane {
            x: x,
            y: y,
            content: Some(content),
            focus: None,
            children: None,
        }
    }
}

pub trait Widget {
    fn render(&self, x: u16, y: u16, width: u16, height: u16) -> Pane {
        let content = self.render_content(width, height);
        let focus = self.render_focus(width, height);
        let children = self.render_children(width,height);
        Pane {
            x: x,
            y: y,
            content: content,
            focus: focus,
            children: children,
        }
    }
    fn render_content(&self, u16, u16) -> Option<Vec<String>> { None }
    fn render_focus(&self, u16, u16) -> Option<(u16,u16)> { None }
    fn render_children(&self, u16, u16) -> Option<Vec<Pane>> { None }
}