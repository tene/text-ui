//use termion::color::{self, Color};
//use termion::cursor::{Goto, Hide, Show};
use termion::cursor::Goto;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use unicode_segmentation::UnicodeSegmentation;

use std::io::{stdin, stdout, Stdout, Write};
use std::iter::repeat;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use {
    Event, GrowthPolicy, InputEvent, Name, RenderBackend, RenderContext, RenderElement, UIEvent,
    Widget,
};

use indextree::IndexTree;

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

pub struct Block<N: Name> {
    pub lines: Vec<Line>,
    pub width: usize,
    pub height: usize,
    pub callbacks: IndexTree<N, Box<Fn(&InputEvent) -> bool>>,
}

impl<N: Name> RenderElement<N> for Block<N> {
    fn add_input_handler(&mut self, name: Option<N>, callback: Box<Fn(&InputEvent) -> bool>) {
        self.callbacks.push(name, callback)
    }
    fn handle_input(&self, name: N, event: &InputEvent) {
        for cb in self.callbacks.get_iter(&name) {
            match (cb)(event) {
                true => break,
                false => continue,
            }
        }
    }
}

impl<N: Name> Block<N> {
    pub fn new(lines: Vec<Line>, width: usize, height: usize) -> Self {
        Block {
            callbacks: IndexTree::new(),
            lines,
            width,
            height,
        }
    }
    pub fn line(text: &str, width: usize) -> Self {
        let line: Line = Span::from_str("".to_owned(), text, width).into();
        Block::new(vec![line], width, 1)
    }
    pub fn from_text(
        text: Vec<String>,
        width: usize,
        height: usize,
        should_grow: GrowthPolicy,
    ) -> Self {
        let lines = text
            .into_iter()
            .flat_map(|l| split_line_graphemes(&l, width).into_iter())
            .map(|l| Span::new("".to_owned(), l, width).into());
        let lines: Vec<Line> = match should_grow {
            GrowthPolicy::FixedSize => lines.take(height).collect(),
            GrowthPolicy::Greedy => lines
                .chain(repeat(Span::from_str("".to_owned(), "", width).into()))
                .take(height)
                .collect(),
        };
        let height = lines.len();
        Block::new(lines, width, height)
    }
    pub fn vconcat(&mut self, mut other: Self) {
        assert_eq!(self.width, other.width);
        self.lines.append(&mut other.lines);
        // map callback position
        self.callbacks.append(&mut other.callbacks);
        self.height += other.height;
    }
}

impl<N: Name> From<Line> for Block<N> {
    fn from(line: Line) -> Self {
        let width = line.width;
        let height = 1;
        let lines = vec![line];
        Block::new(lines, width, height)
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

pub struct TermionBackend {
    screen: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
    pub size: Size,
    receiver: Receiver<Event>,
    sender: Sender<Event>,
}

impl TermionBackend {
    pub fn new() -> Self {
        let screen = MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));
        let (width, height) = termion::terminal_size().unwrap();
        let size = Size::new(width as usize, height as usize);
        let (sender, receiver) = channel();
        TermionBackend {
            size,
            screen,
            sender,
            receiver,
        }
    }
    fn paint_image<N: Name>(&mut self, image: &Block<N>) {
        write!(self.screen, "{}", termion::clear::All).unwrap();
        for (i, line) in image.lines.iter().enumerate() {
            write!(self.screen, "{}", Goto(1, 1 + i as u16)).unwrap();
            for span in line.spans.iter() {
                write!(self.screen, "{}", span.text).unwrap();
            }
        }
        self.screen.flush().unwrap();
    }
    pub fn run<N: Name>(&mut self, app: impl Widget<Self, N>, focus: N) {
        // UGH disentangle input, control commands, and "app" events
        let sender = self.sender.clone();
        thread::spawn(move || {
            /*let stdin = stdin();
            let mut events = stdin.events();*/
            'outer: loop {
                for event in stdin().events() {
                    match sender.send(Event::Input(event.unwrap())) {
                        Ok(()) => continue,
                        Err(_) => break 'outer,
                    }
                }
            }
        });
        'outer: loop {
            let ui: Block<N> =
                app.render(TermionContext::new(self.size.clone(), self.sender.clone()));
            self.paint_image(&ui);
            {
                // LOL wait until an event before doing anything this is a dumb hack
                let event = self.receiver.recv().unwrap();
                let _ = self.sender.send(event);
            }
            for event in self.receiver.try_iter() {
                match event {
                    Event::UI(UIEvent::Exit) => break 'outer,
                    Event::Input(event) => {
                        for cb in ui.callbacks.get_iter(&focus) {
                            match cb(&event) {
                                true => break,
                                false => continue,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TermionContext {
    size: Size,
    sender: Sender<Event>,
}

impl TermionContext {
    fn new(size: Size, sender: Sender<Event>) -> Self {
        TermionContext { size, sender }
    }
    fn with_rows(&self, rows: usize) -> Self {
        let cols = self.size.cols;
        let size = Size { rows, cols };
        let sender = self.sender.clone();
        TermionContext { size, sender }
    }
    fn _with_cols(&self, cols: usize) -> Self {
        let rows = self.size.rows;
        let size = Size { rows, cols };
        let sender = self.sender.clone();
        TermionContext { size, sender }
    }
}

impl<N: Name> RenderContext<TermionBackend, N> for TermionContext {
    fn line(&mut self, content: &str) -> Block<N> {
        Block::line(content, self.size.cols)
    }
    fn text(&mut self, content: Vec<String>) -> Block<N> {
        Block::from_text(
            content,
            self.size.cols,
            self.size.rows,
            GrowthPolicy::Greedy,
        )
    }
    fn vbox(&mut self, widgets: Vec<&dyn Widget<TermionBackend, N>>) -> Block<N> {
        let (fixed, greedy): (
            Vec<(usize, &dyn Widget<TermionBackend, N>)>,
            Vec<(usize, &dyn Widget<TermionBackend, N>)>,
        ) = widgets
            .into_iter()
            .enumerate()
            .partition(|(_, w)| w.growth_policy().height == GrowthPolicy::FixedSize);
        let mut remaining_rows = self.size.rows;
        let cols = self.size.cols;
        let greedy_count = greedy.len();
        let mut blocks: Vec<(usize, Block<N>)> = fixed
            .into_iter()
            .map(|(i, w)| {
                let b = w.render(self.with_rows(remaining_rows));
                remaining_rows -= b.height;
                (i, b)
            })
            .collect();
        blocks.extend(greedy.into_iter().map(|(i, w)| {
            let b = w.render(self.with_rows(remaining_rows / greedy_count));
            remaining_rows -= b.height;
            (i, b)
        }));
        blocks.sort_by_key(|a| a.0);
        let init = Block::new(vec![], cols, 0);
        blocks.into_iter().fold(init, |mut acc, (_, b)| {
            acc.vconcat(b);
            acc
        })
    }
    fn event_sender(&self) -> Sender<Event> {
        self.sender.clone()
    }
}

impl<N: Name> RenderBackend<N> for TermionBackend {
    type Context = TermionContext;
    type Element = Block<N>;
}
