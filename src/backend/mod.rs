pub mod text;
pub mod ggez;
pub use self::text::TextGrid;
pub use self::ggez::GGState;

pub trait Render: Sized {
    fn render(&self, w: &Widget<Self>) -> Self {
        w.render()
    }
    fn string(s: &String) -> Self {
        Self::str(s.as_str())
    }
    fn str(&str) -> Self;
    fn happend(&mut self, &mut Self);
    fn hconcat(&self, &Self) -> Self;
    fn vappend(&mut self, &mut Self);
    fn vconcat(&self, &Self) -> Self;
    fn empty() -> Self;
}

pub trait Widget<R: Render> {
    fn render(&self) -> R;
}