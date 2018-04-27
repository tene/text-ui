extern crate termion;
use std::io::{stdin, stdout};
use termion::cursor::{Goto, Hide, Show};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use app::App;
use pane::Pane;
use std::io::Write;
use widget::Widget;
use Event;
use Position;

fn goto(pos: Position) -> Goto {
    Goto(pos.x, pos.y)
}

// TODO Next step, receive other App events over a chan
pub fn run_app(app: &mut impl App) {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    draw_app(&mut screen, app);

    for c in stdin.events() {
        let evt = c.unwrap();
        match app.handle_event(Event::InputEvent(evt)) {
            Ok(_) => {}
            Err(_) => break,
        }

        draw_app(&mut screen, app);
    }
}

pub fn render_app(app: &impl App) -> Pane {
    let size = app.size();
    let pos = Position::new(1, 1);
    app.widget().render(pos, size)
}

pub fn draw_app(screen: &mut impl Write, app: &impl App) {
    let pos = Position::new(1, 1);
    let pane = render_app(app);
    draw_pane(screen, pos, &pane);
}

pub fn draw_pane(screen: &mut impl Write, pos: Position, pane: &Pane) {
    write!(screen, "{}", termion::clear::All).unwrap();
    let focus = draw_pane_helper(screen, pos, pane);
    match focus {
        Some(pos) => write!(screen, "{}{}", Show, goto(pos)),
        None => write!(screen, "{}", Hide),
    }.unwrap();
    screen.flush().unwrap();
}

fn draw_pane_helper(screen: &mut impl Write, pos: Position, pane: &Pane) -> Option<Position> {
    match &pane.content {
        Some(lines) => for (i, row) in lines.iter().enumerate() {
            write!(
                screen,
                "{}{}",
                goto(pos + pane.position.offset(0, i as u16)),
                row
            ).unwrap();
        },
        None => {}
    }
    let mut child_focus = None;
    match &pane.children {
        Some(children) => for child in children {
            let f = draw_pane_helper(screen, pos, child);
            if f.is_some() {
                child_focus = f
            };
        },
        None => {}
    }
    if pane.focus.is_some() {
        pane.focus.map(|f| f + pane.position + pos)
    } else {
        child_focus
    }
}
