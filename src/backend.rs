//use termion::color::{self, Color};
//use termion::cursor::{Goto, Hide, Show};
use termion::cursor::Goto;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use unicode_segmentation::UnicodeSegmentation;

use std::collections::VecDeque;
use std::io::{stdin, stdout, Stdout, Write};
use std::iter::repeat;

use {Bound, Event, RenderBackend, UIEvent, Widget};

pub fn split_line_graphemes(line: &str, width: usize) -> Vec<String> {
    let mut letters: Vec<&str> = UnicodeSegmentation::graphemes(line, true).collect();
    let len = letters.len();
    match len % width {
        0 => {}
        n => letters.resize(len + (width - n), " "),
    };
    letters
        .chunks(width)
        .map(|ls| ls.concat())
        .collect::<Vec<String>>()
}

#[derive(Debug, Clone)]
pub struct Span {
    pub attr: String,
    pub text: String,
    pub width: usize,
}

impl Span {
    pub fn new(attr: String, text: String, width: usize) -> Self {
        Span { attr, text, width }
    }

    pub fn from_str(attr: String, text: &str, width: usize) -> Self {
        let text = UnicodeSegmentation::graphemes(text, true)
            .chain(repeat(" "))
            .take(width)
            .collect::<String>();
        Span { attr, text, width }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub spans: Vec<Span>,
    pub width: usize,
}

impl Line {
    pub fn new(spans: Vec<Span>, width: usize) -> Self {
        Line { spans, width }
    }
}

impl From<Span> for Line {
    fn from(span: Span) -> Self {
        let width = span.width;
        let spans = vec![span];
        Line { spans, width }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub lines: Vec<Line>,
    pub width: usize,
    pub height: usize,
}

impl Block {
    pub fn new(lines: Vec<Line>, width: usize, height: usize) -> Self {
        Block {
            lines,
            width,
            height,
        }
    }
    pub fn from_text(text: Vec<String>, width: usize, height: usize, should_grow: Bound) -> Self {
        let lines = text
            .into_iter()
            .flat_map(|l| split_line_graphemes(&l, width).into_iter())
            .map(|l| Span::new("".to_owned(), l, width).into());
        let lines: Vec<Line> = match should_grow {
            Bound::Fixed => lines.take(height).collect(),
            Bound::Greedy => lines
                .chain(repeat(Span::from_str("".to_owned(), "", width).into()))
                .take(height)
                .collect(),
        };
        let height = lines.len();
        Block {
            lines,
            width,
            height,
        }
    }
    pub fn vconcat(&mut self, mut other: Self) {
        assert_eq!(self.width, other.width);
        self.lines.append(&mut other.lines);
        self.height += other.height;
    }
}

impl From<Line> for Block {
    fn from(line: Line) -> Self {
        let width = line.width;
        let height = 1;
        let lines = vec![line];
        Block {
            width,
            height,
            lines,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub cols: usize,
    pub rows: usize,
}

impl Size {
    pub fn new(cols: usize, rows: usize) -> Self {
        Size { cols, rows }
    }
}

pub struct Backend {
    screen: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
    pub size: Size,
}

impl Backend {
    pub fn new() -> Self {
        let screen = MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));
        let (width, height) = termion::terminal_size().unwrap();
        let size = Size::new(width as usize, height as usize);
        Backend { size, screen }
    }
    fn paint_image(&mut self, image: Block) {
        write!(self.screen, "{}", termion::clear::All).unwrap();
        for (i, line) in image.lines.into_iter().enumerate() {
            write!(self.screen, "{}", Goto(1, 1 + i as u16)).unwrap();
            for span in line.spans.into_iter() {
                write!(self.screen, "{}", span.text).unwrap();
            }
        }
        self.screen.flush().unwrap();
    }
    pub fn run<B>(&mut self, mut app: impl Widget<B>)
    where
        B: RenderBackend,
    {
        let stdin = stdin();
        let mut events = stdin.events();
        'outer: loop {
            //let ui = app.render();
            let ui: Block = unimplemented!();
            self.paint_image(ui);
            let mut event_buf: VecDeque<Event> = VecDeque::new();
            match events.next() {
                Some(event) => event_buf.push_back(Event::Input(event.unwrap())),
                None => break,
            }
            while !event_buf.is_empty() {
                let event = event_buf.pop_front().unwrap();
                match app.handle_event(&event) {
                    None => {}
                    Some(Event::UI(UIEvent::Exit)) => break 'outer,
                    Some(e) => event_buf.push_back(e),
                };
            }
        }
    }
}
