extern crate termion;
pub mod app;
pub mod backend;
pub mod pane;
pub mod widget;
use std::ops::Add;

pub use termion::event::Event as Input;

// XXX Ugh, naming?!?!
pub enum Event<A> {
    InputEvent(Input),
    AppEvent(A),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Position {
        Position { x: x, y: y }
    }
    pub fn offset(self, x: u16, y: u16) -> Position {
        Position {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Size {
        Size {
            width: width,
            height: height,
        }
    }
    pub fn offset(self, width: u16, height: u16) -> Size {
        Size {
            width: self.width + width,
            height: self.height + height,
        }
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Size {
        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}
