use widget::Widget;
use Size;

pub trait App {
    type UI: Widget;
    fn widget(&self) -> Self::UI;
    fn size(&self) -> Size;
}