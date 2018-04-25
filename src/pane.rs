use Position;

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
