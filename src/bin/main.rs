extern crate text_ui;
use text_ui::input::Key;
use text_ui::widget::{Log, Readline};
use text_ui::{
    shared, widget::layout::Linear, widget::readline::ReadlineEvent, AppEvent, InputEvent, Line,
    RenderBackend, RenderElement, Shared, TermionBackend, Widget, WidgetRenderContext,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum MyNames {
    Input1,
    Input2,
}

#[derive(Debug)]
struct App {
    pub log1: Shared<Log>,
    pub log2: Shared<Log>,
    pub rl1: Readline<MyNames>,
    pub rl2: Readline<MyNames>,
}

impl App {
    pub fn new() -> Self {
        let log1 = shared(Log::new());
        let log2 = shared(Log::new());
        let logref1 = log1.clone();
        let logref2 = log2.clone();
        let rl1 = Readline::new(MyNames::Input1).add_listener(Box::new(move |e| match e {
            ReadlineEvent::Submitted { line, .. } => match logref1.write() {
                Ok(mut log) => {
                    log.log_msg(line);
                    true
                }
                Err(_) => false,
            },
        }));
        let rl2 = Readline::new(MyNames::Input2).add_listener(Box::new(move |e| match e {
            ReadlineEvent::Submitted { line, .. } => match logref2.write() {
                Ok(mut log) => {
                    log.log_msg(line);
                    true
                }
                Err(_) => false,
            },
        }));
        App {
            log1,
            log2,
            rl1,
            rl2,
        }
    }
}

impl<B: RenderBackend<MyNames>> Widget<B, MyNames> for App {
    fn render(&self, ctx: B::RenderContext) -> B::Element {
        use text_ui::ShouldPropagate::*;
        let vline = Line::vertical();
        let hline = Line::horizontal();
        let logs = Linear::hbox(vec![&self.log1, &vline, &self.log2]);
        let ui = Linear::vbox(vec![&logs, &hline, &self.rl1, &hline, &self.rl2]);
        ctx.render(&ui).add_input_handler(
            None,
            Box::new(move |ctx, e| match e {
                InputEvent::Key(Key::Esc) => {
                    ctx.send_event(AppEvent::Exit);
                    Stop
                }
                InputEvent::Key(Key::Ctrl('a')) => {
                    ctx.send_event(AppEvent::SetFocus(MyNames::Input1));
                    Stop
                }
                InputEvent::Key(Key::Ctrl('b')) => {
                    ctx.send_event(AppEvent::SetFocus(MyNames::Input2));
                    Stop
                }
                _ => Continue,
            }),
        )
    }
}

fn main() {
    let app = App::new();
    app.log1.write().unwrap().log_msg("Ctrl+A here");
    app.log2.write().unwrap().log_msg("Ctrl+B here");
    let mut be = TermionBackend::new();
    be.run(app, MyNames::Input1);
}
