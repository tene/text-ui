//use termion::color::{self, Color};
//use termion::cursor::{Goto, Hide, Show};
use termion::cursor::{Goto, Hide, Show};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use {
    AppEvent, InputEvent, Name, RenderBackend, RenderBound, RenderElement, Size, Widget,
    WidgetEventContext, WidgetRenderContext,
};

mod element;
use self::element::Block;

#[derive(Debug, PartialEq)]
enum Event<N: Name> {
    Input(InputEvent),
    App(AppEvent<N>),
}

pub struct TermionBackend<N: Name> {
    screen: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
    pub size: Size,
    receiver: Receiver<Event<N>>,
    sender: Sender<Event<N>>,
}

impl<N: Name + 'static> TermionBackend<N> {
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
    fn paint_image(&mut self, image: &Block<N>, name: &N) {
        write!(self.screen, "{}", termion::clear::All).unwrap();
        for (i, line) in image.lines.iter().enumerate() {
            write!(self.screen, "{}", Goto(1, 1 + i as u16)).unwrap();
            for span in &line.spans {
                write!(self.screen, "{}", span.text).unwrap();
            }
        }
        if let Some(pos) = image.get_cursor(name) {
            write!(
                self.screen,
                "{}{}",
                Goto(pos.col as u16 + 1, pos.row as u16 + 1),
                Show
            );
        } else {
            write!(self.screen, "{}", Hide);
        }
        self.screen.flush().unwrap();
    }
    pub fn run(&mut self, app: impl Widget<Self, N>, mut focus: N) {
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
        let render_ctx = TermionRenderContext::new(self.size.clone().into());
        'outer: loop {
            let ui: Block<N> = render_ctx.render(&app);
            self.paint_image(&ui, &focus);
            {
                // LOL wait until an event before doing anything this is a dumb hack
                let event = self.receiver.recv().unwrap();
                let _ = self.sender.send(event);
            }
            for event in self.receiver.try_iter() {
                match event {
                    Event::App(AppEvent::Exit) => break 'outer,
                    Event::App(AppEvent::SetFocus(f)) => focus = f,
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

impl<N: Name + 'static> Default for TermionBackend<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct TermionRenderContext {
    bound: RenderBound,
}

impl TermionRenderContext {
    fn new(bound: RenderBound) -> Self {
        Self { bound }
    }
    /*fn with_rows(&self, rows: usize) -> Self {
        let mut bound = self.bound.clone();
        bound.height = Some(rows);
        Self::new(bound)
    }
    fn with_cols(&self, cols: usize) -> Self {
        let mut bound = self.bound.clone();
        bound.width = Some(cols);
        Self::new(bound)
    }*/
}

impl<N: Name> WidgetRenderContext<TermionBackend<N>, N> for TermionRenderContext {
    fn render(&self, widget: &Widget<TermionBackend<N>, N>) -> Block<N> {
        widget.render(self.clone())
    }
    fn render_sized(&self, bound: RenderBound, widget: &Widget<TermionBackend<N>, N>) -> Block<N> {
        let block = Self::new(bound).render(widget);
        let size = block.size();
        if let Some(width) = bound.width {
            //assert_eq!(width, size.cols);
            if width != size.cols {
                panic!(
                    "bad block width!\nwidget: {:#?}\nbound: {:?}\nblock: {:#?}",
                    widget, bound, block
                );
            }
        }
        if let Some(height) = bound.height {
            //assert_eq!(height, size.rows);
            if height != size.rows {
                panic!(
                    "bad block height!\nwidget: {:#?}\nbound: {:?}\nblock: {:#?}",
                    widget, bound, block
                );
            }
        }
        block
    }
    fn bound(&self) -> RenderBound {
        self.bound.clone()
    }

    fn line(&mut self, content: &str) -> Block<N> {
        Block::line(content, self.bound)
    }
    fn text(&mut self, content: Vec<String>) -> Block<N> {
        Block::from_text(content, self.bound)
    }
}

pub struct TermionEventContext<N: Name> {
    sender: Sender<Event<N>>,
}

impl<N: Name> TermionEventContext<N> {
    fn new(sender: Sender<Event<N>>) -> Self {
        Self { sender }
    }
}

impl<N: Name> WidgetEventContext<TermionBackend<N>, N> for TermionEventContext<N> {
    fn send_event(&self, event: AppEvent<N>) {
        let _ = self.sender.send(Event::App(event));
    }
}

impl<N: Name> RenderBackend<N> for TermionBackend<N> {
    type RenderContext = TermionRenderContext;
    type EventContext = TermionEventContext<N>;
    type Element = Block<N>;
}
