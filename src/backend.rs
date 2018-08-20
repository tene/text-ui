//use termion::color::{self, Color};
//use termion::cursor::{Goto, Hide, Show};
use termion::cursor::{Goto, Hide, Show};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use signal_hook::iterator::Signals;

use std::io::{stdin, stdout, Stdout, Write};
use std::iter::repeat;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use {
    App, AppEvent, Fragment, InputEvent, Name, RenderBackend, RenderBound, RenderElement, Size,
    Widget, WidgetEventContext, WidgetRenderContext,
};

mod element;
use self::element::{Block, Line};

#[derive(Debug, PartialEq)]
enum Event<N: Name> {
    Input(InputEvent),
    App(AppEvent<N>),
    Resize,
}

pub struct TermionBackend<N: Name> {
    screen: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
    pub size: Size,
    receiver: Receiver<Event<N>>,
    sender: Sender<Event<N>>,
    last_frame: Vec<Line>,
}

impl<N: Name + 'static> TermionBackend<N> {
    pub fn new() -> Self {
        let screen = MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));
        let (width, height) = termion::terminal_size().unwrap();
        let size = Size::new(width as usize, height as usize);
        let (sender, receiver) = channel();
        let last_frame = repeat(Line::blank(width as usize))
            .take(height as usize)
            .collect();
        TermionBackend {
            size,
            screen,
            sender,
            receiver,
            last_frame,
        }
    }
    fn paint_image(&mut self, image: &Block<N>, name: &N) {
        //write!(self.screen, "{}", termion::clear::All).unwrap();
        for (i, (new_line, last_line)) in image.lines.iter().zip(self.last_frame.iter()).enumerate()
        {
            if new_line != last_line {
                write!(self.screen, "{}", Goto(1, 1 + i as u16)).unwrap();
                for span in &new_line.spans {
                    write!(self.screen, "{}{}", span.attr, span.text).unwrap();
                }
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
        self.last_frame = image.lines.clone();
    }
    pub fn run(&mut self, app: &mut impl App<Self, N>, mut focus: N) {
        let input_sender = self.sender.clone();
        thread::spawn(move || {
            /*let stdin = stdin();
            let mut events = stdin.events();*/
            'outer: loop {
                for event in stdin().events() {
                    match input_sender.send(Event::Input(event.unwrap())) {
                        Ok(()) => continue,
                        Err(_) => break 'outer,
                    }
                }
            }
        });
        let signal_sender = self.sender.clone();
        let signals = Signals::new(&[libc::SIGWINCH]).expect("Failed to register signal handler");
        thread::spawn(move || 'outer: loop {
            for signal in &signals {
                let event = match signal {
                    libc::SIGWINCH => Event::Resize,
                    _ => continue,
                };
                match signal_sender.send(event) {
                    Ok(()) => continue,
                    Err(_) => break 'outer,
                }
            }
        });
        let event_ctx = TermionEventContext::new(self.sender.clone());
        write!(self.screen, "{}", termion::clear::All).unwrap();
        'outer: loop {
            let render_ctx = TermionRenderContext::new(self.size.into());
            let ui: Block<N> = render_ctx.render(app);
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
                    Event::Resize => {
                        let (width, height) = termion::terminal_size().unwrap();
                        let size = Size::new(width as usize, height as usize);
                        self.size = size;
                        self.last_frame
                            .resize(height as usize, Line::blank(width as usize));
                        app.handle_resize(size);
                    }
                    Event::Input(event) => {
                        use ShouldPropagate::*;
                        match app.handle_input(&event_ctx, &event) {
                            Stop => break,
                            Continue => {}
                        };
                        match event {
                            InputEvent::Key(k) => {
                                for cb in ui.callbacks.get_iter(&focus) {
                                    match cb(&event_ctx, k) {
                                        Stop => break,
                                        Continue => continue,
                                    }
                                }
                            }
                            InputEvent::Mouse(_) => {}
                            InputEvent::Unsupported(_) => {}
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
    fn with_bound(&self, bound: RenderBound) -> Self {
        Self::new(bound)
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
        self.bound
    }

    fn line<F: Into<Fragment>>(&self, content: F) -> Block<N> {
        let fragment: Fragment = content.into();
        Block::from_fragment(&fragment, self.bound.constrain_height(1))
    }
    fn text<F: Into<Fragment>>(&self, content: F) -> Block<N> {
        let fragment: Fragment = content.into();
        Block::from_fragment(&fragment, self.bound)
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
