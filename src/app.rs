use backend::{color, Color};
use widget::Widget;
use Event;

pub trait App {
    type UI: Widget;
    type MyEvent: Send;
    fn widget(&self) -> Self::UI;
    // TODO need an enum for this
    fn handle_event(&mut self, Event<Self::MyEvent>) -> Result<(), Option<String>>;
    fn style(&self, &str) -> (Option<Box<Color>>, Option<Box<Color>>) {
        (None, None)
    }
    fn default_style(&self) -> Box<Color> {
        Box::new(color::Reset)
    }
}
