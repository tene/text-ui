use input::InputEvent;
use std::fmt::Debug;
use std::hash::Hash;

use AppEvent;

pub mod layout;
pub mod line;
pub mod log;
pub mod readline;

pub use self::layout::Linear;
pub use self::line::Line;
pub use self::log::Log;
pub use self::readline::Readline;

use {Direction, FullGrowthPolicy, Pos, RenderBound, Shared, Size};

pub trait Name: Hash + Eq + Clone + Copy + Debug + Send {}

impl<N> Name for N where N: Hash + Eq + Clone + Copy + Debug + Send {}

pub enum ShouldPropagate {
    Continue,
    Stop,
}

pub type InputCallback<B, N> = Box<Fn(&WidgetEventContext<B, N>, &InputEvent) -> ShouldPropagate>;

pub trait WidgetRenderContext<B, N>
where
    B: RenderBackend<N>,
    N: Name,
{
    fn bound(&self) -> RenderBound;
    fn render(&self, widget: &Widget<B, N>) -> B::Element;
    fn with_bound(&self, bound: RenderBound) -> Self;
    fn render_sized(&self, bound: RenderBound, widget: &Widget<B, N>) -> B::Element;
    fn line(&self, &str) -> B::Element;
    fn text(&self, Vec<String>) -> B::Element;
}

pub trait WidgetEventContext<B, N>
where
    N: Name,
    B: RenderBackend<N>,
{
    fn send_event(&self, AppEvent<N>);
}

pub trait RenderElement<B, N>: Sized
where
    N: Name,
    B: RenderBackend<N>,
{
    fn size(&self) -> Size;
    fn add_input_handler(self, name: Option<N>, callback: InputCallback<B, N>) -> Self;
    fn add_cursor(self, name: N, pos: Pos) -> Self;
    fn get_cursor(&self, name: &N) -> Option<Pos>;
    fn vconcat(self, other: Self) -> Self;
    fn hconcat(self, other: Self) -> Self;
    fn concat_dir(self, direction: Direction, other: Self) -> Self {
        match direction {
            Direction::Horizontal => self.hconcat(other),
            Direction::Vertical => self.vconcat(other),
        }
    }
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
        ctx.render(&*self.read().unwrap())
    }

    // XXX TODO need to replace growth_policy with fn get_bounds(&self, bounds: Bounds) -> Bounds
    // Must return something that fits within the given bounds
    // This is needed for layout, to propagate back up minimum sizes from children
    fn growth_policy(&self) -> FullGrowthPolicy {
        self.read().unwrap().growth_policy()
    }
}
