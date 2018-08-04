use input::{InputEvent, UIEvent};
use std::fmt::Debug;
use std::hash::Hash;

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

pub trait WidgetRenderContext<B, N>
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
}

pub trait WidgetEventContext<B, N>
where
    N: Name,
    B: RenderBackend<N>,
{
    fn send_event(&self, UIEvent);
}

enum _ShouldRedraw {
    PleaseRedraw,
    NoRedrawNeeded,
}

enum _InputHandleResult {
    Ignored,
    Handled(_ShouldRedraw),
}

pub trait RenderElement<B, N>
where
    N: Name,
    B: RenderBackend<N>,
{
    //fn size(&self) -> Size;
    fn add_input_handler(
        &mut self,
        name: Option<N>,
        callback: Box<Fn(&B::EventContext, &InputEvent) -> bool>,
    ); // swap bool for ADT, swap name for generic
}

pub trait RenderBackend<N: Name>: Sized {
    type RenderContext: WidgetRenderContext<Self, N>;
    type EventContext: WidgetEventContext<Self, N>;
    type Element: RenderElement<Self, N>;
}

pub trait Widget<B, N>: Debug
where
    B: RenderBackend<N>,
    N: Name,
{
    fn render(&self, B::RenderContext) -> B::Element;

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
    fn render(&self, ctx: B::RenderContext) -> B::Element {
        self.read().unwrap().render(ctx)
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        self.read().unwrap().growth_policy()
    }
}
