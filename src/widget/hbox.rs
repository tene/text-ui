use ::backend::{Widget, Render};

pub struct HBox<R: Render> {
    pub contents: Vec<Box<Widget<R>>>,
}

impl<R: Render> HBox<R> {
    pub fn from_vec(panes: Vec<Box<Widget<R>>>) -> Self {
        HBox { contents: panes }
    }
    pub fn from_pair(a: Box<Widget<R>>, b: Box<Widget<R>>) -> Self {
        let mut panes: Vec<Box<Widget<R>>> = Vec::new();
        panes.push(a);
        panes.push(b);
        HBox {
            contents: panes,
        }
    }
}


impl<R: Render> Widget<R> for HBox<R> {
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
                    .fold(first, |a, i| R::hconcat(&a, &i))
                }
            };
            rv
        }
}