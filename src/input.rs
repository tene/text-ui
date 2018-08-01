pub use termion::event::Event as InputEvent;
pub use termion::event::{Key, MouseButton, MouseEvent};

/*
XXX UGH
These two need to be completely separate types, handled separately
*/

#[derive(Debug, PartialEq)]
pub enum UIEvent {
    Readline { source: String, line: String },
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

impl Event {
    pub fn readline(source: String, line: String) -> Self {
        Event::UI(UIEvent::Readline { source, line })
    }
}
