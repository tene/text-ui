extern crate text_ui;

use text_ui::app::App;
use text_ui::backend::run_app;
use text_ui::widget::{Linear, Text, TextInput, Widget};
use text_ui::{Event, Input, Position, Size};

extern crate termion;

use termion::event::Key;

use std::sync::{Arc, RwLock};

struct DemoApp {
    log: Arc<RwLock<Text>>,
    timer: Arc<RwLock<Text>>,
    input: Arc<RwLock<TextInput>>,
    vbox: Arc<RwLock<Linear>>,
    outputs: Arc<RwLock<Linear>>,
    height: u16,
    width: u16,
    counter: u32,
}

impl DemoApp {
    fn new() -> DemoApp {
        let size = termion::terminal_size().unwrap();
        let log = Arc::new(RwLock::new(Text::new(vec![])));
        let timer = Arc::new(RwLock::new(Text::new(vec![])));
        let input = Arc::new(RwLock::new(TextInput::new("")));
        let outputs = Arc::new(RwLock::new(Linear::hbox(
                    vec![Box::new(log.clone()), Box::new(timer.clone())])));
        let vbox = Arc::new(RwLock::new(Linear::vbox(vec![
                Box::new(outputs.clone()),
                Box::new(input.clone()),
            ]
        )));
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
        (*self.log)
            .write()
            .unwrap()
            .push((*self.input).write().unwrap().submit());
    }

    fn log_msg(&mut self, msg: &str) {
        let mut log = (*self.log).write().unwrap();
        let lines: Vec<String> = msg.lines().map(|l| l.to_owned()).collect();
        log.lines.extend(lines);
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Char('\n') => self.submit_input(),
            k => (*self.input).write().unwrap().keypress(k),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum DemoEvent {
    Tick,
}

impl App for DemoApp {
    type UI = Arc<RwLock<Linear>>;
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
                (*self.timer)
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
    let mut app = DemoApp::new();
    app.handle_event(Event::AppEvent(DemoEvent::Tick)).unwrap();
    app.handle_event(Event::AppEvent(DemoEvent::Tick)).unwrap();
    app.handle_event(Event::AppEvent(DemoEvent::Tick)).unwrap();
    app.handle_event(Event::AppEvent(DemoEvent::Tick)).unwrap();
    app.log_msg("Esc to exit");
    run_app(&mut app);
}
