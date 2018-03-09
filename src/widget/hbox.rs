use ::backend::{Widget, Builder};

pub struct HBox<B: Builder> {
    pub contents: Vec<Box<Widget<B>>>,
}

impl<B: Builder> HBox<B> {
    pub fn from_vec(panes: Vec<Box<Widget<B>>>) -> Self {
        HBox { contents: panes }
    }
    pub fn from_pair(a: Box<Widget<B>>, b: Box<Widget<B>>) -> Self {
        let mut panes: Vec<Box<Widget<B>>> = Vec::new();
        panes.push(a);
        panes.push(b);
        HBox {
            contents: panes,
        }
    }
}


impl<B: Builder> Widget<B> for HBox<B> {
        fn build_with(&self, b: &mut B) -> B::Drawable {
            let mut items: Vec<B::Drawable> = self
            .contents.iter()
            .map(|w| w.build_with(b))
            .collect();
            let rv = match items.len() {
                0 => b.empty(),
                1 => items.remove(0),
                _ => {
                    let first = items.remove(0);
                    items.into_iter()
                    .fold(first, |a, i| b.hconcat(&a, &i))
                }
            };
            rv
        }
}