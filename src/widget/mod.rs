pub mod input;
pub mod text;
pub mod vbox;

pub use self::input::Input;
pub use self::text::Text;
pub use self::vbox::VBox;

use pane::Pane;
use {Position, Size};

use std::sync::Arc;
use std::sync::RwLock;

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

impl<T> Widget for Arc<T>
where
    T: Widget,
{
    fn render(&self, pos: Position, size: Size) -> Pane {
        (**self).render(pos, size)
    }
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        (**self).render_content(size)
    }
    fn render_focus(&self, size: Size) -> Option<Position> {
        (**self).render_focus(size)
    }
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        (**self).render_children(size)
    }
    fn render_bounds(&self) -> BoundSize {
        (**self).render_bounds()
    }
}

impl<T> Widget for Box<T>
where
    T: Widget,
{
    fn render(&self, pos: Position, size: Size) -> Pane {
        (**self).render(pos, size)
    }
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        (**self).render_content(size)
    }
    fn render_focus(&self, size: Size) -> Option<Position> {
        (**self).render_focus(size)
    }
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        (**self).render_children(size)
    }
    fn render_bounds(&self) -> BoundSize {
        (**self).render_bounds()
    }
}

impl<T> Widget for RwLock<T>
where
    T: Widget,
{
    fn render(&self, pos: Position, size: Size) -> Pane {
        self.read().unwrap().render(pos, size)
    }
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        self.read().unwrap().render_content(size)
    }
    fn render_focus(&self, size: Size) -> Option<Position> {
        self.read().unwrap().render_focus(size)
    }
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        self.read().unwrap().render_children(size)
    }
    fn render_bounds(&self) -> BoundSize {
        self.read().unwrap().render_bounds()
    }
}
