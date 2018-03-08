pub mod text;
pub use self::text::TextGrid;

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