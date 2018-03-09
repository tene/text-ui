extern crate unicode_segmentation;

use self::unicode_segmentation::UnicodeSegmentation;

use ::backend::Builder;

use std::iter::repeat;

// This should be optimized for the single-character case
#[derive(Clone)]
struct TextCell {
    glyph: Option<String>,
}

impl TextCell {
    pub fn from_line(s: &str) -> Vec<TextCell> {
        UnicodeSegmentation::graphemes(s, true)
        .map(|g| TextCell::from_checked(g))
        .collect()
    }
    pub fn from_checked(s: &str) -> TextCell {
        TextCell {
            glyph: Some(s.to_owned())
        }
    }
    pub fn to_string(&self) -> String {
        match &self.glyph {
            &Some(ref s) => s.to_owned(),
            &None        => " ".to_owned(),
        }
    }
    pub fn empty() -> TextCell {
        TextCell {
            glyph: None,
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
    /*fn new_col(size: usize) -> TextGrid {
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
    }*/
    fn blank_row(size: usize) -> Vec<TextCell> {
        let mut row: Vec<TextCell> = Vec::with_capacity(size);
        row.resize(size, TextCell::empty());
        row
    }
    pub fn to_string(&self) -> String {
        (&self.grid).iter()
            .map(
                |row| row.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("")
            ).collect::<Vec<String>>().join("\n")
    }
    
    fn str(s: &str) -> TextGrid {
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
    fn happend(&mut self, other: &mut Self) {
        if self.rows < other.rows {
            self.grid.resize(other.rows, TextGrid::blank_row(self.cols));
            self.rows = other.rows;
        }
        for (r1, r2) in self.grid.iter_mut().zip(
                other.grid.iter_mut()
                .chain(
                    repeat(TextGrid::blank_row(other.cols))
                    .take(self.rows - other.rows)
                    .collect::<Vec<Vec<TextCell>>>()
                    .iter_mut()
                )
                .take(self.rows)
        ) {
            r1.append(r2)
        }
        self.cols += other.cols;
    }
    fn vappend(&mut self, other: &mut Self) {
        if self.cols < other.cols {
            for row in self.grid.iter_mut() {
                row.resize(other.cols, TextCell::empty());
            }
            self.cols = other.cols;
        }
        if self.cols > other.cols {
            for row in other.grid.iter_mut() {
                row.resize(self.cols, TextCell::empty());
            }
        }
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
    fn empty() -> TextGrid {
        TextGrid {
            rows: 0,
            cols: 0,
            grid: Vec::new(),
        }
    }
}

pub struct TextBuilder {}

impl TextBuilder {
    pub fn new() -> TextBuilder {
        TextBuilder {}
    }
}

impl Builder for TextBuilder {
    type Drawable = TextGrid;
    fn str(&mut self, s: &str) -> Self::Drawable {
        TextGrid::str(s)
    }
    fn happend(&mut self, a: &mut Self::Drawable, b: &mut Self::Drawable) {
        TextGrid::happend(a,b)
    }
    fn hconcat(&mut self, a: &Self::Drawable, b: &Self::Drawable) -> Self::Drawable {
        TextGrid::hconcat(a,b)
    }
    fn vappend(&mut self, a: &mut Self::Drawable, b: &mut Self::Drawable) {
        TextGrid::vappend(a,b)
    }
    fn vconcat(&mut self, a: &Self::Drawable, b: &Self::Drawable) -> Self::Drawable {
        TextGrid::vconcat(a,b)
    }
    fn empty(&mut self) -> Self::Drawable {
        TextGrid::empty()
    }
}