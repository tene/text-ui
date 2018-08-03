use input::{MouseButton, MouseEvent};
use {shared, InputEvent, Name, RenderBackend, RenderContext, RenderElement, Shared, Widget};

#[derive(Debug)]
pub struct Log {
    pub lines: Vec<String>,
    pub scroll_pos: Shared<usize>,
    pub selected: Option<usize>,
}

impl Log {
    pub fn new() -> Self {
        let lines = vec![];
        let scroll_pos = shared(0);
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

impl<B, N> Widget<B, N> for Log
where
    B: RenderBackend<N>,
    N: Name,
{
    fn render(&self, mut ctx: B::Context) -> B::Element {
        let mut txt = ctx.text(self.lines.clone());
        let scroll_pos = self.scroll_pos.clone();
        txt.add_input_handler(
            None,
            Box::new(move |e| match e {
                InputEvent::Mouse(MouseEvent::Press(btn, _, _)) => {
                    let mut sp = scroll_pos.write().unwrap();
                    match btn {
                        MouseButton::WheelDown => *sp += 1,
                        MouseButton::WheelUp => if *sp > 0 {
                            *sp -= 1
                        },
                        _ => {}
                    };
                    true
                }
                _ => false,
            }),
        );
        txt
    }
}
