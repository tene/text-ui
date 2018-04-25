pub mod input;
pub mod text;
pub mod vbox;

pub use self::input::Input;
pub use self::text::Text;
pub use self::vbox::VBox;

use pane::Pane;
use {Position, Size};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bound {
    Fixed(u16),
    AtLeast(u16),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoundSize {
    pub width: Bound,
    pub height: Bound,
}

pub trait Widget {
    fn render(&self, pos: Position, size: Size) -> Pane {
        let content = self.render_content(size);
        let focus = self.render_focus(size).map(|f| f + pos);
        let children = self.render_children(size);
        Pane {
            position: pos,
            content: content,
            focus: focus,
            children: children,
        }
    }
    fn render_content(&self, Size) -> Option<Vec<String>> {
        None
    }
    // TODO lots of duplicate work with focus separate here D:
    fn render_focus(&self, Size) -> Option<Position> {
        None
    }
    fn render_children(&self, Size) -> Option<Vec<Pane>> {
        None
    }
    fn render_bounds(&self) -> BoundSize {
        BoundSize {
            width: Bound::AtLeast(1),
            height: Bound::AtLeast(1),
        }
    }
}
