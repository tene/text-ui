#[derive(Debug, PartialEq)]
pub enum Bound {
    Fixed(usize),
    //AtLeast(usize),
    //AtMost(usize),
    //Range(usize, usize),
    Free,
}

impl Default for Bound {
    fn default() -> Self {
        Bound::Free
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Size {
    pub width: Bound,
    pub height: Bound,
}

impl Size {
    pub fn rows(n: usize) -> Size {
        let width = Bound::Free;
        let height = Bound::Fixed(n);
        Size { width, height }
    }
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Line(String),
    Text(Vec<String>),
    VBox(Vec<Element>),
    //HBox(Vec<Element>),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    //pub widget: Option<String>,
    //pub name: Option<String>,
    // borders?
    pub size: Size,
    pub content: Content,
}

impl Element {
    pub fn line(line: &str) -> Element {
        let size = Size::rows(1);
        let content = Content::Line(line.to_owned());
        Element { size, content }
    }
    pub fn text(text: Vec<String>) -> Element {
        let size = Size::default();
        let content = Content::Text(text);
        Element { size, content }
    }
    pub fn vbox(elems: Vec<Element>) -> Element {
        let size = Size::default();
        let content = Content::VBox(elems);
        Element { size, content }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::{Content, Element};
        let ex1 = Element::line("lol");
        assert_eq!(ex1.content, Content::Line("lol".to_owned()));
    }
}
