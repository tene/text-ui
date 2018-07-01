use ast::Element;
use input::Event;
use std::fmt::Debug;

pub mod log;
pub mod readline;

pub use self::log::Log;
pub use self::readline::Readline;

use Shared;

pub trait Widget: Debug {
    fn render(&self) -> Element;
    fn handle_event(&mut self, &Event) -> Option<Event>;
}

impl<W> Widget for Shared<W>
where
    W: Widget,
{
    fn render(&self) -> Element {
        self.read().unwrap().render()
    }
    fn handle_event(&mut self, event: &Event) -> Option<Event> {
        self.write().unwrap().handle_event(event)
    }
}
