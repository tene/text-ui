extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::iter::repeat;

#[derive(Clone)]
pub struct TextCell {
    glyph: Option<String>,
}

impl TextCell {
    fn from_line(s: &str) -> Vec<TextCell> {
        UnicodeSegmentation::graphemes(s, true)
        .map(|g| TextCell::from_checked(g))
        .collect()
    }
    fn from_checked(s: &str) -> TextCell {
        TextCell {
            glyph: Some(s.to_owned())
        }
    }
    fn to_string(&self) -> String {
        match &self.glyph {
            &Some(ref s) => s.to_owned(),
            &None        => "".to_owned(),
        }
    }
    fn empty() -> TextCell {
        TextCell {
            glyph: None,
        }
    }
}
impl<'a> TextCell {
    fn to_str(&'a self) -> &'a str {
        match &self.glyph {
            &Some(ref s) => s,
            _       => " ",
        }
    }
}

#[derive(Clone)]
pub struct TextGrid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<TextCell>>,
}

impl TextGrid {
    fn new_col(size: usize) -> TextGrid {
        let mut col: Vec<Vec<TextCell>> = Vec::with_capacity(size);
        col.resize(size, Vec::new());
        TextGrid {
            rows: size,
            cols: 0,
            grid: col,
        }
    }
    fn new_row(size: usize) -> TextGrid {
        TextGrid {
            rows: 0,
            cols: size,
            grid: Vec::new(),
        }
    }
    fn from_str(s: &str) -> TextGrid {
        let mut grid: Vec<Vec<TextCell>> = s.lines()
            .map(TextCell::from_line)
            .collect();
        let rows = grid.len();
        let cols = (&grid).iter().max_by_key(|l| l.len()).unwrap().len();
        for row in grid.iter_mut() {
            row.resize(cols, TextCell::empty());
        }
        TextGrid { rows: rows, cols: cols, grid: grid }
    }
    fn to_string(&self) -> String {
        (&self.grid).iter()
            .map(
                |row| row.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("")
            ).collect::<Vec<String>>().join("\n")
    }
    fn happend(&mut self, other: &mut Self) {
        assert_eq!(&self.rows, &other.rows);
        for (r1, r2) in self.grid.iter_mut().zip(other.grid.iter_mut()) {
            r1.append(r2)
        }
        self.cols += other.cols;
    }
    fn vappend(&mut self, other: &mut Self) {
        assert_eq!(&self.cols, &other.cols);
        self.grid.append(&mut other.grid);
        self.rows += other.rows;
    }
    fn hconcat(&self, other: &Self) -> TextGrid {
        let mut n = (*self).clone();
        (&mut n).happend(&mut other.clone());
        n
    }
    fn vconcat(&self, other: &Self) -> TextGrid {
        let mut n = (*self).clone();
        (&mut n).vappend(&mut other.clone());
        n
    }
}

#[derive(Clone)]
pub enum Content {
    Text(String),
    VBox(Vec<Pane>),
    HBox(Vec<Pane>),
    HLine,
    VLine,
}

#[derive(Clone)]
pub struct Pane {
    pub content: Content,
    pub rows: usize,
    pub cols: usize,
}

pub fn txt(s: String) -> Pane {
    Pane {
        rows: 1,
        cols: s.len(),
        content: Content::Text(s),
    }
}

pub fn paragraph(s: &str) -> Pane {
    let lines : Vec<Pane> = s.lines().map(|l| txt(l.to_owned())).collect();
    match lines.len() {
        0 => txt("".to_owned()),
        1 => lines.into_iter().next().unwrap(),
        _ => vbox(lines),
    }
}

pub fn vbox(mut panes: Vec<Pane>) -> Pane {
    let cols = (&panes).iter().max_by_key(|p| p.cols).unwrap().cols;
    let rows : usize = (&panes).iter().map(|p| p.rows).sum();
    for p in panes.iter_mut() {
        p.cols = cols;
    }
    Pane {
        rows: rows,
        cols: cols,
        content: Content::VBox(panes),
    }
}

pub fn hbox(mut panes: Vec<Pane>) -> Pane {
    let rows = (&panes).iter().max_by_key(|p| p.rows).unwrap().rows;
    let cols : usize = (&panes).iter().map(|p| p.cols).sum();
    for p in panes.iter_mut() {
        p.rows = rows;
    }
    Pane {
        cols: cols,
        rows: rows,
        content: Content::HBox(panes),
    }
}

pub fn render_string(pane: &Pane) -> String {
    render(pane).to_string()
}

pub fn render(pane: &Pane) -> TextGrid {
    match &pane.content {
        &Content::Text(ref s) => TextGrid::from_str(&format!("{:width$}", s, width=pane.cols)),
        &Content::VBox(ref lines) => lines
            .iter()
            .map(|p| render(&p))
            .chain(repeat(TextGrid::from_str(&format!("{:width$}", "", width=pane.cols))))
            .take(pane.rows)
            .fold(TextGrid::new_row(pane.cols),
                |a, i| a.vconcat(&i)
            ),
        &Content::HBox(ref cols) => cols
            .iter()
            .map(|p| render(&p))
            .fold(TextGrid::new_col(pane.rows),
                |a, i| a.hconcat(&i)
            ),
        _ => TextGrid::from_str("???"),
    }
}

#[test]
fn basics() {
    let s = txt("Hello World".to_owned());
    let s2 = vbox(vec!(s.clone(), s.clone()));
    assert_eq!(render_string(&s), "Hello World".to_owned());
    assert_eq!(render_string(&s2), "Hello World\nHello World".to_owned());
}