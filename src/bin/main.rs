extern crate text_ui;
use text_ui::input::Key;
use text_ui::widget::{Log, Readline};
use text_ui::{shared, Backend, Element, Event, InputEvent, Shared, UIEvent, Widget};

#[derive(Debug)]
struct App {
    pub log: Shared<Log>,
    pub rl: Shared<Readline>,
}

impl App {
    pub fn new() -> Self {
        let log = shared(Log::new());
        let rl = shared(Readline::new("input"));
        App { log, rl }
    }
}

impl Widget for App {
    fn render(&self) -> Element {
        Element::vbox(vec![
            Element::proxy_shared(&self.log),
            Element::proxy_shared(&self.rl),
        ])
    }
    fn handle_event(&mut self, event: &Event) -> Option<Event> {
        match event {
            Event::Input(InputEvent::Key(Key::Esc)) => Some(Event::UI(UIEvent::Exit)),
            Event::Input(InputEvent::Key(_)) => self.rl.write().unwrap().handle_event(event),
            Event::Input(InputEvent::Mouse(_)) => self.log.write().unwrap().handle_event(event),
            Event::UI(UIEvent::Readline { source: _, line }) => {
                self.log.write().unwrap().log_msg(line);
                None
            }
            _ => None,
        }
    }
}

fn main() {
    let app = App::new();
    app.log.write().unwrap().log_msg("asdf");
    let mut be = Backend::new();
    be.run(app);
}
