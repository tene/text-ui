extern crate termion;
use std::io::Write;
use std::io::{stdin, stdout};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use termion::cursor::{Goto, Hide, Show};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use app::App;
use pane::Pane;
use widget::Widget;
use Event;
use Position;
use Size;

fn goto(pos: Position) -> Goto {
    Goto(pos.x, pos.y)
}

pub struct Backend<A: App> {
    pub sender: Sender<Event<A::MyEvent>>,
    receiver: Receiver<Event<A::MyEvent>>,
    size: Size,
}

impl<A> Backend<A>
where
    A: App,
    A::MyEvent: 'static,
{
    pub fn new() -> Self {
        let (width, height) = termion::terminal_size().unwrap();
        let (sender, receiver) = channel();
        let size = Size::new(width, height);
        Backend {
            sender,
            receiver,
            size,
        }
    }
    pub fn run_app(&mut self, app: &mut A) {
        let stdin = stdin();
        let mut screen =
            MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));

        draw_app(&mut screen, app, self.size);

        let inputsender = self.sender.clone();
        thread::spawn(move || {
            for c in stdin.events() {
                let evt = c.unwrap();
                match inputsender.send(Event::InputEvent(evt)) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        for e in self.receiver.iter() {
            match app.handle_event(e) {
                Ok(_) => {}
                Err(_) => break,
            }
            draw_app(&mut screen, app, self.size);
        }
    }
}

impl<A: App + 'static> Default for Backend<A> {
    fn default() -> Self {
        Self::new()
    }
}

fn render_app(app: &impl App, size: Size) -> Pane {
    let pos = Position::new(1, 1);
    app.widget().render(pos, size)
}

fn draw_app(screen: &mut impl Write, app: &impl App, size: Size) {
    let pos = Position::new(1, 1);
    let pane = render_app(app, size);
    //eprintln!("{:#?}{:?}", pane, size);
    draw_pane(screen, pos, &pane);
}

fn draw_pane(screen: &mut impl Write, pos: Position, pane: &Pane) {
    write!(screen, "{}", termion::clear::All).unwrap();
    let focus = draw_pane_helper(screen, pos, pane);
    match focus {
        Some(pos) => write!(screen, "{}{}", Show, goto(pos)),
        None => write!(screen, "{}", Hide),
    }.unwrap();
    screen.flush().unwrap();
}

fn draw_pane_helper(screen: &mut impl Write, pos: Position, pane: &Pane) -> Option<Position> {
    match &pane.content {
        Some(lines) => for (i, row) in lines.iter().enumerate() {
            write!(
                screen,
                "{}{}",
                goto(pos + pane.position.offset(0, i as u16)),
                row
            ).unwrap();
        },
        None => {}
    }
    let mut child_focus = None;
    match &pane.children {
        Some(children) => for child in children {
            let f = draw_pane_helper(screen, pos + pane.position, child);
            if f.is_some() {
                child_focus = f
            };
        },
        None => {}
    }
    if pane.focus.is_some() {
        pane.focus.map(|f| f + pane.position + pos)
    } else {
        child_focus
    }
}
