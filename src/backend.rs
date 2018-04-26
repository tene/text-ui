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
use Position;
use Size;

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
        match app.handle_event(evt) {
            Ok(_) => {}
            Err(_) => break,
        }

        draw_app(&mut screen, app);
    }
}

pub fn draw_app(screen: &mut impl Write, app: &impl App) {
    let size = app.size();
    let pane = app.widget()
        .render(Position::new(1, 1), Size::new(size.width, size.height));
    draw_pane(screen, &pane);
}

pub fn draw_pane(screen: &mut impl Write, p: &Pane) {
    write!(screen, "{}", termion::clear::All).unwrap();
    let focus = draw_pane_helper(screen, p);
    match focus {
        Some(pos) => {
            //write!(screen, "{}pane: {:?} focus: {:?}", Goto(50,1), p.position, pos).unwrap();
            write!(screen, "{}{}", Show, goto(pos))
        }
        None => write!(screen, "{}", Hide),
    }.unwrap();
    screen.flush().unwrap();
}

fn draw_pane_helper(screen: &mut impl Write, p: &Pane) -> Option<Position> {
    match &p.content {
        Some(lines) => for (i, row) in lines.iter().enumerate() {
            write!(screen, "{}{}", goto(p.position.offset(0, i as u16)), row).unwrap();
        },
        None => {}
    }
    let mut child_focus = None;
    match &p.children {
        Some(children) => for child in children {
            let f = draw_pane_helper(screen, child);
            if f.is_some() {
                child_focus = f
            };
        },
        None => {}
    }
    if p.focus.is_some() {
        p.focus
    } else {
        child_focus
    }
}
