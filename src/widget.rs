use ast::Element;
use input::Event;

pub mod log;
pub mod readline;

pub use self::log::Log;
pub use self::readline::Readline;

pub trait Widget {
    fn render(&self) -> Element;
    fn handle_event(&mut self, &Event) -> Option<Event>;
}
