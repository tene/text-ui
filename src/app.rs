use widget::Widget;
use Size;

pub trait App<W> {
    fn widget(&self) -> W;
    fn size(&self) -> Size;
}