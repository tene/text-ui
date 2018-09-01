//use termion::color::{self, Color};
//use termion::cursor::{Goto, Hide, Show};
use termion::color as termion_color;
use termion::color::Color as TermColor;
use termion::cursor::{Goto, Hide, Show};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use signal_hook::iterator::Signals;

use std::fmt;
use std::io::{stdin, stdout, Stdout, Write};
use std::iter::repeat;
use std::thread;

use {BackendContext, Color, Frame, Name, RenderBackend, Size};

pub struct TermionBackend {
    screen: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
    pub size: Size,
    last_frame: Vec<String>,
}

impl RenderBackend for TermionBackend {
    fn new<N: Name + 'static>(ctx: BackendContext<N>) -> Self {
        let mut screen =
            MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode().unwrap()));
        write!(screen, "{}", termion::clear::All).unwrap();
        let (width, height) = termion::terminal_size().unwrap();
        let size = Size::new(width as usize, height as usize);
        let last_frame = repeat("".to_owned()).take(height as usize).collect();
        let ctx2 = ctx.clone();
        thread::spawn(move || {
            /*let stdin = stdin();
            let mut events = stdin.events();*/
            'outer: loop {
                for event in stdin().events() {
                    match ctx.send_input(event.unwrap()) {
                        Ok(()) => continue,
                        Err(_) => break 'outer,
                    }
                }
            }
        });
        thread::spawn(move || 'outer: loop {
            let signals =
                Signals::new(&[libc::SIGWINCH]).expect("Failed to register signal handler");
            for signal in &signals {
                match signal {
                    libc::SIGWINCH => {
                        let (width, height) = termion::terminal_size().unwrap();
                        match ctx2.resize(Size::new(width as usize, height as usize)) {
                            Ok(()) => continue,
                            Err(_) => break 'outer,
                        }
                    }
                    _ => continue,
                };
            }
        });

        TermionBackend {
            size,
            screen,
            last_frame,
        }
    }
    fn paint_frame(&mut self, frame: Frame) {
        //write!(self.screen, "{}", termion::clear::All).unwrap();
        let new_frame: Vec<String> = frame
            .image
            .into_iter()
            .map(|chunks| {
                chunks
                    .into_iter()
                    .fold(String::new(), |mut l, (fg, bg, text, _)| {
                        l.push_str(&format!(
                            "{}{}",
                            termion_color::Fg(fg),
                            termion_color::Bg(bg)
                        ));
                        l.push_str(text);
                        l
                    })
            }).collect();
        for (i, (new_line, last_line)) in new_frame.iter().zip(self.last_frame.iter()).enumerate() {
            if new_line != last_line {
                write!(self.screen, "{}{}", Goto(1, 1 + i as u16), new_line).unwrap();
            }
        }
        if let Some(pos) = frame.focus {
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
        self.last_frame = new_frame;
    }
    fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.last_frame
            .resize(new_size.rows as usize, "".to_owned());
    }
    fn size(&self) -> Size {
        self.size
    }
}

// This is redundant because termion colors are not sized, and I didn't want to add a box everywhere
impl TermColor for Color {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::LightBlack => termion_color::LightBlack.write_fg(f),
            Color::LightBlue => termion_color::LightBlue.write_fg(f),
            Color::LightCyan => termion_color::LightCyan.write_fg(f),
            Color::LightGreen => termion_color::LightGreen.write_fg(f),
            Color::LightMagenta => termion_color::LightMagenta.write_fg(f),
            Color::LightRed => termion_color::LightRed.write_fg(f),
            Color::LightWhite => termion_color::LightWhite.write_fg(f),
            Color::LightYellow => termion_color::LightYellow.write_fg(f),
            Color::Black => termion_color::Black.write_fg(f),
            Color::Blue => termion_color::Blue.write_fg(f),
            Color::Cyan => termion_color::Cyan.write_fg(f),
            Color::Green => termion_color::Green.write_fg(f),
            Color::Magenta => termion_color::Magenta.write_fg(f),
            Color::Red => termion_color::Red.write_fg(f),
            Color::White => termion_color::White.write_fg(f),
            Color::Yellow => termion_color::Yellow.write_fg(f),
            Color::Rgb(r, g, b) => termion_color::Rgb(*r, *g, *b).write_fg(f),
            Color::Reset => termion_color::Reset.write_fg(f),
        }
    }
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::LightBlack => termion_color::LightBlack.write_bg(f),
            Color::LightBlue => termion_color::LightBlue.write_bg(f),
            Color::LightCyan => termion_color::LightCyan.write_bg(f),
            Color::LightGreen => termion_color::LightGreen.write_bg(f),
            Color::LightMagenta => termion_color::LightMagenta.write_bg(f),
            Color::LightRed => termion_color::LightRed.write_bg(f),
            Color::LightWhite => termion_color::LightWhite.write_bg(f),
            Color::LightYellow => termion_color::LightYellow.write_bg(f),
            Color::Black => termion_color::Black.write_bg(f),
            Color::Blue => termion_color::Blue.write_bg(f),
            Color::Cyan => termion_color::Cyan.write_bg(f),
            Color::Green => termion_color::Green.write_bg(f),
            Color::Magenta => termion_color::Magenta.write_bg(f),
            Color::Red => termion_color::Red.write_bg(f),
            Color::White => termion_color::White.write_bg(f),
            Color::Yellow => termion_color::Yellow.write_bg(f),
            Color::Rgb(r, g, b) => termion_color::Rgb(*r, *g, *b).write_bg(f),
            Color::Reset => termion_color::Reset.write_bg(f),
        }
    }
}
