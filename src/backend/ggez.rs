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
    width: f32,
    height: f32,
}

impl GGState {
    fn new(content: GGContent, width: f32, height: f32) -> GGState {
        GGState {
            content: content,
            width: width,
            height: height,
        }
    }
}

impl Render for GGState {
    fn str(s: &str) -> Self {
        let text = GGContent::Text(s.to_owned());
        Self::new(text, 10.0, 10.0)
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
        self.width += other.width;
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
        self.height += other.height;
    }
    fn vconcat(&self, other: &Self) -> Self {
        let mut new = self.clone();
        (&mut new).vappend(&mut other.clone());
        new
    }
    fn empty() -> Self {
        Self::new(GGContent::Empty, 0.0, 0.0)
    }
}

extern crate ggez;
use self::ggez::{Context, graphics};
use self::ggez::graphics::{Point2};
use self::ggez::error::GameResult;
pub fn draw(ctx: &mut Context, p: Point2, d: &GGState) -> GameResult<()> {
    match d.content {
        GGContent::Text(ref s) => {
            let fnt = graphics::Font::default_font().expect("Fonts?!?!");
            let txt = graphics::Text::new(ctx, &s, &fnt).expect("Fonts!?!?");
            graphics::draw(ctx, &txt, p, 0.0)
        },
        _ => unimplemented!(),
    }
}