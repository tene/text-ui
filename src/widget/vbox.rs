use ::backend::{Widget, Render};

pub struct VBox<R: Render> {
    pub contents: Vec<Box<Widget<R>>>,
}

impl<R: Render> VBox<R> {
    pub fn from_vec(panes: Vec<Box<Widget<R>>>) -> Self {
        VBox { contents: panes }
    }
    pub fn from_pair(a: Box<Widget<R>>, b: Box<Widget<R>>) -> Self {
        let mut panes: Vec<Box<Widget<R>>> = Vec::new();
        panes.push(a);
        panes.push(b);
        VBox {
            contents: panes,
        }
    }
    pub fn append(&mut self, w: Box<Widget<R>>) {
        self.contents.push(w);
    }
}

impl<R: Render> Widget<R> for VBox<R> {
    fn render(&self) -> R {
        let mut items: Vec<R> = self
        .contents.iter()
        .map(|w| w.render())
        .collect();
        let rv = match items.len() {
            0 => R::empty(),
            1 => items.remove(0),
            _ => {
                let first = items.remove(0);
                items.into_iter()
                .fold(first, |a, i| R::vconcat(&a, &i))
            }
        };
        rv
    }
}