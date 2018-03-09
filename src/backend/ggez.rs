extern crate ggez;
use self::ggez::{Context, graphics};
use ::backend::Builder;
use std::cmp::max;

#[derive(Clone)]
enum GGContent {
    VBox(Vec<GGState>),
    HBox(Vec<GGState>),
    Text(graphics::Text),
    Empty,
}

#[derive(Clone)]
pub struct GGState {
    content: GGContent,
    width: u32,
    height: u32,
}

impl GGState {
    fn new(content: GGContent, width: u32, height: u32) -> GGState {
        GGState {
            content: content,
            width: width,
            height: height,
        }
    }
    pub fn text(text: graphics::Text) -> Self {
        let width = text.width();
        let height = text.height();
        let cnt = GGContent::Text(text);
        Self::new(cnt, width, height)
    }
}

pub struct GGBuilder {
//    ctx: Rc<Context>,
}

impl GGBuilder {
    pub fn new() -> GGBuilder { GGBuilder {} }
/*    pub fn new(ctx: Rc<Context>)
    -> GGBuilder {
        GGBuilder {
            ctx: ctx,
        }
    }*/
}

impl Builder for GGBuilder {
    type Drawable = GGState;
    // XXX I can't figure out how to thread &mut Context through the Builder
    fn str(&mut self, _s: &str) -> Self::Drawable {
        /*let font = self.ctx.default_font.clone();
        let text = graphics::Text::new(Rc::get_mut(&mut self.ctx).unwrap(), s, &font)
            .expect("Failed to render text");
        let width = text.width();
        let height = text.height();
        let cnt = GGContent::Text(text);
        Self::Drawable::new(cnt, width, height)*/
        unimplemented!()
    }
    fn happend(&mut self, a: &mut Self::Drawable, b: &mut Self::Drawable) {
        let mut newbox = false;
        match &mut a.content {
            &mut GGContent::HBox(ref mut v) => v.push(b.clone()),
            _ => newbox = true,
        };
        if newbox {
            let x = a.clone();
            let mut list = Vec::new();
            list.push(x);
            list.push(b.clone());
            a.content = GGContent::HBox(list);
        }
        a.width += b.width;
        a.height = max(a.height, b.height);
    }
    fn hconcat(&mut self, a: &Self::Drawable, b: &Self::Drawable) -> Self::Drawable {
        let mut new = a.clone();
        self.happend(&mut new, &mut b.clone());
        new
    }
    fn vappend(&mut self, a: &mut Self::Drawable, b: &mut Self::Drawable) {
        let mut newbox = false;
        match &mut a.content {
            &mut GGContent::VBox(ref mut v) => v.push(b.clone()),
            _ => newbox = true,
        };
        if newbox {
            let x = a.clone();
            let mut list = Vec::new();
            list.push(x);
            list.push(b.clone());
            a.content = GGContent::VBox(list);
        }
        a.height += b.height;
        a.width = max(a.width, b.width);
    }
    fn vconcat(&mut self, a: &Self::Drawable, b: &Self::Drawable) -> Self::Drawable {
        let mut new = a.clone();
        self.vappend(&mut new, &mut b.clone());
        new
    }
    fn empty(&mut self) -> Self::Drawable {
        Self::Drawable::new(GGContent::Empty, 0, 0)
    }
}

use self::ggez::graphics::{Point2};
use self::ggez::error::GameResult;
pub fn draw(ctx: &mut Context, p: Point2, d: &GGState) -> GameResult<()> {
    match d.content {
        GGContent::Text(ref s) => {
            graphics::draw(ctx, s, p, 0.0)
        },
        GGContent::VBox(ref lst) => {
            let mut offset: f32 = 0.0;
            for i in lst.iter() {
                let _ = draw(ctx, Point2::new(p.x, p.y + offset), i);
                offset += i.height as f32;
            }
            Ok(())
        },
        GGContent::HBox(ref lst) => {
            let mut offset: f32 = 0.0;
            for i in lst.iter() {
                let _ = draw(ctx, Point2::new(p.x + offset, p.y), i);
                offset += i.width as f32;
            }
            Ok(())
        },
        _ => unimplemented!(),
    }
}