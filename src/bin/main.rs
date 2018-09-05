extern crate text_ui;
use text_ui::input::Key;
use text_ui::widget::{List, Log, SimpleInput};
use text_ui::{
    shared, widget::layout::Linear, widget::simple_input::SimpleInputEvent, App, AppEvent, Color,
    ContentID, EventContext, Executor, InputEvent, Line, Readline, ReadlineEvent, RenderContext,
    Shared, Size, TermionBackend, TextBlock, Widget,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum MyNames {
    Log1,
    Log2,
    Input1,
    Input2,
    NumberList,
    Number(usize),
}

#[derive(Debug)]
struct DemoApp {
    pub log1: Shared<Log<MyNames>>,
    pub log2: Shared<Log<MyNames>>,
    pub simpleinput: SimpleInput<MyNames>,
    pub rlinput: Readline<MyNames>,
    pub nl: List<MyNames, Vec<Log<MyNames>>>,
}

impl DemoApp {
    pub fn new() -> Self {
        let log1 = shared(Log::new(Some(MyNames::Log1)));
        let log2 = shared(Log::new(Some(MyNames::Log2)));
        let logref1 = log1.clone();
        let logref2 = log2.clone();
        let simpleinput =
            SimpleInput::new(MyNames::Input1).add_listener(Box::new(move |e| match e {
                SimpleInputEvent::Submitted { line, .. } => match logref1.write() {
                    Ok(mut log) => {
                        log.log_msg(line);
                        true
                    }
                    Err(_) => false,
                },
            }));
        let rlinput = Readline::new(MyNames::Input2).add_listener(Box::new(move |e| match e {
            ReadlineEvent::Submitted { line, .. } => match logref2.write() {
                Ok(mut log) => {
                    log.log_msg(line);
                    true
                }
                Err(_) => false,
            },
        }));
        let numbers: Vec<Log<MyNames>> = (1..100)
            .map(|i| {
                let mut log = Log::new(Some(MyNames::Number(i)));
                for _ in 0..i {
                    log.log_msg(&format!("Blah {}", i));
                }
                log
            }).collect();
        let nl = List::new(Some(MyNames::NumberList), numbers, 5, 2);
        Self {
            log1,
            log2,
            simpleinput,
            rlinput,
            nl,
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
        let logs: Linear<MyNames> =
            Linear::hbox(vec![&self.log1, &vline, &self.log2, &vline, &self.nl]);
        let ui = Linear::vbox(vec![
            &logs,
            &hline,
            &self.simpleinput,
            &hline,
            &self.rlinput,
        ]);
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
            (Some(MyNames::Number(n)), ..) => match n % 3 {
                0 => (Some(Color::LightBlue), None),
                1 => (Some(Color::LightRed), None),
                2 => (Some(Color::LightYellow), None),
                _ => unreachable!(),
            },
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
