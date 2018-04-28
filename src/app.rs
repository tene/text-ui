use widget::Widget;
use Event;
use Size;

pub trait App {
    type UI: Widget;
    type MyEvent: Send;
    fn widget(&self) -> Self::UI;
    fn size(&self) -> Size;
    // TODO need an enum for this
    fn handle_event(&mut self, Event<Self::MyEvent>) -> Result<(), Option<String>>;
}
