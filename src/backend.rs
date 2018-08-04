//use termion::color::{self, Color};
//use termion::cursor::{Goto, Hide, Show};
use termion::cursor::Goto;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use {
    Event, GrowthPolicy, Name, RenderBackend, UIEvent, Widget, WidgetEventContext,
    WidgetRenderContext,
};

mod element;
use self::element::Block;

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
            for span in &line.spans {
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
        let event_ctx = TermionEventContext::new(self.sender.clone());
        'outer: loop {
            let ui: Block<N> = app.render(TermionRenderContext::new(self.size.clone()));
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
                            use ShouldPropagate::*;
                            match cb(&event_ctx, &event) {
                                Stop => break,
                                Continue => continue,
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Default for TermionBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct TermionRenderContext {
    size: Size,
}

impl TermionRenderContext {
    fn new(size: Size) -> Self {
        Self { size }
    }
    fn with_rows(&self, rows: usize) -> Self {
        let cols = self.size.cols;
        let size = Size { rows, cols };
        Self::new(size)
    }
    fn _with_cols(&self, cols: usize) -> Self {
        let rows = self.size.rows;
        let size = Size { rows, cols };
        Self::new(size)
    }
}

// XXX TODO These should be generic default impls
impl<N: Name> WidgetRenderContext<TermionBackend, N> for TermionRenderContext {
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
            }).collect();
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
}

pub struct TermionEventContext {
    sender: Sender<Event>,
}

impl TermionEventContext {
    fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }
}

impl<N: Name> WidgetEventContext<TermionBackend, N> for TermionEventContext {
    fn send_event(&self, event: UIEvent) {
        let _ = self.sender.send(Event::UI(event));
    }
}

impl<N: Name> RenderBackend<N> for TermionBackend {
    type RenderContext = TermionRenderContext;
    type EventContext = TermionEventContext;
    type Element = Block<N>;
}
