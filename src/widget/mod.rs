pub mod debug;
pub mod input;
pub mod line;
pub mod linear;
pub mod readline;
pub mod text;

pub use self::debug::DbgDump;
pub use self::input::TextInput;
pub use self::line::Line;
pub use self::linear::Linear;
pub use self::readline::Readline;
pub use self::text::Text;

use pane::Pane;
use {Position, Size};

use std::fmt;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::{LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Shared<W>(Arc<RwLock<W>>);

impl<W> Shared<W> {
    pub fn update(&mut self, f: &Fn(&mut W)) {
        if let Ok(mut w) = self.0.write() {
            f(w.deref_mut())
        }
    }
    pub fn write(&self) -> LockResult<RwLockWriteGuard<W>> {
        self.0.write()
    }
    pub fn read(&self) -> LockResult<RwLockReadGuard<W>> {
        self.0.read()
    }
}

impl<W> From<Arc<RwLock<W>>> for Shared<W> {
    fn from(s: Arc<RwLock<W>>) -> Self {
        Shared(s)
    }
}

impl<W> fmt::Debug for Shared<W>
where
    W: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.read().unwrap().fmt(f)
    }
}

impl<W> Clone for Shared<W> {
    fn clone(&self) -> Self {
        Shared(self.0.clone())
    }
}

pub fn shared<W>(w: W) -> Shared<W> {
    Shared(Arc::new(RwLock::new(w)))
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bound {
    Fixed(usize),
    AtLeast(usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoundSize {
    pub width: Bound,
    pub height: Bound,
}

impl BoundSize {
    fn split(&self) -> (Bound, Bound) {
        (self.width, self.height)
    }
}

pub trait Widget {
    fn render(&self, position: Position, size: Size) -> Pane {
        let content = self.render_content(size);
        let focus = self.render_focus(size);
        let children = self.render_children(size);
        let style = self.render_style();
        Pane {
            position,
            size,
            content,
            focus,
            children,
            style,
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
    fn render_style(&self) -> Option<String> {
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
