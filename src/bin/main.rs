extern crate text_ui;
use text_ui::widget::{Log, Readline};
use text_ui::{Element, Event, InputEvent, UIEvent, Widget};

#[derive(Debug)]
struct App {
    pub log: Log,
    pub rl: Readline,
}

impl App {
    pub fn new() -> Self {
        let log = Log::new();
        let rl = Readline::new("input");
        App { log, rl }
    }
}

impl Widget for App {
    fn render(&self) -> Element {
        Element::vbox(vec![self.log.render(), self.rl.render()])
    }
    fn handle_event(&mut self, event: &Event) -> Option<Event> {
        match event {
            Event::Input(InputEvent::Key(_)) => self.rl.handle_event(event),
            Event::Input(InputEvent::Mouse(_)) => self.log.handle_event(event),
            Event::UI(UIEvent::Readline { source: _, line }) => {
                self.log.log_msg(line);
                None
            }
            _ => None,
        }
    }
}

fn main() {
    let mut app = App::new();
    app.log.log_msg("asdf");
    println!("{:#?}", app);
}
