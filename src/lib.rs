#[derive(Debug, Copy, Clone)]
pub struct Context {
    pub rows: u32,
    pub cols: u32,
}

// XXX Padding?  Margins?  Borders?
pub struct TextArea {
    pub rows: u32,
    pub cols: u32,
    pub content: Vec<String>,
}

// This API is definitely at the wrong layer
// Might maybe be okay if refactored into a hierarchy?
// But then it's just a copy of widget?
// Where do we fit scrolling?
impl TextArea {
    pub fn new() -> TextArea {
        TextArea {
            rows: 0,
            cols: 0,
            content: Vec::new(),
        }
    }

    // XXX These should probably return new TAs instead of mutating?
    pub fn vconcat(&mut self, mut other: TextArea) {
        self.rows += other.rows;
        self.cols = std::cmp::max(self.cols, other.cols);
        self.content.append(&mut other.content);
    }
    pub fn hconcat(&mut self, other: TextArea) {
        self.content = self.content
            .clone()
            .into_iter()
            .zip(other.content.into_iter())
            .map(|(a,b)| a + &b)
            .collect();
        self.cols += other.cols;
        self.rows = std::cmp::max(self.rows, other.rows);
    }
}

pub trait Widget {
    fn render(&self, ctx: Context) -> TextArea;
}

pub struct Text {
    pub content: Vec<String>,
}

impl Widget for Text {
    fn render(&self, ctx: Context) -> TextArea {
        TextArea {
            rows: ctx.rows,
            cols: ctx.cols,
            content: self.content.clone(),
        }
    }
}

// XXX Should this be its own widget, or a property on something??
pub struct Bordered {
    pub wrapped: Box<Widget>,
}

impl Widget for Bordered {
    fn render(&self, ctx: Context) -> TextArea {
        unimplemented!()
    }
}

pub struct VBox {
    pub inner: Vec<Box<Widget>>,
}

impl Widget for VBox {
    fn render(&self, ctx: Context) -> TextArea {
        self.inner.iter().fold(TextArea::new(), |mut ta, i| {
            ta.vconcat(i.render(ctx.clone()));
            ta
        })
    }
}

pub struct HBox {
    pub inner: Vec<Box<Widget>>,
}

impl Widget for HBox {
    fn render(&self, ctx: Context) -> TextArea {
        self.inner.iter().fold(TextArea::new(), |mut ta, i| {
            ta.hconcat(i.render(ctx.clone()));
            ta
        })
    }
}