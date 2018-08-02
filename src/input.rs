pub use termion::event::Event as InputEvent;
pub use termion::event::{Key, MouseButton, MouseEvent};

#[derive(Debug, PartialEq)]
pub enum UIEvent {
    Exit,
}

// Maybe this should be done with From/Into instead?
#[derive(Debug, PartialEq)]
pub enum Event {
    Input(InputEvent),
    UI(UIEvent),
    // UIEvent (form/readline submit)
    //AppEvent(A),
}
