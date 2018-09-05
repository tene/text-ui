use input::{InputEvent, Key, MouseEvent};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::mpsc::Sender;

use AppEvent;

pub mod layout;
pub mod line;
pub mod list;
pub mod log;
pub mod simple_input;

pub use self::layout::Linear;
pub use self::line::Line;
pub use self::list::List;
pub use self::log::Log;
pub use self::simple_input::SimpleInput;

use executor::Event;
use {Color, ContentID, Frame, FullGrowthPolicy, Pos, RenderBound, Shared, Size, TextBlock};

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
pub struct RenderContext<N: Name> {
    bound: RenderBound,
    name: Option<N>,
    widget_type: &'static str,
}

impl<N: Name> RenderContext<N> {
    pub(crate) fn from_widget(bound: RenderBound, widget: &dyn Widget<N>) -> Self {
        let name = widget.name();
        let widget_type = widget.widget_type();
        Self::new(bound, name, widget_type)
    }
    fn new(bound: RenderBound, name: Option<N>, widget_type: &'static str) -> Self {
        Self {
            bound,
            name,
            widget_type,
        }
    }
    pub fn with_bound(&self, bound: RenderBound) -> Self {
        Self::new(bound, self.name, self.widget_type)
    }
    pub fn render_sized(&self, bound: RenderBound, widget: &dyn Widget<N>) -> TextBlock<N> {
        let block = widget.render(Self::from_widget(bound, widget));
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
    pub fn clip_lines(&self, class: &'static str, lines: Vec<String>) -> TextBlock<N> {
        let id = ContentID::new(self.name, class, self.widget_type);
        TextBlock::clip_lines(id, lines, self.bound)
    }
    pub fn wrap_lines(&self, class: &'static str, lines: Vec<String>) -> TextBlock<N> {
        let id = ContentID::new(self.name, class, self.widget_type);
        TextBlock::wrap_lines(id, lines, self.bound)
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
    sender: Sender<Event<N>>,
}

impl<N: Name> EventContext<N> {
    pub(crate) fn new(sender: Sender<Event<N>>) -> Self {
        Self { sender }
    }
    pub fn send_event(&self, event: AppEvent<N>) -> Result<(), ()> {
        self.sender.send(Event::App(event)).map_err(|_| ())
    }
}

// XXX TODO I think we can avoid having this parameterized somehow...
#[derive(Clone)]
pub struct BackendContext<N: Name> {
    sender: Sender<Event<N>>,
}

impl<N: Name> BackendContext<N> {
    pub(crate) fn new(sender: Sender<Event<N>>) -> Self {
        Self { sender }
    }
    pub fn send_input(&self, input: InputEvent) -> Result<(), ()> {
        self.sender.send(Event::Input(input)).map_err(|_| ())
    }
    pub fn resize(&self, size: Size) -> Result<(), ()> {
        self.sender.send(Event::Resize(size)).map_err(|_| ())
    }
}

pub trait RenderBackend {
    fn paint_frame(&mut self, frame: Frame);
    fn new<N: Name + 'static>(BackendContext<N>) -> Self; // XXX TODO I think we can avoid having this parameterized somehow...
    fn size(&self) -> Size;
    fn resize(&mut self, Size);
}

pub trait Widget<N>: Debug
where
    N: Name,
{
    fn render(&self, RenderContext<N>) -> TextBlock<N>;

    fn name(&self) -> Option<N>;

    fn widget_type(&self) -> &'static str;

    fn growth_policy(&self) -> FullGrowthPolicy {
        FullGrowthPolicy::default()
    }
}

impl<W, N> Widget<N> for Shared<W>
where
    W: Widget<N>,
    N: Name,
{
    fn render(&self, ctx: RenderContext<N>) -> TextBlock<N> {
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

    fn widget_type(&self) -> &'static str {
        self.read().unwrap().widget_type()
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
