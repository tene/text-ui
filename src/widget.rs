use input::Event;
use std::fmt::Debug;

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
}

pub trait RenderElement {
    //fn size(&self) -> Size;
    //fn focus(&mut self, impl Fn InputEvent) -> should_propagate  // Handle per-Widget events (form submission) with listener callbacks?
    fn add_input_handler(&mut self, name: &str, callback: impl FnMut(&Event) -> bool); // swap bool for ADT, swap name for generic
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
    fn handle_event(&mut self, &Event) -> Option<Event>; // drop in favor of renderelement focus

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
    fn handle_event(&mut self, event: &Event) -> Option<Event> {
        self.write().unwrap().handle_event(event)
    }
    fn growth_policy(&self) -> FullGrowthPolicy {
        self.read().unwrap().growth_policy()
    }
}
