extern crate text_ui;

extern crate termion;

use termion::event::{Key, Event};
use termion::input::{TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::cursor::Goto;
use std::io::{Write, stdout, stdin};

struct Pane {
    x: u16,
    y: u16,
    content: Vec<String>,
}

impl Pane {
    fn render_into(&self, screen: &mut impl Write) {
        for (i, row) in self.content.iter().enumerate() {
            write!(screen, "{}{}", Goto(self.x, self.y + i as u16), row).unwrap();
        }
    }
    fn new(x: u16, y: u16, content: Vec<String>) -> Pane {
        Pane {
            x: x,
            y: y,
            content: content,
        }
    }
}

struct App {
    log: Vec<String>,
    input: String,
    height: u16,
    width: u16,
}

impl App {
    fn new() -> App {
        let size = termion::terminal_size().unwrap();
        App {
            log: vec!(),
            input: String::new(),
            width: size.0,
            height: size.1,
        }
    }

    fn log_msg(&mut self) {
        self.log.push(self.input.split_off(0));
    }

    fn render(&self) -> Vec<Pane> {
        let loglen = self.log.len();
        let lines = if loglen > self.height as usize -1 {
            self.log.clone().split_off(loglen - (self.height as usize -1))
        } else {
            self.log.clone()
        };
        let log = Pane::new(1, 1, lines);
        let input = Pane::new(1, self.height, vec!(self.input.clone()));
        vec!(log, input)
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Char('\n') => self.log_msg(),
            Key::Char(k) => self.input.push(k),
            _ => {},
        }
    }
}

fn render_panes(screen: &mut impl Write, panes: Vec<Pane>) {
    write!(screen, "{}", termion::clear::All).unwrap();
    for pane in panes.into_iter() {
        pane.render_into(screen);
    }
    screen.flush().unwrap();
}

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut app = App::new();

    let mut panes = app.render();
    render_panes(&mut screen, panes);

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Esc) => break,
            Event::Key(k) => app.input(k),
            
            _ => {}
        }
        panes = app.render();
        render_panes(&mut screen, panes);
    }
}