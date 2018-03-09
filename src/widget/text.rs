use ::backend::{Widget, Builder};
use std::marker;

#[derive(Debug, Clone)]
pub struct Text<B: Builder> {
    pub text: String,
    _marker: marker::PhantomData<B>,
}

impl<B: Builder> Text<B> {
    pub fn from_string(s: String) -> Text<B> {
        Text { text: s, _marker: marker::PhantomData }
    }
}

impl<B: Builder> Widget<B> for Text<B> {
        fn build_with(&self, b: &mut B) -> B::Drawable {
            b.str(&self.text)
        }
}