#[derive(Debug, PartialEq)]
pub enum Bound {
    Fixed,
    Greedy,
}

impl Default for Bound {
    fn default() -> Self {
        Bound::Greedy
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Bounds {
    pub width: Bound,
    pub height: Bound,
}

impl Bounds {
    pub fn fixed_height() -> Bounds {
        let width = Bound::Greedy;
        let height = Bound::Fixed;
        Bounds { width, height }
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
    pub bounds: Bounds,
    pub content: Content,
}

impl Element {
    pub fn line(line: &str) -> Element {
        let bounds = Bounds::fixed_height();
        let content = Content::Line(line.to_owned());
        Element { bounds, content }
    }
    pub fn text(text: Vec<String>) -> Element {
        let bounds = Bounds::default();
        let content = Content::Text(text);
        Element { bounds, content }
    }
    pub fn vbox(elems: Vec<Element>) -> Element {
        let bounds = Bounds::default();
        let content = Content::VBox(elems);
        Element { bounds, content }
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
