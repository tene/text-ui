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

    pub fn check_intersect(&self, pos: Position, size: Size) -> bool {
        intersect_boxes(self.position, self.size, pos, size).is_some()
    }

    pub fn clip(mut self, clip_pos: Position, clip_size: Size) -> Option<Pane> {
        let (clip_pos, clip_size) =
            match intersect_boxes(self.position, self.size, clip_pos, clip_size) {
                None => return None,
                Some(rect) => rect,
            };
        if !self.check_intersect(clip_pos, clip_size) {
            return None;
        }
        self.content = match self.content.take() {
            None => None,
            Some(mut content) => {
                let clip_pos = clip_pos - self.position;
                let content = content
                    .into_iter()
                    .skip(clip_pos.y as usize)
                    .take(clip_size.height as usize)
                    .map(|l| {
                        l.chars()
                            .skip(clip_pos.x as usize)
                            .take(clip_size.width as usize)
                            .collect()
                    })
                    .collect();
                Some(content)
            }
        };
        self.children = match self.children.take() {
            None => None,
            Some(mut children) => {
                let clip_pos = clip_pos - self.position;
                let mut children: Vec<Pane> = children
                    .into_iter()
                    .filter_map(|child| child.clip(clip_pos, clip_size))
                    .collect();
                for child in children.iter_mut() {
                    child.position = child.position - clip_pos;
                }
                Some(children)
            }
        };
        self.position = clip_pos;
        self.size = clip_size;
        Some(self)
    }

    /*pub fn normalize(&mut self) {
        match (&mut self.children, &self.clip) {
            (Some(ref mut children), Some((clip_pos, clip_size))) => {
                children.retain(|child| child.check_intersect(*clip_pos, *clip_size));
                for child in children.iter_mut() {
                    let (mut new_clip_pos, mut new_clip_size) =
                        intersect_boxes(*clip_pos, *clip_size, child.position, child.size)
                            .expect("Non-overlapping child pane in pre-filtered list");
                    if let Some((_child_clip_pos, _child_clip_size)) = child.clip.take() {
                        unimplemented!()
                    }
                    child.clip = Some((new_clip_pos, new_clip_size));
                    child.normalize();
                }
            }
            (Some(ref mut children), None) => {
                for child in children.iter_mut() {
                    child.normalize();
                }
            }
            _ => {}
        }
    }*/

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

fn intersect_boxes(
    a_pos: Position,
    a_size: Size,
    b_pos: Position,
    b_size: Size,
) -> Option<(Position, Size)> {
    use std::cmp::{max, min};
    if a_pos.x + a_size.width < b_pos.x || b_pos.x + b_size.width < a_pos.x {
        return None;
    };
    if a_pos.y + a_size.height < b_pos.y || b_pos.y + b_size.height < a_pos.y {
        return None;
    };
    let x = max(a_pos.x, b_pos.x);
    let y = max(a_pos.y, b_pos.y);
    let width = min(a_pos.x + a_size.width, b_pos.x + b_size.width) - x;
    let height = min(a_pos.y + a_size.height, b_pos.y + b_size.height) - y;
    Some((Position::new(x, y), Size::new(width, height)))
}
