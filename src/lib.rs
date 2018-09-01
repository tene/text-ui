#![feature(rust_2018_preview)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default))]

extern crate libc;
extern crate signal_hook;
extern crate termion;
extern crate unicode_segmentation;

use std::ops;
use std::sync::{Arc, RwLock};

pub mod backend;
pub mod executor;
mod indextree;
pub mod input;
pub mod ir;
pub mod widget;

pub use backend::TermionBackend;
pub use executor::Executor;
pub use input::{InputEvent, Key, MouseEvent};
pub use ir::{ContentID, Frame, FrameLine, Segment, TextBlock, TextLine};
pub use widget::{
    App, BackendContext, EventContext, KeyCallback, Line, Linear, MouseCallback, Name,
    RenderBackend, RenderContext, ShouldPropagate, Widget,
};

#[derive(Debug, PartialEq)]
pub enum AppEvent<N: Name> {
    Exit,
    SetFocus(N),
}

pub type Shared<T> = Arc<RwLock<T>>;
pub fn shared<T>(item: T) -> Shared<T> {
    Arc::new(RwLock::new(item))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub cols: usize,
    pub rows: usize,
}

impl Size {
    pub fn new(cols: usize, rows: usize) -> Self {
        Size { cols, rows }
    }
    pub fn in_direction(&self, dir: Direction) -> usize {
        match dir {
            Direction::Vertical => self.rows,
            Direction::Horizontal => self.cols,
        }
    }
    pub fn against_direction(&self, dir: Direction) -> usize {
        match dir {
            Direction::Horizontal => self.rows,
            Direction::Vertical => self.cols,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Direction {
    pub fn against(self) -> Self {
        match self {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GrowthPolicy {
    FixedSize,
    Greedy,
}

impl Default for GrowthPolicy {
    fn default() -> Self {
        GrowthPolicy::Greedy
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct FullGrowthPolicy {
    pub width: GrowthPolicy,
    pub height: GrowthPolicy,
}

impl FullGrowthPolicy {
    pub fn fixed_height() -> FullGrowthPolicy {
        let width = GrowthPolicy::Greedy;
        let height = GrowthPolicy::FixedSize;
        FullGrowthPolicy { width, height }
    }
    pub fn fixed_width() -> FullGrowthPolicy {
        let width = GrowthPolicy::FixedSize;
        let height = GrowthPolicy::Greedy;
        FullGrowthPolicy { width, height }
    }
    pub fn in_direction(&self, dir: Direction) -> GrowthPolicy {
        match dir {
            Direction::Vertical => self.height,
            Direction::Horizontal => self.width,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub col: usize,
    pub row: usize,
}

impl Pos {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let col = self.col + other.col;
        let row = self.row + other.row;
        Self { col, row }
    }
}

impl ops::Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let col = self.col - other.col;
        let row = self.row - other.row;
        Self { col, row }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderBound {
    pub width: Option<usize>, // Maybe this should be NonZeroUsize?
    pub height: Option<usize>,
}

impl RenderBound {
    pub fn new(width: Option<usize>, height: Option<usize>) -> Self {
        Self { width, height }
    }
    pub fn against_direction(&self, dir: Direction) -> Option<usize> {
        match dir {
            Direction::Vertical => self.width,
            Direction::Horizontal => self.height,
        }
    }
    pub fn in_direction(&self, dir: Direction) -> Option<usize> {
        match dir {
            Direction::Horizontal => self.width,
            Direction::Vertical => self.height,
        }
    }
    pub fn free_direction(&self, dir: Direction) -> Self {
        match dir {
            Direction::Vertical => Self {
                width: self.width,
                height: None,
            },
            Direction::Horizontal => Self {
                width: None,
                height: self.height,
            },
        }
    }
    pub fn constrain_direction(&self, dir: Direction, constraint: usize) -> Self {
        let new = Some(constraint);
        match dir {
            Direction::Vertical => Self {
                width: self.width,
                height: new,
            },
            Direction::Horizontal => Self {
                width: new,
                height: self.height,
            },
        }
    }
    pub fn constrain_height(&self, constraint: usize) -> Self {
        Self {
            width: self.width,
            height: Some(constraint),
        }
    }
    pub fn constrain_against(&self, dir: Direction, constraint: usize) -> Self {
        self.constrain_direction(dir.against(), constraint)
    }
}

impl From<Size> for RenderBound {
    fn from(size: Size) -> Self {
        let width = Some(size.cols);
        let height = Some(size.rows);
        RenderBound { width, height }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
    Rgb(u8, u8, u8),
    Reset,
}
