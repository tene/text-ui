use input::{Event, InputEvent};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::mpsc::Sender;

pub mod log;
pub mod readline;

pub use self::log::Log;
pub use self::readline::Readline;

use {FullGrowthPolicy, Shared};

pub trait Name: Hash + Eq + Clone + Debug {}

impl<N> Name for N
where
    N: Hash + Eq + Clone + Debug,
{
}

pub trait RenderContext<B, N>
where
    B: RenderBackend<N>,
    N: Name,
{
    //fn constraints()
    //fn render_widget?  // break cycle by returning different types?
    //fn line constraintwidth?
    fn line(&mut self, &str) -> B::Element;
    fn text(&mut self, Vec<String>) -> B::Element;
    fn vbox(&mut self, Vec<&dyn Widget<B, N>>) -> B::Element;
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

pub trait RenderElement<N: Name> {
    //fn size(&self) -> Size;
    fn add_input_handler(&mut self, name: Option<N>, callback: Box<Fn(&InputEvent) -> bool>); // swap bool for ADT, swap name for generic
    fn handle_input(&self, N, &InputEvent);
}

pub trait RenderBackend<N: Name>: Sized {
    type Context: RenderContext<Self, N>;
    type Element: RenderElement<N>;
}

pub trait Widget<B, N>: Debug
where
    B: RenderBackend<N>,
    N: Name,
{
    fn render(&self, B::Context) -> B::Element;

    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::default()
    }
}

impl<W, B, N> Widget<B, N> for Shared<W>
where
    W: Widget<B, N>,
    B: RenderBackend<N>,
    N: Name,
{
    fn render(&self, ctx: B::Context) -> B::Element {
        self.read().unwrap().render(ctx)
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        self.read().unwrap().growth_policy()
    }
}
