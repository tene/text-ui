//use input::{MouseButton, MouseEvent};
use {shared, Color, Name, RenderBackend, Shared, TextBlock, Widget, WidgetRenderContext};

#[derive(Debug, Default)]
pub struct Log<N: Name> {
    pub lines: Vec<String>,
    pub scroll_pos: Shared<usize>,
    pub selected: Option<usize>,
    pub name: Option<N>,
    pub fg: Option<Color>,
}

impl<N> Log<N>
where
    N: Name,
{
    pub fn new(name: Option<N>, fg: Option<Color>) -> Self {
        let lines = vec![];
        let scroll_pos = shared(0);
        let selected = None;
        Log {
            lines,
            scroll_pos,
            selected,
            fg,
            name,
        }
    }
    pub fn log_msg(&mut self, msg: &str) {
        self.lines.push(msg.to_owned());
    }
}

impl<B, N> Widget<B, N> for Log<N>
where
    B: RenderBackend<N>,
    N: Name,
{
    fn name(&self) -> Option<N> {
        self.name
    }
    fn render(&self, ctx: B::RenderContext) -> B::Element {
        //let scroll_pos = self.scroll_pos.clone();
        let text = TextBlock::new_lines(self.name, "Log", "Content", self.lines.clone());
        ctx.text(text) /*.add_key_handler(
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
