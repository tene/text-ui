extern crate termion;
use termion::cursor::{Goto,Show,Hide};

use pane::{Pane,Render};
use std::io::{Write};

pub fn draw_panes(screen: &mut impl Write, panes: Vec<Pane>) {
    write!(screen, "{}", termion::clear::All).unwrap();
    let mut focus = None;
    for pane in panes.into_iter() {
        draw_pane(&pane, screen);
        match pane.focus {
            Some((x,y)) => focus = Some((x+pane.x, y+pane.y)),
            None => {},
        }
    }
    match focus {
        Some((x,y)) => write!(screen, "{}{}", Show, Goto(x,y)),
        None        => write!(screen, "{}", Hide),
    }.unwrap();
    screen.flush().unwrap();
}

fn draw_pane(p: &Pane, screen: &mut impl Write) {
    for (i, row) in p.content.iter().enumerate() {
        write!(screen, "{}{}", Goto(p.x, p.y + i as u16), row).unwrap();
    }
}