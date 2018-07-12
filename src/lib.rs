#![feature(rust_2018_preview)]
extern crate termion;
extern crate unicode_segmentation;

use std::sync::{Arc, RwLock};

pub mod backend;
pub mod input;
pub mod widget;

pub use backend::{Backend, Size};
pub use input::{Event, InputEvent, MouseEvent, UIEvent};
pub use widget::{RenderBackend, RenderContext, RenderElement, Widget};

pub type Shared<T> = Arc<RwLock<T>>;
pub fn shared<T>(item: T) -> Shared<T> {
    Arc::new(RwLock::new(item))
}

#[derive(Debug, PartialEq)]
pub enum Bound {
    Fixed,
    Greedy,
}

impl Default for Bound {
    fn default() -> Self {
        Bound::Greedy
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Bounds {
    pub width: Bound,
    pub height: Bound,
}

impl Bounds {
    pub fn fixed_height() -> Bounds {
        let width = Bound::Greedy;
        let height = Bound::Fixed;
        Bounds { width, height }
    }
}
