extern crate text_ui;

use text_ui::backend::draw_pane;
use text_ui::pane::Pane;
use text_ui::widget::{Input, Text, Widget};
use text_ui::{Position, Size};

extern crate termion;

use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

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
            log: Text::new(vec![]),
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
    fn render(&self, pos: Position, size: Size) -> Pane {
        use text_ui::widget::VBox;
        let vbox = VBox {
            contents: vec![Box::new(self.log.clone()), Box::new(self.input.clone())],
        };
        vbox.render(pos, size)
    }
    fn render_children(&self, size: Size) -> Option<Vec<Pane>> {
        let log = self.log
            .render(Position::new(1, 1), Size::new(size.width, size.height - 1));
        let input = self.input
            .render(Position::new(1, size.width), Size::new(size.width, 1));
        Some(vec![log, input])
    }
    fn render_focus(&self, size: Size) -> Option<Position> {
        if let Some(pos) = self.input.render_focus(Size::new(size.width, 1)) {
            Some(pos.offset(0, size.height - 1))
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

    let mut pane = app.render(Position::new(1, 1), Size::new(app.width, app.height));
    draw_pane(&mut screen, &pane);

    for c in stdin.events() {
        let evt = c.unwrap();
        app.log.push(format!("{:?}", evt));
        match evt {
            Event::Key(Key::Esc) => break,
            Event::Key(k) => app.input(k),

            _ => {}
        }
        pane = app.render(Position::new(1, 1), Size::new(app.width, app.height));
        draw_pane(&mut screen, &pane);
    }
}
