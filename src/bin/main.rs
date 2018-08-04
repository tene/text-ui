extern crate text_ui;
use text_ui::input::Key;
use text_ui::widget::{Log, Readline};
use text_ui::{
    shared, widget::readline::ReadlineEvent, InputEvent, RenderBackend, RenderElement, Shared,
    TermionBackend, UIEvent, Widget, WidgetRenderContext,
};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum MyNames {
    Input,
}

#[derive(Debug)]
struct App {
    pub log: Shared<Log>,
    pub rl: Readline<MyNames>,
}

impl App {
    pub fn new() -> Self {
        let log = shared(Log::new());
        let logref = log.clone();
        let mut rl = Readline::new(MyNames::Input);
        rl.add_listener(Box::new(move |e| match e {
            ReadlineEvent::Submitted { line, .. } => match logref.write() {
                Ok(mut log) => {
                    log.log_msg(line);
                    true
                }
                Err(_) => false,
            },
        }));
        App { log, rl }
    }
}

impl<B: RenderBackend<MyNames>> Widget<B, MyNames> for App {
    fn render(&self, mut ctx: B::RenderContext) -> B::Element {
        use text_ui::ShouldPropagate::*;
        ctx.vbox(vec![&self.log, &self.rl]).add_input_handler(
            None,
            Box::new(move |ctx, e| match e {
                InputEvent::Key(Key::Esc) => {
                    ctx.send_event(UIEvent::Exit);
                    Stop
                }
                _ => Continue,
            }),
        )
    }
}

fn main() {
    let app = App::new();
    app.log.write().unwrap().log_msg("asdf");
    let mut be = TermionBackend::new();
    be.run(app, MyNames::Input);
}
