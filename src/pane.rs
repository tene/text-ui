pub struct Pane {
    pub x: u16,
    pub y: u16,
    pub content: Vec<String>,
    pub focus: Option<(u16,u16)>,
}

impl Pane {
    pub fn new(x: u16, y: u16, content: Vec<String>) -> Pane {
        Pane {
            x: x,
            y: y,
            content: content,
            focus: None,
        }
    }
}

pub trait Render {
    fn render(&self, x: u16, y: u16, width: u16, height: u16) -> Pane {
        let (content, focus) = self.render_content(width, height);
        Pane {
            x: x,
            y: y,
            content: content,
            focus: focus,
        }
    }
    fn render_content(&self, u16, u16) -> (Vec<String>, Option<(u16,u16)>);
}