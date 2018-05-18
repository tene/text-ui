extern crate termion;
use std::io::Write;
use std::io::{stdin, stdout};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub use termion::color::{self, Color};
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
    Goto(pos.x as u16, pos.y as u16)
}

pub struct Backend<A: App> {
    pub sender: Sender<A::MyEvent>,
    receiver: Receiver<A::MyEvent>,
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
        let size = Size::new(width as usize, height as usize);
        Backend {
            sender,
            receiver,
            size,
        }
    }
    pub fn run_app(mut self, app: &mut A) {
        let mut screen =
            MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));

        draw_app(&mut screen, app, self.size);

        let (event_sender, event_receiver) = channel();
        let inputsender = event_sender.clone();
        thread::spawn(move || {
            let stdin = stdin();
            for c in stdin.events() {
                let evt = c.unwrap();
                match inputsender.send(Event::InputEvent(evt)) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        let app_events = self.receiver.into_iter();

        thread::spawn(move || {
            for a in app_events {
                match event_sender.send(Event::AppEvent(a)) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        for e in event_receiver.iter() {
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
    let pos = Position::new(0, 0);
    app.widget().render(pos, size)
}

fn draw_app(screen: &mut impl Write, app: &impl App, size: Size) {
    let pos = Position::new(1, 1);
    let pane = render_app(app, size);
    //eprintln!("{:#?}{:?}", pane, size);
    draw_pane(screen, app, pos, &pane);
}

fn draw_pane(screen: &mut impl Write, app: &impl App, pos: Position, pane: &Pane) {
    write!(screen, "{}", termion::clear::All).unwrap();
    let focus = draw_pane_helper(screen, app, pos, pane);
    match focus {
        Some(fpos) => write!(screen, "{}{}", Show, goto(fpos)),
        None => write!(screen, "{}", Hide),
    }.unwrap();
    screen.flush().unwrap();
}

fn style_color(app: &impl App, s: &Option<String>) -> String {
    match s {
        Some(name) => match app.style(&name) {
            (Some(fg), Some(bg)) => format!("{}{}", color::Fg(&*fg), color::Bg(&*bg)),
            (Some(fg), _) => format!("{}", color::Fg(&*fg)),
            (_, Some(bg)) => format!("{}", color::Bg(&*bg)),
            _ => "".to_string(),
        },
        None => "".to_owned(),
    }
}

fn style_reset(app: &impl App, s: &Option<String>) -> String {
    match s {
        Some(name) => match (app.style(&name), app.default_style()) {
            ((Some(_fg), Some(_bg)), def) => format!("{}{}", color::Fg(&*def), color::Bg(&*def)),
            ((Some(_fg), _), def) => format!("{}", color::Fg(&*def)),
            ((_, Some(bg)), def) => format!("{}", color::Bg(&*def)),
            _ => "".to_string(),
        },
        None => "".to_owned(),
    }
}

fn draw_pane_helper(
    screen: &mut impl Write,
    app: &impl App,
    pos: Position,
    pane: &Pane,
) -> Option<Position> {
    match &pane.content {
        Some(lines) => {
            write!(screen, "{}", style_color(app, &pane.style));
            let blank_row = format!("{: <width$}", " ", width = pane.size.width);
            for i in 0..pane.size.height {
                let row = lines.get(i).unwrap_or(&blank_row);
                write!(
                    screen,
                    "{}{: <width$}",
                    goto(pos + pane.position.offset(0, i)),
                    row,
                    width = pane.size.width
                ).unwrap();
            }
            write!(screen, "{}", style_reset(app, &pane.style));
        }
        None => {}
    }
    let mut child_focus = None;
    match &pane.children {
        Some(children) => for child in children {
            let f = draw_pane_helper(screen, app, pos + pane.position, child);
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
