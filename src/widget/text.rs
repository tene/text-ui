use ::backend::{Widget, Render};
use std::marker;

#[derive(Debug, Clone)]
pub struct Text<R: Render> {
    pub text: String,
    _marker: marker::PhantomData<R>,
}

impl<R: Render> Text<R> {
    pub fn from_string(s: String) -> Text<R> {
        Text { text: s, _marker: marker::PhantomData }
    }
}

impl<R: Render> Widget<R> for Text<R> {
        fn render(&self) -> R {
            R::str(&self.text)
        }
}