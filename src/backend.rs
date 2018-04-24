extern crate termion;
use termion::cursor::{Goto,Show,Hide};

use pane::{Pane};
use std::io::{Write};

pub fn draw_pane(screen: &mut impl Write, p: &Pane) {
    write!(screen, "{}", termion::clear::All).unwrap();
    draw_pane_helper(screen, p);
    match &p.focus {
        Some((x,y)) => write!(screen, "{}{}", Show, Goto(x+p.x, y+p.y)),
        None        => write!(screen, "{}", Hide),
    }.unwrap();
    screen.flush().unwrap();
}

fn draw_pane_helper(screen: &mut impl Write, p: &Pane) {
    match &p.content {
        Some(lines) => for (i, row) in lines.iter().enumerate() {
            write!(screen, "{}{}", Goto(p.x, p.y + i as u16), row).unwrap();
        },
        None => {},
    }
    match &p.children {
        Some(children) => for child in children {
            draw_pane_helper(screen, child)
        },
        None => {},
    }
}