//use input::{MouseButton, MouseEvent};
use {shared, Color, Fragment, Name, RenderBackend, Shared, Widget, WidgetRenderContext};

#[derive(Debug, Default)]
pub struct Log {
    pub lines: Vec<String>,
    pub scroll_pos: Shared<usize>,
    pub selected: Option<usize>,
    pub fg: Option<Color>,
}

impl Log {
    pub fn new(fg: Option<Color>) -> Self {
        let lines = vec![];
        let scroll_pos = shared(0);
        let selected = None;
        Log {
            lines,
            scroll_pos,
            selected,
            fg,
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
    fn render(&self, ctx: B::RenderContext) -> B::Element {
        //let scroll_pos = self.scroll_pos.clone();
        let mut frag: Fragment = self.lines.clone().into();
        frag.fg = self.fg;
        ctx.text(frag) /*.add_key_handler(
            None,
            Box::new(move |_ctx, e| {
                use ShouldPropagate::*;
                match e {
                    InputEvent::Mouse(MouseEvent::Press(btn, _, _)) => {
                        let mut sp = scroll_pos.write().unwrap();
                        match btn {
                            MouseButton::WheelDown => *sp += 1,
                            MouseButton::WheelUp => if *sp > 0 {
                                *sp -= 1
                            },
                            _ => {}
                        };
                        Stop
                    }
                    _ => Continue,
                }
            }),
        )*/
    }
}
