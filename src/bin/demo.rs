extern crate text_ui;

use text_ui::pane::{Pane, Widget};
use text_ui::widget::{Input,Text};
use text_ui::backend::draw_pane;

extern crate termion;

use termion::event::{Key, Event};
use termion::input::{TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{stdout, stdin};

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

    fn input(&mut self, key: Key) {
        match key {
            Key::Char('\n') => self.log_msg(),
            k => self.input.keypress(k),
        }
    }
}

impl Widget for App {
    fn render_children(&self, width: u16, height: u16) -> Option<Vec<Pane>> {
        let log = self.log.render(1, 1, width, height-1);
        let input = self.input.render(1, height ,width, 1);
        Some(vec!(log,input))
    }
    fn render_focus(&self, width: u16, height: u16) -> Option<(u16, u16)> {
        if let Some((col, _)) = self.input.render_focus(width, height) {
            Some((col, height))
        } else {
            None
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut app = App::new();
    app.log.push("Esc to exit".to_string());

    let mut pane = app.render(1, 1, app.width, app.height);
    draw_pane(&mut screen, &pane);

    for c in stdin.events() {
        let evt = c.unwrap();
        app.log.push(format!("{:?}", evt));
        match evt {
            Event::Key(Key::Esc) => break,
            Event::Key(k) => app.input(k),
            
            _ => {}
        }
        pane = app.render(1, 1, app.width, app.height);
        draw_pane(&mut screen, &pane);
    }
}