use indextree::IndexTree;
use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;
use termion::color as termion_color;
use termion::color::Color as TermColor;
use unicode_segmentation::UnicodeSegmentation;

use {Color, Fragment, InputCallback, Name, Pos, RenderBound, RenderElement, Size};

use super::TermionBackend;

// This is redundant because termion colors are not sized, and I didn't want to add a box everywhere
impl TermColor for &Color {
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

fn termion_attr_fragment(frag: &Fragment) -> String {
    match (&frag.fg, &frag.bg) {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub attr: String,
    pub text: String,
    pub width: usize,
}

impl Span {
    pub fn new(attr: String, text: String, width: usize) -> Self {
        Span { attr, text, width }
    }

    pub fn from_str_constrained(attr: String, text: &str, width: usize) -> Self {
        let text = UnicodeSegmentation::graphemes(text, true)
            .chain(repeat(" "))
            .take(width)
            .collect::<String>();
        Span { attr, text, width }
    }

    pub fn from_str_unconstrained(attr: String, text: &str) -> Self {
        let width = UnicodeSegmentation::graphemes(text, true).count();
        let text = text.to_owned();
        Span { attr, text, width }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub spans: Vec<Span>,
    pub width: usize,
}

impl Line {
    pub fn new(spans: Vec<Span>, width: usize) -> Self {
        Line { spans, width }
    }
    pub fn hconcat(&mut self, mut other: Self) {
        self.width += other.width;
        self.spans.append(&mut other.spans);
    }
    pub fn blank(width: usize) -> Self {
        Span::from_str_constrained("".to_owned(), " ", width).into()
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
    pub callbacks: IndexTree<N, InputCallback<TermionBackend<N>, N>>,
    pub cursors: HashMap<N, Pos>,
}

impl<N: Name> RenderElement<TermionBackend<N>, N> for Block<N> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
    fn add_input_handler(
        mut self,
        name: Option<N>,
        callback: InputCallback<TermionBackend<N>, N>,
    ) -> Self {
        self.callbacks.push(name, callback);
        self
    }
    fn add_cursor(mut self, name: N, pos: Pos) -> Self {
        self.cursors.insert(name, pos);
        self
    }
    fn get_cursor(&self, name: &N) -> Option<Pos> {
        self.cursors.get(name).cloned()
    }
    // Maybe factor out the common parts of these?
    fn vconcat(mut self, mut other: Self) -> Self {
        assert_eq!(self.width, other.width); // XXX TODO Maybe expand the smaller to fit?
        self.lines.append(&mut other.lines);
        // map callback position
        self.callbacks.append(&mut other.callbacks);
        let offset = Pos::new(0, self.height);
        self.cursors
            .extend(other.cursors.into_iter().map(move |(n, p)| (n, p + offset)));
        self.height += other.height;
        self
    }
    fn hconcat(mut self, mut other: Self) -> Self {
        assert_eq!(self.height, other.height); // XXX TODO Maybe expand the smaller to fit?
        for (mut a, b) in self.lines.iter_mut().zip(other.lines.into_iter()) {
            a.hconcat(b);
        }
        // map callback position
        self.callbacks.append(&mut other.callbacks);
        let offset = Pos::new(self.height, 0);
        self.cursors
            .extend(other.cursors.into_iter().map(move |(n, p)| (n, p + offset)));
        self.width += other.width;
        self
    }
}

impl<N: Name> Block<N> {
    pub fn new(lines: Vec<Line>, width: usize, height: usize) -> Self {
        assert_eq!(lines.len(), height);
        Block {
            callbacks: IndexTree::new(),
            cursors: HashMap::new(),
            lines,
            width,
            height,
        }
    }
    pub fn from_fragment(frag: &Fragment, bound: RenderBound) -> Self {
        let attr = termion_attr_fragment(frag);
        let width = bound
            .width
            .expect("Constructing text without width constraint");
        let lines = frag
            .text
            .lines()
            .flat_map(|l| split_line_graphemes(&l, width).into_iter())
            .map(|l| Span::new(attr.clone(), l, width).into());
        let lines: Vec<Line> = match bound.height {
            None => lines.collect(),
            Some(height) => lines
                .chain(repeat(Line::blank(width)))
                .take(height)
                .collect(),
        };
        let height = lines.len();
        Block::new(lines, width, height)
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

impl<N> fmt::Debug for Block<N>
where
    N: Name,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block {{ lines: {:?}, size: {:?}, cursors: {:?} }}",
            self.lines,
            self.size(),
            self.cursors
        )
    }
}
