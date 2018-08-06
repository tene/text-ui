use indextree::IndexTree;
use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;
use unicode_segmentation::UnicodeSegmentation;

use {InputCallback, Name, Pos, RenderBound, RenderElement, Size};

use super::TermionBackend;

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    pub fn line(text: &str, bound: RenderBound) -> Self {
        let line: Line = match bound.width {
            Some(width) => Span::from_str_constrained("".to_owned(), text, width).into(),
            None => Span::from_str_unconstrained("".to_owned(), text).into(),
        };
        let width = line.width;
        match bound.height {
            None => Block::new(vec![line], width, 1),
            Some(height) => {
                let blank = Line::blank(width);
                let lines: Vec<Line> = Some(line)
                    .into_iter()
                    .chain(repeat(blank))
                    .take(height)
                    .collect();
                Block::new(lines, width, height)
            }
        }
    }
    pub fn from_text(text: Vec<String>, bound: RenderBound) -> Self {
        let width = bound
            .width
            .expect("Constructing text without width constraint");
        let lines = text
            .into_iter()
            .flat_map(|l| split_line_graphemes(&l, width).into_iter())
            .map(|l| Span::new("".to_owned(), l, width).into());
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
