extern crate ggez;
extern crate text_ui;
use ggez::*;
use ggez::graphics::{DrawMode, Point2};
use text_ui::backend::ggez as backend;
use text_ui::backend::{Widget, GGState};
use text_ui::widget::Text;

struct MainState {
    widget: Box<Widget<GGState>>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { widget: Box::new(Text::from_string("Hello".to_owned())) };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let state = self.widget.render();
        let _ = backend::draw(ctx, Point2::new(0.0, 0.0), &state);
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}