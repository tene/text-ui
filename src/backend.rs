/*use termion::color::{self, Color};
use termion::cursor::{Goto, Hide, Show};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::io::{stdin, stdout};

pub struct Span {
    pub attr: String,
    pub text: String,
    pub width: usize,
}

pub struct Line {
    pub spans: Vec<Span>,
    pub width: usize,
}

pub struct Block {
    pub lines: Vec<Line>,
    pub width: usize,
    pub height: usize,
}

pub struct Backend {}

impl Backend {
    pub fn run() {
        let mut screen =
            MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));
    }
}
*/
