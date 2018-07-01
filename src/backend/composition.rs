use super::{Block, Line, Size, Span};
use {Bound, Content, Element};

fn compose_line(line: String, size: Size) -> Block {
    let line: Line = Span::from_str("".to_owned(), &line, size.cols).into();
    line.into()
}

fn compose_text(text: Vec<String>, size: Size, bound: Bound) -> Block {
    Block::from_text(text, size.cols, size.rows, bound)
}

fn compose_vbox(elements: Vec<Element>, size: Size) -> Block {
    let (fixed, greedy): (Vec<(usize, Element)>, Vec<(usize, Element)>) = elements
        .into_iter()
        .enumerate()
        .partition(|(_, e)| e.bounds.height == Bound::Fixed);
    let mut remaining_rows = size.rows;
    let cols = size.cols;
    let greedy_count = greedy.len();
    let mut blocks: Vec<(usize, Block)> = fixed
        .into_iter()
        .map(|(i, e)| {
            let b = compose_image(e, Size::new(cols, remaining_rows));
            remaining_rows -= b.height;
            (i, b)
        })
        .collect();
    blocks.extend(greedy.into_iter().map(|(i, e)| {
        let b = compose_image(e, Size::new(cols, remaining_rows / greedy_count));
        remaining_rows -= b.height;
        (i, b)
    }));
    blocks.sort_by_key(|a| a.0);
    let init = Block::new(vec![], cols, 0);
    blocks.into_iter().fold(init, |mut acc, (_, b)| {
        acc.vconcat(b);
        acc
    })
}

pub fn compose_image(ui: Element, size: Size) -> Block {
    let block = match ui.content {
        Content::Line(line) => compose_line(line, size),
        Content::Text(text) => compose_text(text, size, ui.bounds.height),
        Content::VBox(elements) => compose_vbox(elements, size),
    };
    block
}
