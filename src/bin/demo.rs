extern crate text_ui;

use text_ui::backend::draw_app;
use text_ui::widget::{Input, Text, VBox};
use text_ui::{Size};
use text_ui::app::App;

extern crate termion;

use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::sync::{Arc, RwLock};

struct DemoApp {
    log: Arc<RwLock<Text>>,
    input: Arc<RwLock<Input>>,
    vbox: Arc<RwLock<VBox>>,
    height: u16,
    width: u16,
}

impl DemoApp {
    fn new() -> DemoApp {
        let size = termion::terminal_size().unwrap();
        let log = Arc::new(RwLock::new(Text::new(vec![])));
        let input = Arc::new(RwLock::new(Input::new("")));
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

impl App<Arc<RwLock<VBox>>> for DemoApp {
    fn widget(&self) -> Arc<RwLock<VBox>> {
        self.vbox.clone()
    }
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut app = DemoApp::new();
    app.log_msg("Esc to exit");

    draw_app(&mut screen, &app);

    for c in stdin.events() {
        let evt = c.unwrap();
        app.log_msg(&format!("{:?}", evt));
        match evt {
            Event::Key(Key::Esc) => break,
            Event::Key(k) => app.input(k),

            _ => {}
        }
        draw_app(&mut screen, &app);
    }
}
