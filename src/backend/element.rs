use indextree::IndexTree;
use std::collections::HashMap;
use std::iter::repeat;
use unicode_segmentation::UnicodeSegmentation;

use {GrowthPolicy, InputCallback, Name, Pos, RenderElement};

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

    pub fn from_str(attr: String, text: &str, width: usize) -> Self {
        let text = UnicodeSegmentation::graphemes(text, true)
            .chain(repeat(" "))
            .take(width)
            .collect::<String>();
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
}

impl<N: Name> Block<N> {
    pub fn new(lines: Vec<Line>, width: usize, height: usize) -> Self {
        Block {
            callbacks: IndexTree::new(),
            cursors: HashMap::new(),
            lines,
            width,
            height,
        }
    }
    pub fn line(text: &str, width: usize) -> Self {
        let line: Line = Span::from_str("".to_owned(), text, width).into();
        Block::new(vec![line], width, 1)
    }
    pub fn from_text(
        text: Vec<String>,
        width: usize,
        height: usize,
        should_grow: GrowthPolicy,
    ) -> Self {
        let lines = text
            .into_iter()
            .flat_map(|l| split_line_graphemes(&l, width).into_iter())
            .map(|l| Span::new("".to_owned(), l, width).into());
        let lines: Vec<Line> = match should_grow {
            GrowthPolicy::FixedSize => lines.take(height).collect(),
            GrowthPolicy::Greedy => lines
                .chain(repeat(Span::from_str("".to_owned(), "", width).into()))
                .take(height)
                .collect(),
        };
        let height = lines.len();
        Block::new(lines, width, height)
    }
    pub fn vconcat(&mut self, mut other: Self) {
        assert_eq!(self.width, other.width);
        self.lines.append(&mut other.lines);
        // map callback position
        self.callbacks.append(&mut other.callbacks);
        let offset = Pos::new(0, self.height);
        self.cursors
            .extend(other.cursors.into_iter().map(move |(n, p)| (n, p + offset)));
        self.height += other.height;
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
