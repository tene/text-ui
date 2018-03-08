use ::backend::Render;

#[derive(Clone)]
enum GGContent {
    VBox(Vec<Box<GGState>>),
    HBox(Vec<Box<GGState>>),
    Text(String),
    Empty,
}

#[derive(Clone)]
pub struct GGState {
    content: GGContent,
}

impl Render for GGState {
    fn str(s: &str) -> Self {
        GGState { content: GGContent::Text(s.to_owned())}
    }
    fn happend(&mut self, other: &mut Self) {
        let mut newbox = false;
        match &mut self.content {
            &mut GGContent::HBox(ref mut v) => v.push(Box::new(other.clone())),
            _ => newbox = true,
        };
        if newbox {
            let x = self.clone();
            let mut list = Vec::new();
            list.push(Box::new(x));
            list.push(Box::new(other.clone()));
            self.content = GGContent::HBox(list);
        }
    }
    fn hconcat(&self, other: &Self) -> Self {
        let mut new = self.clone();
        (&mut new).happend(&mut other.clone());
        new
    }
    fn vappend(&mut self, other: &mut Self) {
        let mut newbox = false;
        match &mut self.content {
            &mut GGContent::VBox(ref mut v) => v.push(Box::new(other.clone())),
            _ => newbox = true,
        };
        if newbox {
            let x = self.clone();
            let mut list = Vec::new();
            list.push(Box::new(x));
            list.push(Box::new(other.clone()));
            self.content = GGContent::VBox(list);
        }
    }
    fn vconcat(&self, other: &Self) -> Self {
        let mut new = self.clone();
        (&mut new).vappend(&mut other.clone());
        new
    }
    fn empty() -> Self {
        GGState { content: GGContent::Empty }
    }
}

extern crate ggez;
use self::ggez::{Context, graphics};
pub fn draw(ctx: &mut Context, d: &GGState) {
    match d.content {
        GGContent::Text(ref s) => {
            let fnt = graphics::Font::default_font().expect("Fonts?!?!");
            let txt = graphics::Text::new(ctx, &s, &fnt).expect("Fonts!?!?");
            graphics::draw(ctx, &txt, graphics::Point2::new(0.0, 0.0), 0.0);
        },
        _ => unimplemented!(),
    }
}