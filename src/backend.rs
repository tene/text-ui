extern crate termion;
use termion::cursor::{Goto, Hide, Show};

use pane::Pane;
use std::io::Write;
use Position;

fn goto(pos: Position) -> Goto {
    Goto(pos.x, pos.y)
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
