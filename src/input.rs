pub use termion::event::Event as InputEvent;
pub use termion::event::{Key, MouseButton, MouseEvent};

// XXX TODO rename me
#[derive(Debug, PartialEq)]
pub enum UIEvent {
    Exit,
    // SetFocus(Name)
}

// Move to non-public in backend
#[derive(Debug, PartialEq)]
pub enum Event {
    Input(InputEvent),
    UI(UIEvent),
}
