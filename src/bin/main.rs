extern crate text_ui;
use text_ui::input::Key;
use text_ui::widget::{Log, Readline};
use text_ui::{
    shared, widget::readline::ReadlineEvent, Event, InputEvent, RenderBackend, RenderContext,
    RenderElement, Shared, TermionBackend, UIEvent, Widget,
};

#[derive(Debug)]
struct App {
    pub log: Shared<Log>,
    pub rl: Readline,
}

impl App {
    pub fn new() -> Self {
        let log = shared(Log::new());
        let logref = log.clone();
        let mut rl = Readline::new("input");
        rl.add_listener(Box::new(move |e| match e {
            ReadlineEvent::Submitted { name: _, line } => match logref.write() {
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

impl<B: RenderBackend> Widget<B> for App {
    fn render(&self, mut ctx: B::Context) -> B::Element {
        let sender = ctx.event_sender();
        let mut app = ctx.vbox(vec![&self.log, &self.rl]);
        app.add_input_handler(
            "app",
            Box::new(move |e| match e {
                InputEvent::Key(Key::Esc) => {
                    let _ = sender.send(Event::UI(UIEvent::Exit));
                    true
                }
                _ => false,
            }),
        );
        app
    }
}

fn main() {
    let app = App::new();
    app.log.write().unwrap().log_msg("asdf");
    let mut be = TermionBackend::new();
    be.run(app);
}
