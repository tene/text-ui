extern crate text_ui;
use text_ui::app::App;
use text_ui::backend::Backend;
use text_ui::widget::{shared, Linear, Shared, Text, TextInput, Widget};
use text_ui::{Event, Input, Position, Size};

extern crate termion;
use termion::event::Key;

use std::thread;
use std::time::Duration;

struct DemoApp {
    log: Shared<Text>,
    timer: Shared<Text>,
    input: Shared<TextInput>,
    vbox: Shared<Linear>,
    outputs: Shared<Linear>,
    height: u16,
    width: u16,
    counter: u32,
}

impl DemoApp {
    fn new() -> DemoApp {
        let size = termion::terminal_size().unwrap();
        let log = shared(Text::new(vec![]));
        let timer = shared(Text::new(vec![]));
        let input = shared(TextInput::new(""));
        let mut outbox = Linear::hbox();
        outbox.push(&log);
        outbox.push(&timer);
        let outputs = shared(outbox);
        let mut mainbox = Linear::vbox();
        mainbox.push(&outputs);
        mainbox.push(&input);
        let vbox = shared(mainbox);
        DemoApp {
            log: log,
            input: input,
            timer: timer,
            vbox: vbox,
            outputs: outputs,
            width: size.0,
            height: size.1,
            counter: 0,
        }
    }

    fn submit_input(&mut self) {
        self.log
            .write()
            .unwrap()
            .push(self.input.write().unwrap().submit());
    }

    fn log_msg(&mut self, msg: &str) {
        let lines: Vec<String> = msg.lines().map(|l| l.to_owned()).collect();
        self.log.write().unwrap().lines.extend(lines);
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Char('\n') => self.submit_input(),
            k => self.input.write().unwrap().keypress(k),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum DemoEvent {
    Tick,
}

impl App for DemoApp {
    type UI = Shared<Linear>;
    type MyEvent = DemoEvent;
    fn widget(&self) -> Self::UI {
        self.vbox.clone()
    }
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
    fn handle_event(&mut self, event: Event<Self::MyEvent>) -> Result<(), Option<String>> {
        self.log_msg(&format!("{:?}", event));
        match event {
            Event::InputEvent(i) => match i {
                Input::Key(Key::Esc) => Err(None),
                Input::Key(Key::Alt('d')) => {
                    let pane = self.widget().render(Position::new(1, 1), self.size());
                    self.log_msg(&format!("{:#?}", pane));
                    Ok(())
                }
                Input::Key(Key::Alt('f')) => {
                    self.outputs.write().unwrap().flip();
                    Ok(())
                }
                Input::Key(k) => {
                    self.input(k);
                    Ok(())
                }
                _ => Ok(()),
            },
            Event::AppEvent(_) => {
                self.timer
                    .write()
                    .unwrap()
                    .push(format!("{}", self.counter));
                self.counter += 1;
                Ok(())
            }
        }
    }
}

fn main() {
    let mut be = Backend::new();
    let myevents = be.sender.clone();
    thread::spawn(move || loop {
        myevents.send(Event::AppEvent(DemoEvent::Tick)).unwrap();
        thread::sleep(Duration::from_millis(500));
    });
    let mut app = DemoApp::new();
    app.log_msg("Esc to exit");
    be.run_app(&mut app);
}
