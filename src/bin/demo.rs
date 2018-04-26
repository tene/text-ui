extern crate text_ui;

use text_ui::app::App;
use text_ui::backend::{run_app};
use text_ui::widget::{TextInput, Text, VBox};
use text_ui::{Event, Size, Input};

extern crate termion;

use termion::event::{Key};

use std::sync::{Arc, RwLock};

struct DemoApp {
    log: Arc<RwLock<Text>>,
    input: Arc<RwLock<TextInput>>,
    vbox: Arc<RwLock<VBox>>,
    height: u16,
    width: u16,
}

impl DemoApp {
    fn new() -> DemoApp {
        let size = termion::terminal_size().unwrap();
        let log = Arc::new(RwLock::new(Text::new(vec![])));
        let input = Arc::new(RwLock::new(TextInput::new("")));
        let vbox = Arc::new(RwLock::new(VBox {
            contents: vec![Box::new(log.clone()), Box::new(input.clone())],
        }));
        DemoApp {
            log: log,
            input: input,
            vbox: vbox,
            width: size.0,
            height: size.1,
        }
    }

    fn submit_input(&mut self) {
        (*self.log)
            .write()
            .unwrap()
            .push((*self.input).write().unwrap().submit());
    }

    fn log_msg(&mut self, msg: &str) {
        (*self.log).write().unwrap().push(msg.to_owned());
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
    type UI = Arc<RwLock<VBox>>;
    type MyEvent = DemoEvent;
    fn widget(&self) -> Self::UI {
        self.vbox.clone()
    }
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
    fn handle_event(&mut self, event: Event<Self::MyEvent>) -> Result<(), Option<String>> {
        match event {
            Event::InputEvent(i) => {
                self.log_msg(&format!("{:?}", i));
                match i {
                    Input::Key(Key::Esc) => Err(None),
                    Input::Key(k) => {
                        self.input(k);
                        Ok(())
                    }
                    _ => Ok(()),
                }
            },
            Event::AppEvent(e) => {
                self.log_msg(&format!("{:?}", e));
                Ok(())
            }
        }
    }
}

fn main() {
    let mut app = DemoApp::new();
    app.log_msg("Esc to exit");
    run_app(&mut app);
}
