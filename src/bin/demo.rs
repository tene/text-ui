extern crate text_ui;

use text_ui::pane::Pane;
use text_ui::widget::{Input,Text};

extern crate termion;

use termion::event::{Key, Event};
use termion::input::{TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::cursor::Goto;
use std::io::{Write, stdout, stdin};

struct App {
    log: Text,
    input: Input,
    height: u16,
    width: u16,
}

impl App {
    fn new() -> App {
        let size = termion::terminal_size().unwrap();
        App {
            log: Text::new(vec!()),
            input: Input::new(""),
            width: size.0,
            height: size.1,
        }
    }

    fn log_msg(&mut self) {
        self.log.push(self.input.submit());
    }

    fn render(&self) -> Vec<Pane> {
        let log = Pane::new(1, 1, self.log.render(self.width, self.height-1));
        let input = Pane::new(1, self.height, self.input.render(self.width, 1));
        vec!(log, input)
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Char('\n') => self.log_msg(),
            k => self.input.keypress(k),
        }
    }
}

pub fn render_panes(screen: &mut impl Write, panes: Vec<Pane>) {
    write!(screen, "{}", termion::clear::All).unwrap();
    for pane in panes.into_iter() {
        render_pane(&pane, screen);
    }
    screen.flush().unwrap();
}

fn render_pane(p: &Pane, screen: &mut impl Write) {
    for (i, row) in p.content.iter().enumerate() {
        write!(screen, "{}{}", Goto(p.x, p.y + i as u16), row).unwrap();
    }
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