extern crate text_ui;
use text_ui::app::App;
use text_ui::backend::Backend;
use text_ui::widget::{shared, Linear, Readline, Shared, Text, TextInput};
use text_ui::{Event, Input, Key};

use std::thread;
use std::time::Duration;

struct DemoApp {
    log: Shared<Text>,
    timer: Shared<Text>,
    input: Shared<TextInput>,
    readline: Shared<Readline>,
    vbox: Shared<Linear>,
    outputs: Shared<Linear>,
    counter: u32,
}

impl DemoApp {
    fn new() -> DemoApp {
        let log = shared(Text::new(vec![]));
        let timer = shared(Text::new(vec![]));
        let mut rl = Readline::new();
        rl.width(80);
        rl.update("test1234test1234", 10);
        let readline = shared(rl);
        let input = shared(TextInput::new(""));
        let mut outbox = Linear::hbox();
        outbox.push(&log);
        outbox.push(&timer);
        let outputs = shared(outbox);
        let mut mainbox = Linear::vbox();
        mainbox.push(&outputs);
        mainbox.push(&readline);
        mainbox.push(&input);
        let vbox = shared(mainbox);
        DemoApp {
            log,
            input,
            timer,
            readline,
            vbox,
            outputs,
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
    fn handle_event(&mut self, event: Event<Self::MyEvent>) -> Result<(), Option<String>> {
        self.log_msg(&format!("{:?}", event));
        match event {
            Event::InputEvent(i) => match i {
                Input::Key(Key::Esc) => Err(None),
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
