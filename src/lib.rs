pub mod backend;
pub mod widget;
use backend::{Render, Widget};
//use widget::hbox;

use std::iter::repeat;
use backend::text::TextGrid;

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
        &Content::Text(ref s) => TextGrid::str(&format!("{:width$}", s, width=pane.cols)),
        &Content::VBox(ref lines) => lines
            .iter()
            .map(|p| render(&p))
            .chain(repeat(TextGrid::str(&format!("{:width$}", "", width=pane.cols))))
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
        _ => TextGrid::str("???"),
    }
}

#[test]
fn basics() {
    let s = txt("Hello World".to_owned());
    let s2 = vbox(vec!(s.clone(), s.clone()));
    assert_eq!(render_string(&s), "Hello World".to_owned());
    assert_eq!(render_string(&s2), "Hello World\nHello World".to_owned());
}