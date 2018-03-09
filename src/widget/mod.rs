pub mod text;
pub use self::text::*;
pub mod vbox;
pub use self::vbox::*;
pub mod hbox;
pub use self::hbox::*;

use ::backend::{Builder, Widget};

pub fn demo_widget<B: Builder + 'static>() -> HBox<B> {
    let g1 = Text::from_string("Hi".to_owned());
    let g2 = Text::from_string("Hello".to_owned());
    let v1 = VBox::from_pair(Box::new(g1), Box::new(g2));
    let n1 = Text::from_string("Eve".to_owned());
    let n2 = Text::from_string("Chel".to_owned());
    let n3 = Text::from_string("Susan".to_owned());
    let mut v2 = VBox::from_pair(Box::new(n1), Box::new(n2));
    (&mut v2).append(Box::new(n3));
    let b  = HBox::from_pair(Box::new(v1), Box::new(v2));
    b
}

pub struct Wrap<B: Builder> where B::Drawable: Clone {
    inner: B::Drawable,
}

impl<B: Builder> Wrap<B> where B::Drawable: Clone {
    pub fn new(i: B::Drawable) -> Self {
        Wrap { inner: i }
    }
}

impl<B: Builder> Widget<B> for Wrap<B> where B::Drawable: Clone {
        fn build_with(&self, _b: &mut B) -> B::Drawable {
            self.inner.clone()
        }
}