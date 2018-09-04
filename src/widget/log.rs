//use input::{MouseButton, MouseEvent};
use {shared, Name, RenderContext, Shared, TextBlock, Widget};

#[derive(Debug, Default)]
pub struct Log<N: Name> {
    pub lines: Vec<String>,
    pub scroll_pos: Shared<usize>,
    pub selected: Option<usize>,
    pub name: Option<N>,
}

impl<N> Log<N>
where
    N: Name,
{
    pub fn new(name: Option<N>) -> Self {
        let lines = vec![];
        let scroll_pos = shared(0);
        let selected = None;
        Log {
            lines,
            scroll_pos,
            selected,
            name,
        }
    }
    pub fn log_msg(&mut self, msg: &str) {
        self.lines.push(msg.to_owned());
    }
}

impl<N> Widget<N> for Log<N>
where
    N: Name,
{
    fn name(&self) -> Option<N> {
        self.name
    }
    fn render(&self, ctx: RenderContext<N>) -> TextBlock<N> {
        //let scroll_pos = self.scroll_pos.clone();
        ctx.wrap_lines("Content", self.lines.clone())
        /*.add_key_handler(
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
    fn widget_type(&self) -> &'static str {
        "Log"
    }
}
