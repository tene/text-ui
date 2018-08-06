#![feature(rust_2018_preview)]
extern crate termion;
extern crate unicode_segmentation;

use std::ops;
use std::sync::{Arc, RwLock};

pub mod backend;
mod indextree;
pub mod input;
pub mod widget;

pub use backend::{Size, TermionBackend};
pub use input::{InputEvent, MouseEvent};
pub use widget::{
    InputCallback, Name, RenderBackend, RenderElement, ShouldPropagate, Widget, WidgetEventContext,
    WidgetRenderContext,
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
