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
    App, AppEvent, EventContext, InputEvent, Name, RenderBackend, RenderContext, Size, TextBlock,
    TextLine, Widget,
};

mod element;

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
    last_frame: Vec<String>,
}

impl<N: Name + 'static> TermionBackend<N> {
    pub fn new() -> Self {
        let screen = MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));
        let (width, height) = termion::terminal_size().unwrap();
        let size = Size::new(width as usize, height as usize);
        let (sender, receiver) = channel();
        let last_frame = repeat("".to_owned()).take(height as usize).collect();
        TermionBackend {
            size,
            screen,
            sender,
            receiver,
            last_frame,
        }
    }
    pub fn run(&mut self, app: &mut impl App<N>, mut focus: N) {
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
        let event_ctx = EventContext::new(self.sender.clone());
        write!(self.screen, "{}", termion::clear::All).unwrap();
        'outer: loop {
            let render_ctx = RenderContext::new(self.size.into());
            let ui: TextBlock<N> = app.render(render_ctx);
            self.paint_frame(&ui, &focus);
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
                        self.last_frame.resize(height as usize, "".to_owned());
                        app.handle_resize(size);
                    }
                    Event::Input(event) => {
                        use ShouldPropagate::*;
                        match app.handle_input(&event_ctx, &event) {
                            Stop => break,
                            Continue => {}
                        };
                        match event {
                            InputEvent::Key(k) => ui.handle_key(&event_ctx, &focus, k),
                            InputEvent::Mouse(m) => ui.handle_mouse(&event_ctx, m),
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

impl<N: Name> RenderBackend<N> for TermionBackend<N> {
    fn paint_frame(&mut self, image: &TextBlock<N>, focus: &N) {
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
        if let Some(pos) = image.get_cursor(focus) {
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
}
