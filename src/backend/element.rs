use indextree::IndexTree;
use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;
use termion::color as termion_color;
use termion::color::Color as TermColor;
use unicode_segmentation::UnicodeSegmentation;

use {
    Color, Key, KeyCallback, MouseCallback, MouseEvent, Name, Pos, RenderBound, RenderElement,
    Size, TextBlock,
};

use super::{TermionBackend, TermionEventContext};

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
    key_callbacks: IndexTree<N, KeyCallback<TermionBackend<N>, N>>,
    cursors: HashMap<N, Pos>,
    hit_map: Vec<Vec<Option<usize>>>,
    mouse_callbacks: IndexTree<usize, (Option<N>, Pos, MouseCallback<TermionBackend<N>, N>)>,
}

impl<N: Name> RenderElement<TermionBackend<N>, N> for Block<N> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
    fn add_key_handler(
        mut self,
        name: Option<N>,
        callback: KeyCallback<TermionBackend<N>, N>,
    ) -> Self {
        self.key_callbacks.push(name, callback);
        self
    }
    fn add_mouse_handler(
        mut self,
        name: Option<N>,
        callback: MouseCallback<TermionBackend<N>, N>,
    ) -> Self {
        let idx = self
            .mouse_callbacks
            .push(None, (name, Pos::new(0, 0), callback));
        for row in &mut self.hit_map {
            for cell in row {
                if cell.is_none() {
                    *cell = Some(idx);
                }
            }
        }
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
        let pos_offset = Pos::new(0, self.height);
        self.lines.append(&mut other.lines);

        self.key_callbacks.append(&mut other.key_callbacks);

        let mut offset_mouse_callbacks = other
            .mouse_callbacks
            .map(|(name, pos, cb)| (name, pos + pos_offset, cb));
        let idx_offset = self.mouse_callbacks.append(&mut offset_mouse_callbacks);
        for row in &mut other.hit_map {
            for cell in row {
                *cell = cell.map(|i| i + idx_offset);
            }
        }
        self.hit_map.append(&mut other.hit_map);

        self.cursors.extend(
            other
                .cursors
                .into_iter()
                .map(move |(n, p)| (n, p + pos_offset)),
        );
        self.height += other.height;
        self
    }
    fn hconcat(mut self, mut other: Self) -> Self {
        assert_eq!(self.height, other.height); // XXX TODO Maybe expand the smaller to fit?
        let pos_offset = Pos::new(self.height, 0);
        for (mut a, b) in self.lines.iter_mut().zip(other.lines.into_iter()) {
            a.hconcat(b);
        }
        self.key_callbacks.append(&mut other.key_callbacks);

        let mut offset_mouse_callbacks = other
            .mouse_callbacks
            .map(|(name, pos, cb)| (name, pos + pos_offset, cb));
        let idx_offset = self.mouse_callbacks.append(&mut offset_mouse_callbacks);
        for row in &mut other.hit_map {
            for cell in row {
                *cell = cell.map(|i| i + idx_offset);
            }
        }

        for (mut a, b) in self.hit_map.iter_mut().zip(other.hit_map.into_iter()) {
            a.extend_from_slice(&b)
        }

        self.cursors.extend(
            other
                .cursors
                .into_iter()
                .map(move |(n, p)| (n, p + pos_offset)),
        );
        self.width += other.width;
        self
    }
}

impl<N: Name> Block<N> {
    pub fn new(lines: Vec<Line>, width: usize, height: usize) -> Self {
        assert_eq!(lines.len(), height);
        let mut empty_line = Vec::new();
        // https://github.com/rust-lang/rust/issues/41758
        empty_line.resize(width, None);
        let mut hit_map = Vec::new();
        hit_map.resize(height, empty_line);
        Block {
            key_callbacks: IndexTree::new(),
            cursors: HashMap::new(),
            hit_map,
            mouse_callbacks: IndexTree::new(),
            lines,
            width,
            height,
        }
    }
    /*pub fn from_textline(text: Text<N>, bound: RenderBound) -> Self {
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
    }*/
    pub fn handle_key(&self, event_ctx: &TermionEventContext<N>, focus: &N, key: Key) {
        use ShouldPropagate::*;
        for cb in self.key_callbacks.get_iter(focus) {
            match cb(event_ctx, key) {
                Stop => break,
                Continue => continue,
            }
        }
    }
    // XXX TODO Need to use internal mouse event type instead of termion's, with relative coords
    pub fn handle_mouse(&self, event_ctx: &TermionEventContext<N>, mevent: MouseEvent) {
        use ShouldPropagate::*;
        let (x, y) = match mevent {
            MouseEvent::Press(_, x, y) => (x as usize - 1, y as usize - 1),
            MouseEvent::Release(x, y) => (x as usize - 1, y as usize - 1),
            MouseEvent::Hold(x, y) => (x as usize - 1, y as usize - 1),
        };
        if let Some(idx) = self.hit_map[y][x] {
            let frame_pos = Pos::new(x, y);
            for (_name, pos, cb) in self.mouse_callbacks.get_iter_idx(idx) {
                match cb(event_ctx, frame_pos - *pos, mevent) {
                    Stop => break,
                    Continue => continue,
                }
            }
        }
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
