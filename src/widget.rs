use input::Event;
use std::fmt::Debug;

pub mod log;
pub mod readline;

pub use self::log::Log;
pub use self::readline::Readline;

use Shared;

pub trait RenderContext<B>
where
    B: RenderBackend,
{
    fn line(&mut self, &str) -> B::Element;
    fn text(&mut self, Vec<String>) -> B::Element;
    fn vbox(&mut self, Vec<&dyn Widget<B>>) -> B::Element;
}

pub trait RenderElement {}

pub trait RenderBackend: Sized {
    type Context: RenderContext<Self>;
    type Element: RenderElement;
}

pub trait Widget<B>: Debug
where
    B: RenderBackend,
{
    fn render(&self, B::Context) -> B::Element;
    fn handle_event(&mut self, &Event) -> Option<Event>;
}

impl<W, B> Widget<B> for Shared<W>
where
    W: Widget<B>,
    B: RenderBackend,
{
    fn render(&self, ctx: B::Context) -> B::Element {
        self.read().unwrap().render(ctx)
    }
    fn handle_event(&mut self, event: &Event) -> Option<Event> {
        self.write().unwrap().handle_event(event)
    }
}

/*
fn compose_vbox(elements: Vec<Element>, size: Size) -> Block {
    let (fixed, greedy): (Vec<(usize, Element)>, Vec<(usize, Element)>) = elements
        .into_iter()
        .enumerate()
        .partition(|(_, e)| e.bounds.height == Bound::Fixed);
    let mut remaining_rows = size.rows;
    let cols = size.cols;
    let greedy_count = greedy.len();
    let mut blocks: Vec<(usize, Block)> = fixed
        .into_iter()
        .map(|(i, e)| {
            let b = compose_image(e, Size::new(cols, remaining_rows));
            remaining_rows -= b.height;
            (i, b)
        })
        .collect();
    blocks.extend(greedy.into_iter().map(|(i, e)| {
        let b = compose_image(e, Size::new(cols, remaining_rows / greedy_count));
        remaining_rows -= b.height;
        (i, b)
    }));
    blocks.sort_by_key(|a| a.0);
    let init = Block::new(vec![], cols, 0);
    blocks.into_iter().fold(init, |mut acc, (_, b)| {
        acc.vconcat(b);
        acc
    })
}
*/
