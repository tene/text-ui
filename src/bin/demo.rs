extern crate text_ui;

use text_ui::backend::draw_pane;
use text_ui::pane::Pane;
use text_ui::widget::{Input, Text, VBox, Widget};
use text_ui::{Position, Size};

extern crate termion;

use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::sync::{Arc, RwLock};

struct App {
    log: Arc<RwLock<Text>>,
    input: Arc<RwLock<Input>>,
    vbox: VBox,
    height: u16,
    width: u16,
}

impl App {
    fn new() -> App {
        let size = termion::terminal_size().unwrap();
        let log = Arc::new(RwLock::new(Text::new(vec![])));
        let input = Arc::new(RwLock::new(Input::new("")));
        let vbox = VBox {
            contents: vec![Box::new(log.clone()), Box::new(input.clone())],
        };
        App {
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

impl Widget for App {
    fn render(&self, pos: Position, size: Size) -> Pane {
        self.vbox.render(pos, size)
    }
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        self.vbox.render_children(size)
    }
    fn render_focus(&self, size: Size) -> Option<Position> {
        self.vbox.render_focus(size)
    }
}

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut app = App::new();
    app.log_msg("Esc to exit");

    let mut pane = app.render(Position::new(1, 1), Size::new(app.width, app.height));
    draw_pane(&mut screen, &pane);

    for c in stdin.events() {
        let evt = c.unwrap();
        app.log_msg(&format!("{:?}", evt));
        match evt {
            Event::Key(Key::Esc) => break,
            Event::Key(k) => app.input(k),

            _ => {}
        }
        pane = app.render(Position::new(1, 1), Size::new(app.width, app.height));
        draw_pane(&mut screen, &pane);
    }
}
