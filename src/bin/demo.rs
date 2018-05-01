extern crate text_ui;
use text_ui::app::App;
use text_ui::backend::Backend;
use text_ui::widget::{shared, DbgDump, Line, Linear, Readline, Shared, Text};
use text_ui::{Event, Input, Key};

use std::thread;
use std::time::Duration;

struct DemoApp {
    log: Shared<Text>,
    side: Shared<Linear>,
    readline: Shared<Readline>,
    vbox: Shared<Linear>,
    outputs: Shared<Linear>,
    show_side: bool,
}

impl DemoApp {
    fn new() -> DemoApp {
        let log = shared(Text::new(vec![]));
        let rl = Readline::new();
        let readline = shared(rl);
        let mut sidebox = Linear::vbox();
        let dbg = shared(DbgDump::new(&readline));
        sidebox.push(&dbg);
        let side = shared(sidebox);
        let mut outbox = Linear::hbox();
        outbox.push(&log);
        outbox.push(&shared(Line::vertical()));
        outbox.push(&side);
        let outputs = shared(outbox);
        let mut mainbox = Linear::vbox();
        mainbox.push(&outputs);
        mainbox.push(&shared(Line::horizontal()));
        mainbox.push(&readline);
        let vbox = shared(mainbox);
        DemoApp {
            log,
            side,
            readline,
            vbox,
            outputs,
            show_side: true,
        }
    }

    fn toggle_side(&mut self) {
        let mut outputs = self.outputs.write().unwrap();
        match self.show_side {
            true => {
                self.show_side = false;
                outputs.contents.truncate(0);
                outputs.push(&self.log);
            }
            false => {
                self.show_side = true;
                outputs.contents.truncate(0);
                outputs.push(&self.log);
                outputs.push(&self.side);
            }
        }
    }

    fn submit_input(&mut self) {
        let mut rl = self.readline.write().unwrap();
        let line = rl.finalize();
        self.log
            .write()
            .unwrap()
            .push(line);
    }

    fn log_msg(&mut self, msg: &str) {
        let lines: Vec<String> = msg.lines().map(|l| l.to_owned()).collect();
        self.log.write().unwrap().lines.extend(lines);
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Char('\n') => self.submit_input(),
            Key::Alt('\r') => self.readline.write().unwrap().process_key(Key::Char('\n')),
            k => {
                self.log_msg(&format!("{:?}", k));
                let mut rl = self.readline.write().unwrap();
                rl.process_key(k);
            }
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
        match event {
            Event::InputEvent(i) => match i {
                Input::Key(Key::Esc) => Err(None),
                Input::Key(Key::Alt('t')) => {
                    self.toggle_side();
                    Ok(())
                }
                Input::Key(k) => {
                    self.input(k);
                    Ok(())
                }
                _ => {
                    self.log_msg(&format!("{:?}", i));
                    Ok(())
                }
            },
            Event::AppEvent(_) => Ok(()),
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
