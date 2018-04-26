use termion::event::Event;
use widget::Widget;
use Size;

pub trait App {
    type UI: Widget;
    fn widget(&self) -> Self::UI;
    fn size(&self) -> Size;
    // TODO need an enum for this
    fn handle_event(&mut self, Event) -> Result<(), Option<String>>;
}
