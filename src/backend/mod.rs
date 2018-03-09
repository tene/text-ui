pub mod text;
pub mod ggez;
pub use self::text::{TextGrid, TextBuilder};
pub use self::ggez::{GGState, GGBuilder};

pub trait Builder: Sized {
    type Drawable;
    fn build_from(&mut self, w: &Widget<Self>) -> Self::Drawable {
        w.build_with(self)
    }
    fn string(&mut self, s: &String) -> Self::Drawable {
        self.str(s.as_str())
    }
    fn str(&mut self, &str) -> Self::Drawable;
    fn happend(&mut self, &mut Self::Drawable, &mut Self::Drawable);
    fn hconcat(&mut self, &Self::Drawable, &Self::Drawable) -> Self::Drawable;
    fn vappend(&mut self, &mut Self::Drawable, &mut Self::Drawable);
    fn vconcat(&mut self, &Self::Drawable, &Self::Drawable) -> Self::Drawable;
    fn empty(&mut self) -> Self::Drawable;
}

pub trait Widget<B: Builder> {
    fn build_with(&self, &mut B) -> B::Drawable;
}