use input::{MouseButton, MouseEvent};
use {Event, InputEvent, RenderBackend, RenderContext, Widget};

#[derive(Debug)]
pub struct Log {
    pub lines: Vec<String>,
    pub scroll_pos: usize,
    pub selected: Option<usize>,
}

impl Log {
    pub fn new() -> Self {
        let lines = vec![];
        let scroll_pos = 0;
        let selected = None;
        Log {
            lines,
            scroll_pos,
            selected,
        }
    }
    pub fn log_msg(&mut self, msg: &str) {
        self.lines.push(msg.to_owned());
    }
}

impl<B> Widget<B> for Log
where
    B: RenderBackend,
{
    fn render(&self, mut ctx: B::Context) -> B::Element {
        ctx.text(self.lines.clone())
    }
    /*fn handle_event(&mut self, event: &Event) -> Option<Event> {
        match event {
            Event::Input(InputEvent::Mouse(MouseEvent::Press(btn, _, _))) => {
                match btn {
                    MouseButton::WheelDown => self.scroll_pos += 1,
                    MouseButton::WheelUp => if self.scroll_pos > 0 {
                        self.scroll_pos -= 1
                    },
                    _ => {}
                };
            }
            _ => {}
        }
        None
    }*/
}
