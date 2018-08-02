use input::{Event, InputEvent};
use std::fmt::Debug;
use std::sync::mpsc::Sender;

pub mod log;
pub mod readline;

pub use self::log::Log;
pub use self::readline::Readline;

use {FullGrowthPolicy, Shared};

pub trait RenderContext<B>
where
    B: RenderBackend,
{
    //fn constraints()
    //fn render_widget?  // break cycle by returning different types?
    //fn line constraintwidth?
    fn line(&mut self, &str) -> B::Element;
    fn text(&mut self, Vec<String>) -> B::Element;
    fn vbox(&mut self, Vec<&dyn Widget<B>>) -> B::Element;
    fn event_sender(&self) -> Sender<Event>;
}

enum _ShouldRedraw {
    PleaseRedraw,
    NoRedrawNeeded,
}

enum _InputHandleResult {
    Ignored,
    Handled(_ShouldRedraw),
}

pub trait RenderElement {
    //fn size(&self) -> Size;
    fn add_input_handler(&mut self, name: &str, callback: Box<Fn(&InputEvent) -> bool>); // swap bool for ADT, swap name for generic
    fn handle_input(&self, String, &InputEvent); // Need to swap String as name out for generic
}

pub trait RenderBackend: Sized {
    type Context: RenderContext<Self>;
    type Element: RenderElement;
}

pub trait Widget<B>: Debug
where
    B: RenderBackend,
{
    fn render(&self, B::Context) -> B::Element;

    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::default()
    }
}

impl<W, B> Widget<B> for Shared<W>
where
    W: Widget<B>,
    B: RenderBackend,
{
    fn render(&self, ctx: B::Context) -> B::Element {
        self.read().unwrap().render(ctx)
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        self.read().unwrap().growth_policy()
    }
}
