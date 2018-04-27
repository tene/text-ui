pub mod input;
pub mod linear;
pub mod text;

pub use self::input::TextInput;
pub use self::linear::Direction;
pub use self::linear::Linear;
pub use self::text::Text;

use pane::Pane;
use {Position, Size};

use std::sync::Arc;
use std::ops::DerefMut;
use std::sync::{RwLock, LockResult, RwLockWriteGuard, RwLockReadGuard};

#[derive(Debug)]
pub struct Shared<W>(Arc<RwLock<W>>);

impl<W> Shared<W> {
    pub fn update(&mut self, f: &Fn(&mut W)) {
        match self.0.write() {
            Ok(mut w) => f(w.deref_mut()),
            Err(_) => {},
        }
    }
    pub fn write(&self) -> LockResult<RwLockWriteGuard<W>> {
        self.0.write()
    }
    pub fn read(&self) -> LockResult<RwLockReadGuard<W>> {
        self.0.read()
    }
}

impl<W> Clone for Shared<W> {
    fn clone(&self) -> Self {
        Shared(self.0.clone())
    }
}

pub fn shared<W: Widget>(w: W) -> Shared<W> {
    Shared(Arc::new(RwLock::new(w)))
}

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

impl<T> Widget for Shared<T>
where
    T: Widget,
{
    fn render(&self, pos: Position, size: Size) -> Pane {
        (self.0).render(pos, size)
    }
    fn render_content(&self, size: Size) -> Option<Vec<String>> {
        (self.0).render_content(size)
    }
    fn render_focus(&self, size: Size) -> Option<Position> {
        (self.0).render_focus(size)
    }
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        (self.0).render_children(size)
    }
    fn render_bounds(&self) -> BoundSize {
        (self.0).render_bounds()
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
