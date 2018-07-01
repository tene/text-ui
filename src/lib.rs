#![feature(rust_2018_preview)]
extern crate termion;
extern crate unicode_segmentation;

use std::sync::{Arc, RwLock};

pub mod ast;
pub mod backend;
pub mod input;
pub mod widget;

pub use ast::{Bound, Bounds, Content, Element};
pub use backend::{Backend, Size};
pub use input::{Event, InputEvent, MouseEvent, UIEvent};
pub use widget::Widget;

pub type Shared<T> = Arc<RwLock<T>>;
pub fn shared<T>(item: T) -> Shared<T> {
    Arc::new(RwLock::new(item))
}
