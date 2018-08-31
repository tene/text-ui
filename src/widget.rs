use input::{InputEvent, Key, MouseEvent};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::mpsc::Sender;

use AppEvent;

pub mod layout;
pub mod line;
pub mod log;
pub mod readline;

pub use self::layout::Linear;
pub use self::line::Line;
pub use self::log::Log;
pub use self::readline::Readline;

use {
    Color, ContentID, Direction, FullGrowthPolicy, Pos, RenderBound, Shared, Size, TextBlock,
    TextLine,
};

pub trait Name: Hash + Eq + Clone + Copy + Debug + Send {}

impl<N> Name for N where N: Hash + Eq + Clone + Copy + Debug + Send {}

pub enum ShouldPropagate {
    Continue,
    Stop,
}

pub type KeyCallback<N> = Box<Fn(&EventContext<N>, Key) -> ShouldPropagate>;
// XXX TODO need to use internal mouse event type instead of termion's
pub type MouseCallback<N> = Box<Fn(&EventContext<N>, Pos, MouseEvent) -> ShouldPropagate>;

#[derive(Clone)]
pub struct RenderContext {
    bound: RenderBound,
}

impl RenderContext {
    pub fn new(bound: RenderBound) -> Self {
        Self { bound }
    }
    pub fn with_bound(&self, bound: RenderBound) -> Self {
        Self::new(bound)
    }
    pub fn render_sized<N: Name, W: Widget<N>>(
        &self,
        bound: RenderBound,
        widget: &W,
    ) -> TextBlock<N> {
        let block = widget.render(Self::new(bound));
        let size = block.size();
        if let Some(width) = bound.width {
            //assert_eq!(width, size.cols);
            if width != size.cols {
                panic!(
                    "bad block width!\nwidget: {:#?}\nbound: {:?}\nblock: {:#?}",
                    widget, bound, block
                );
            }
        }
        if let Some(height) = bound.height {
            //assert_eq!(height, size.rows);
            if height != size.rows {
                panic!(
                    "bad block height!\nwidget: {:#?}\nbound: {:?}\nblock: {:#?}",
                    widget, bound, block
                );
            }
        }
        block
    }
    pub fn bound(&self) -> RenderBound {
        self.bound
    }
    /*    fn line<F: Into<Fragment>>(&self, content: F) -> Block<N> {
        let fragment: Fragment = content.into();
        Block::from_fragment(&fragment, self.bound.constrain_height(1))
    }
    fn text<F: Into<Fragment>>(&self, content: F) -> Block<N> {
        let fragment: Fragment = content.into();
        Block::from_fragment(&fragment, self.bound)
    }*/
    /*
    fn clip_line<L: Into<TextLine<N>>>(&self, line: L) -> Block<N> {
        unimplemented!();
    }
    fn wrap_line<L: Into<TextLine<N>>>(&self, line: L) -> Block<N> {
        unimplemented!();
    }
    fn text<T: Into<TextBlock<N>>>(&self, text: T) -> Block<N> {
        unimplemented!();
    }
    */
}

pub struct EventContext<N: Name> {
    sender: Sender<AppEvent<N>>,
}

impl<N: Name> EventContext<N> {
    pub fn new(sender: Sender<AppEvent<N>>) -> Self {
        Self { sender }
    }
    pub fn send_event(&self, event: AppEvent<N>) {
        let _ = self.sender.send(event);
    }
}

pub trait RenderBackend<N: Name>: Sized {
    // XXX TODO Rather than accepting a TextBlock, this should accept a Frame, that's just Size + Lines + Option<Focus>
    fn paint_frame(&mut self, frame: &TextBlock<N>, focus: &N);
}

pub trait Widget<N>: Debug
where
    N: Name,
{
    fn render(&self, RenderContext) -> TextBlock<N>;

    fn name(&self) -> Option<N>;

    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::default()
    }
}

impl<W, N> Widget<N> for Shared<W>
where
    W: Widget<N>,
    N: Name,
{
    fn render(&self, ctx: RenderContext) -> TextBlock<N> {
        self.read().unwrap().render(ctx)
    }

    // XXX TODO need to replace growth_policy with fn get_bounds(&self, bounds: Bounds) -> Bounds
    // Must return something that fits within the given bounds
    // This is needed for layout, to propagate back up minimum sizes from children
    fn growth_policy(&self) -> FullGrowthPolicy {
        self.read().unwrap().growth_policy()
    }

    fn name(&self) -> Option<N> {
        self.read().unwrap().name()
    }
}

pub trait App<N>: Widget<N>
where
    N: Name,
{
    fn handle_input(&mut self, _ctx: &EventContext<N>, _event: &InputEvent) -> ShouldPropagate {
        ShouldPropagate::Continue
    }
    fn handle_resize(&mut self, Size) {}
    fn style(&self, ContentID<N>) -> (Option<Color>, Option<Color>);
}
