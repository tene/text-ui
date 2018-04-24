pub mod text;
pub mod input;

pub use self::text::Text;
pub use self::input::Input;

use ::{Size, Position};
use ::pane::Pane;

pub trait Widget {
    fn render(&self, pos: Position, size: Size) -> Pane {
        let content = self.render_content(size);
        let focus = self.render_focus(size);
        let children = self.render_children(size);
        Pane {
            position: pos,
            content: content,
            focus: focus,
            children: children,
        }
    }
    fn render_content(&self, Size) -> Option<Vec<String>> { None }
    fn render_focus(&self, Size) -> Option<Position> { None }
    fn render_children(&self, Size) -> Option<Vec<Pane>> { None }
}