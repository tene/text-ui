use std::fmt;
use termion::color as termion_color;
use termion::color::Color as TermColor;
use unicode_segmentation::UnicodeSegmentation;

use Color;

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

fn termion_attr_fragment(fg: Option<Color>, bg: Option<Color>) -> String {
    match (fg, bg) {
        (Some(fg), Some(bg)) => format!("{}{}", termion_color::Fg(fg), termion_color::Bg(bg)),
        (Some(fg), None) => format!(
            "{}{}",
            termion_color::Fg(fg),
            termion_color::Bg(termion_color::Reset)
        ),
        (None, Some(bg)) => format!(
            "{}{}",
            termion_color::Fg(termion_color::Reset),
            termion_color::Bg(bg)
        ),
        (None, None) => format!(
            "{}{}",
            termion_color::Fg(termion_color::Reset),
            termion_color::Bg(termion_color::Reset)
        ),
    }
}

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
