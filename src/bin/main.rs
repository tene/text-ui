extern crate text_ui;
use text_ui::input::Key;
use text_ui::widget::{Log, Readline};
use text_ui::{
    shared, widget::layout::Linear, widget::readline::ReadlineEvent, App, AppEvent, Color,
    ContentID, EventContext, Executor, InputEvent, Line, RenderContext, Shared, Size,
    TermionBackend, TextBlock, Widget,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum MyNames {
    Log1,
    Log2,
    Input1,
    Input2,
}

#[derive(Debug)]
struct DemoApp {
    pub log1: Shared<Log<MyNames>>,
    pub log2: Shared<Log<MyNames>>,
    pub rl1: Readline<MyNames>,
    pub rl2: Readline<MyNames>,
}

impl DemoApp {
    pub fn new() -> Self {
        let log1 = shared(Log::new(Some(MyNames::Log1), Some(Color::Red)));
        let log2 = shared(Log::new(Some(MyNames::Log2), Some(Color::LightGreen)));
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
        Self {
            log1,
            log2,
            rl1,
            rl2,
        }
    }
}

impl Widget<MyNames> for DemoApp {
    fn name(&self) -> Option<MyNames> {
        None
    }
    fn render(&self, ctx: RenderContext<MyNames>) -> TextBlock<MyNames> {
        let vline = Line::vertical();
        let hline = Line::horizontal();
        let logs: Linear<MyNames> = Linear::hbox(vec![&self.log1, &vline, &self.log2]);
        let ui = Linear::vbox(vec![&logs, &hline, &self.rl1, &hline, &self.rl2]);
        ui.render(ctx)
    }
    fn widget_type(&self) -> &'static str {
        "DemoApp"
    }
}

impl App<MyNames> for DemoApp {
    fn handle_input(
        &mut self,
        ctx: &EventContext<MyNames>,
        event: &InputEvent,
    ) -> text_ui::ShouldPropagate {
        use text_ui::ShouldPropagate::*;
        match event {
            InputEvent::Key(Key::Esc) => {
                let _ = ctx.send_event(AppEvent::Exit);
                Stop
            }
            InputEvent::Key(Key::Ctrl('a')) => {
                let _ = ctx.send_event(AppEvent::SetFocus(MyNames::Input1));
                Stop
            }
            InputEvent::Key(Key::Ctrl('b')) => {
                let _ = ctx.send_event(AppEvent::SetFocus(MyNames::Input2));
                Stop
            }
            InputEvent::Mouse(m) => {
                self.log2.write().unwrap().log_msg(&format!("{:?}", m));
                Continue
            }
            _ => Continue,
        }
    }
    fn handle_resize(&mut self, size: Size) {
        self.log2
            .write()
            .unwrap()
            .log_msg(&format!("Resized to: {:?}", size));
    }
    fn style(&self, cid: ContentID<MyNames>) -> (Option<Color>, Option<Color>) {
        match cid.as_tuple() {
            (Some(MyNames::Log1), ..) => (Some(Color::Red), None),
            (Some(MyNames::Log2), ..) => (Some(Color::LightGreen), None),
            (_, _, _) => (None, None),
        }
    }
}

fn main() {
    let mut app = DemoApp::new();
    app.log1.write().unwrap().log_msg("Ctrl+A here");
    app.log2.write().unwrap().log_msg("Ctrl+B here");
    let mut ex: Executor<MyNames, TermionBackend> = Executor::new();
    ex.run(&mut app, MyNames::Input1);
}
